/* rava 项目进展全景 · D3 + dagre 泳道 DAG
 * 数据：data.json（或单文件版的 window.RAVA_DATA）——更新进展只改数据。 */
const STATUS = {
  done:    { c: "var(--c-done)",    label: "✅ 完成" },
  doing:   { c: "var(--c-doing)",   label: "🔄 进行中" },
  blocked: { c: "var(--c-blocked)", label: "⛔ 卡点" },
  risk:    { c: "var(--c-risk)",    label: "🟠 风险" },
  waiting: { c: "var(--c-waiting)", label: "🟣 等待触发" },
  todo:    { c: "var(--c-todo)",    label: "⚪ 排队" },
};
const BADGE = { blocker: "⛔", bolt: "⚡", risk: "⚠️" };
const W = 176, H = 62, PADX = 26, PADY = 16;

let DATA, nodesById, adjacency, curFilter = "all", criticalOn = false, ganttOn = false, selected = null;

async function loadData() {
  // 兼容两种数据约定：window.RAVA_DATA（本目录 data.js）或全局 const DASHBOARD_DATA（维护方生成器）
  DATA = (typeof window !== "undefined" && window.RAVA_DATA)
      || (typeof DASHBOARD_DATA !== "undefined" ? DASHBOARD_DATA : null);
  if (!DATA) { try {
      const r = await fetch("data.json"); if (!r.ok) throw new Error("HTTP " + r.status);
      DATA = await r.json();
    } catch (e) {
      document.body.insertAdjacentHTML("afterbegin", '<div style="padding:10px 20px;color:#f0978f">数据加载失败：data.js 的变量名须为 window.RAVA_DATA 或全局 DASHBOARD_DATA（当前两者都未定义）；HTTP 服务下也可提供 data.json。</div>'); throw e; } }
}
const colorOf = s => ({ done:"#2e9e5b", doing:"#2f7fd3", blocked:"#d64541", risk:"#e08a1e", waiting:"#8e5bd3", todo:"#9aa3ab" }[s] || "#9aa3ab");

/* ---------- 统计条 ---------- */
function renderStats() {
  const n = DATA.nodes.filter(x => x.type !== "context");
  const cnt = k => n.filter(x => x.status === k).length;
  const el = document.getElementById("stats");
  el.innerHTML = `<span>✅ <b>${cnt("done")}</b> 已完成</span><span>🔄 <b>${cnt("doing")}</b> 进行中</span>` +
    `<span>🔴 <b>${cnt("blocked")}</b> 卡点</span><span>🟣 <b>${cnt("waiting")}</b> 等待</span><span>⚪ <b>${cnt("todo")}</b> 排队</span>` +
    `<span>总进度 <b>${Math.round(cnt("done") / n.length * 100)}%</b></span>`;
}

/* ---------- 管线健康条 ---------- */
function renderPipe() {
  document.getElementById("pipe").innerHTML = DATA.pipeline.map(p =>
    `<div class="stage h-${p.health}"><div class="nm">${p.name}</div>${p.note ? `<div class="nt">${p.note}</div>` : ""}</div>`).join("");
}

/* ---------- dagre 布局（LR 流向 + 泳道纵带） ---------- */
function layout() {
  const g = new dagre.graphlib.Graph();
  g.setGraph({ rankdir: "LR", nodesep: 22, ranksep: 90, edgesep: 10 });
  g.setDefaultEdgeLabel(() => ({}));
  DATA.nodes.forEach(n => g.setNode(n.id, { width: W, height: H }));
  DATA.edges.forEach(e => g.setEdge(e.from, e.to));
  dagre.layout(g);
  // 纵向吸附到泳道带：同泳道内按 dagre 的 y 排序铺开
  const laneOrder = DATA.lanes.map(l => l.id);
  const laneGroups = {};
  DATA.nodes.forEach(n => {
    const li = laneOrder.indexOf(n.lane);
    (laneGroups[li] = laneGroups[li] || []).push({ id: n.id, y: g.node(n.id).y });
  });
  let laneTops = [], yCursor = 0;
  laneOrder.forEach((_, li) => {
    const items = (laneGroups[li] || []).sort((a, b) => a.y - b.y);
    const h = Math.max(items.length * (H + PADY), 120);
    laneTops[li] = yCursor;
    items.forEach((it, i) => { const nd = g.node(it.id); nd.y = yCursor + 30 + i * (H + PADY); });
    yCursor += h + 40;
  });
  const xs = DATA.nodes.map(n => g.node(n.id).x);
  const w = Math.max(...xs) + W + 140, h = yCursor + 60;
  DATA.nodes.forEach(n => { const nd = g.node(n.id); n.x = nd.x; n.y = nd.y; });
  return { g, w, h, laneTops };
}

/* ---------- 主图渲染 ---------- */
function renderGraph() {
  const { g, w, h, laneTops } = layout();
  const svg = d3.select("#graph"), laneOrder = DATA.lanes.map(l => l.id);
  svg.selectAll("*").remove();
  svg.attr("viewBox", `0 0 ${w} ${h}`);
  const root = svg.append("g");
  // 泳道底带
  DATA.lanes.forEach((l, i) => {
    root.append("rect").attr("class", "laneband").attr("x", 0).attr("y", laneTops[i])
      .attr("width", w).attr("height", (laneTops[i + 1] ?? h - 40) - laneTops[i] - 16).attr("rx", 10);
    root.append("text").attr("class", "lanelabel").attr("x", 14).attr("y", laneTops[i] + 22).text(l.name);
  });
  // 边
  const edgeG = root.append("g");
  DATA.edges.forEach(e => {
    const pts = g.edge(e.from, e.to).points;
    if (!pts) return;
    const src = nodesById[e.from], cls = ["edge", e.critical ? "critical" : "",
      src.status === "done" ? "e-done" : (e.to === "k-fix" ? "e-doing" : ((nodesById[e.to] || {}).badge === "bolt" ? "e-blocked" : "e-doing"))].join(" ");
    const p = edgeG.append("path").attr("class", cls).attr("data-from", e.from).attr("data-to", e.to)
      .attr("d", d3.line().x(d => d.x).y(d => d.y).curve(d3.curveBumpY)(pts));
    if (e.label) {
      const mid = pts[Math.floor(pts.length / 2)];
      edgeG.append("text").attr("class", cls).attr("data-from", e.from).attr("data-to", e.to).attr("x", mid.x).attr("y", mid.y - 6).attr("text-anchor", "middle").text(e.label);
    }
    p.on("click", () => selectNode(nodesById[e.to]));
  });
  // 节点
  const ng = root.append("g");
  DATA.nodes.forEach(n => {
    const grp = ng.append("g").attr("class", `node ${n.type || ""} ${n.critical ? "crit" : ""}`).attr("transform", `translate(${n.x},${n.y})`)
      .attr("data-id", n.id);
    grp.append("rect").attr("class", "box").attr("width", W).attr("height", H).attr("rx", n.type === "milestone" ? 20 : 8);
    grp.append("rect").attr("width", 5).attr("height", H).attr("rx", 2).attr("fill", colorOf(n.status)); // 左状态条
    grp.append("text").attr("class", "title").attr("x", 12).attr("y", 20)
      .text(n.name.length > 17 ? n.name.slice(0, 16) + "…" : n.name);
    if (n.badge) grp.append("text").attr("class", "badge").attr("x", W - 20).attr("y", 20).text(BADGE[n.badge] || "•");
    // 分段子任务条（多色）
    if (n.sub && n.sub.length) {
      const bw = W - 24, x0 = 12, y0 = H - 18;
      let x = x0;
      n.sub.forEach(s => { const seg = bw / n.sub.length;
        grp.append("rect").attr("x", x).attr("y", y0).attr("width", seg - 2).attr("height", 7).attr("rx", 2).attr("fill", colorOf(s.status)); x += seg; });
    }
    if (n.waitOn) grp.append("text").attr("class", "wait").attr("x", 12).attr("y", H - 26).text("等：" + n.waitOn);
    else if (n.note) grp.append("text").attr("class", "wait").attr("x", 12).attr("y", H - 26)
      .text(n.note.length > 24 ? n.note.slice(0, 23) + "…" : n.note);
    grp.on("click", () => selectNode(n)).on("mouseenter", () => highlight(n.id)).on("mouseleave", clearHighlight);
  });
  // 图例
  const lg = d3.select("#graphWrap").append("div").attr("id", "legend");
  lg.html(Object.entries(STATUS).map(([k, v]) => `<span class="it"><span class="sw" style="background:${colorOf(k)}"></span>${v.label}</span>`).join("")
    + `<span class="it">━ 红 = 关键路径</span><span class="it">内嵌小条 = 子任务状态</span>`);
  applyFilter(); if (criticalOn) applyCritical();
  const zb = d3.zoom().scaleExtent([0.15, 4]).on("zoom", ev => root.attr("transform", ev.transform));
  zb(svg); window.__zoom = zb; window.__root = root;
  fitView();
}

/* ---------- 悬停链路高亮 ---------- */
function reach(id, dir) {
  const seen = new Set([id]), q = [id];
  while (q.length) { const c = q.shift(); (adjacency[dir][c] || []).forEach(t => { if (!seen.has(t)) { seen.add(t); q.push(t); } }); }
  return seen;
}
function highlight(id) {
  const keep = new Set([...reach(id, "down"), ...reach(id, "up")]);
  d3.selectAll(".node").classed("dim", function () { return !keep.has(this.getAttribute("data-id")); });
  d3.selectAll(".edge").classed("dim", function () { return !(keep.has(this.getAttribute("data-from")) && keep.has(this.getAttribute("data-to"))); });
}
function clearHighlight() { d3.selectAll(".node").classed("dim", false); d3.selectAll(".edge").classed("dim", false); }

/* ---------- 详情侧栏 + 卡点影响 ---------- */
function downstreamOf(id) { return [...reach(id, "down")].filter(x => x !== id); }
function selectNode(n) {
  selected = n;
  const d = document.getElementById("detail");
  const rows = [`<h3>${n.name}</h3>`,
    `<div class="row"><span class="k">状态</span><span class="chip" style="background:${colorOf(n.status)}">${STATUS[n.status].label}</span>${n.badge ? ` ${BADGE[n.badge]}` : ""}</div>`,
    `<div class="row"><span class="k">所属线</span>${(DATA.lanes.find(l => l.id === n.lane) || {}).name || "-"}</div>`];
  if (n.waitOn) rows.push(`<div class="row"><span class="k">等待</span>${n.waitOn}</div>`);
  if (n.blockReason) rows.push(`<div class="row"><span class="k">受阻因</span>${n.blockReason}</div>`);
  if (n.note) rows.push(`<div class="row"><span class="k">说明</span>${n.note}</div>`);
  if (n.ev) rows.push(`<div class="row"><span class="k">证据</span><code>${n.ev}</code></div>`);
  if (n.sub) rows.push(`<div class="row"><span class="k">子任务</span></div><ul>${n.sub.map(s => `<li><span class="sw" style="display:inline-block;width:9px;height:9px;border-radius:2px;background:${colorOf(s.status)};margin-right:5px"></span>${s.name}</li>`).join("")}</ul>`);
  const ds = downstreamOf(n.id);
  rows.push(`<div class="row"><span class="k">下游</span>${ds.length} 项${ds.length ? "：" + ds.slice(0, 8).map(i => nodesById[i].name).join("、") + (ds.length > 8 ? "…" : "") : ""}</div>`);
  if (n.isBlocker) rows.push(`<div class="impact">⛔ <b>卡点影响分析</b>：该节点直接/间接阻塞 <b>${ds.length}</b> 个下游任务。建议优先攻克。</div>`);
  d.innerHTML = rows.join("");
}

/* ---------- 筛选 / 关键路径 ---------- */
function applyFilter() {
  const keep = new Set(curFilter === "all" ? DATA.nodes.map(n => n.id) : DATA.nodes.filter(n => n.status === curFilter || (curFilter === "blocked" && n.badge === "bolt")).map(n => n.id));
  d3.selectAll(".node").classed("hide", function () { return !keep.has(this.getAttribute("data-id")); });
  d3.selectAll(".edge").classed("hide", function () { return !(keep.has(this.getAttribute("data-from")) && keep.has(this.getAttribute("data-to"))); });
}
function applyCritical() {
  const critE = DATA.edges.filter(e => e.critical), critN = new Set([...critE.flatMap(e => [e.from, e.to])]);
  d3.selectAll(".node").classed("dim", function () { return criticalOn && !critN.has(this.getAttribute("data-id")); });
  d3.selectAll(".edge").classed("dim", function () { return criticalOn && !this.classList.contains("critical"); });
}

/* ---------- 甘特辅助视图（泳道 × 状态条） ---------- */
function renderGantt() {
  const el = document.getElementById("gantt");
  el.innerHTML = DATA.lanes.map(l => {
    const ns = DATA.nodes.filter(n => n.lane === l.id);
    return `<div class="g-row"><div class="g-lane">${l.name}</div><div class="g-cells">` +
      ns.map(n => `<div class="g-cell" style="background:${colorOf(n.status)}" title="${n.name}" data-id="${n.id}">${n.name.replace(/[^\u4e00-\u9fa5A-Za-z0-9]/g, "").slice(0, 6)}</div>`).join("") +
      `</div></div>`;
  }).join("");
  el.querySelectorAll(".g-cell").forEach(c => c.onclick = () => selectNode(nodesById[c.dataset.id]));
}

/* ---------- 底部卡点清单（按下游影响排序 + 建议攻克顺序） ---------- */
function renderBlockers() {
  const impactOf = id => downstreamOf(id).length;
  const blks = DATA.nodes.filter(n => n.isBlocker || n.badge === "bolt")
    .map(n => ({ n, imp: n.isBlocker ? impactOf(n.id) : impactOf(n.id) })).sort((a, b) => b.imp - a.imp);
  const el = document.getElementById("blockers");
  el.innerHTML = `<h2>⛔ 当前卡点清单（按下游影响排序 · 建议攻克顺序即列表顺序）</h2>` +
    blks.map((b, i) => {
      const isRoot = b.n.isBlocker;
      const ds = downstreamOf(b.n.id);
      const sug = isRoot
        ? `建议第 ${i + 1} 位攻克：修复清单已交付（T13 排雷）——2×import + 1×模板笔误 + 3×发射族，一次做全避免再 re-tip。`
        : `随上游解除自动解锁（等 ${b.n.waitOn || b.n.blockReason || "触发条件"}）。`;
      return `<div class="blk"><b>#${i + 1} ${b.n.name}</b> · 阻塞 <b>${ds.length}</b> 项下游${ds.length ? "（" + ds.slice(0, 6).map(x => nodesById[x].name).join("、") + (ds.length > 6 ? "…" : "") + "）" : ""}<div class="why">${b.n.note || ""}</div><div class="sug">${sug}</div></div>`;
    }).join("");
}

/* ---------- 缩放适配 / 全屏 ---------- */
function fitView() {
  const svgN = document.getElementById("graph");
  const vw = svgN.clientWidth || 1200, vh = svgN.clientHeight || 700;
  const xs = DATA.nodes.map(n => n.x), ys = DATA.nodes.map(n => n.y);
  const x0 = Math.min(...xs) - 30, y0 = Math.min(...ys) - 40;
  const bw = Math.max(...xs) + W + 60 - x0, bh = Math.max(...ys) + H + 60 - y0;
  const k = Math.min(vw / bw, vh / bh, 1.15);
  const tx = (vw - bw * k) / 2 - x0 * k, ty = (vh - bh * k) / 2 - y0 * k;
  svgN.__zoom ? null : null;
  d3.select(svgN).call(window.__zoom.transform, d3.zoomIdentity.translate(tx, ty).scale(k));
}
function stepZoom(f) {
  const svgN = document.getElementById("graph");
  d3.select(svgN).transition().duration(220).call(window.__zoom.scaleBy, f);
}
function toggleFull() {
  try {
    if (!document.fullscreenElement) (document.documentElement.requestFullscreen || document.documentElement.webkitRequestFullscreen).call(document.documentElement);
    else (document.exitFullscreen || document.webkitExitFullscreen).call(document);
  } catch (e) { document.getElementById("fsHint").textContent = "此环境不允许全屏，请用浏览器直接打开文件"; }
}

/* ---------- 控件绑定与启动 ---------- */
function bindControls() {
  const fb = document.getElementById("filters");
  [["all", "全部"], ...Object.entries(STATUS).map(([k, v]) => [k, v.label])].forEach(([k, label]) => {
    const b = document.createElement("button"); b.textContent = label; if (k === "all") b.classList.add("on");
    b.onclick = () => { fb.querySelectorAll("button").forEach(x => x.classList.remove("on")); b.classList.add("on"); curFilter = k; applyFilter(); };
    fb.appendChild(b);
  });
  document.getElementById("btnCritical").onclick = e => { criticalOn = !criticalOn; e.target.classList.toggle("on", criticalOn); applyCritical(); };
  document.getElementById("btnZoomIn").onclick = () => stepZoom(1.3);
  document.getElementById("btnZoomOut").onclick = () => stepZoom(1 / 1.3);
  document.getElementById("btnFit").onclick = fitView;
  document.getElementById("btnFull").onclick = toggleFull;
  document.getElementById("btnView").onclick = e => {
    ganttOn = !ganttOn; e.target.classList.toggle("on", ganttOn); e.target.textContent = ganttOn ? "DAG 视图" : "甘特视图";
    document.getElementById("graphWrap").style.display = ganttOn ? "none" : "block";
    document.getElementById("gantt").style.display = ganttOn ? "block" : "none";
  };
}
(async function init() {
  await loadData();
  nodesById = Object.fromEntries(DATA.nodes.map(n => [n.id, n]));
  adjacency = { up: {}, down: {} };
  DATA.edges.forEach(e => { (adjacency.down[e.from] = adjacency.down[e.from] || []).push(e.to); (adjacency.up[e.to] = adjacency.up[e.to] || []).push(e.from); });
  document.querySelector("header .ts").textContent = DATA.meta.timestamp + " · " + DATA.meta.note;
  renderStats(); renderPipe(); renderGraph(); renderGantt(); renderBlockers(); bindControls();
  selectNode(nodesById["k-fix"]); // 默认展示当前关键卡点
})();

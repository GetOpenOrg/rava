/* rava 项目进展全景 · D3 + dagre 泳道 DAG
 * 数据：data.js 的全局 DASHBOARD_DATA——更新进展只改数据，逻辑不携带任何具体节点内容。
 * 视图：默认折叠已完成（里程碑/上下文保留，其余按泳道收成汇总节点），顶栏「折叠已完成」可切换。 */
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
let foldOn = true, VIEW = null, viewById = {}, foldMembers = {};

async function loadData() {
  // 唯一数据约定：data.js 里的全局 const DASHBOARD_DATA
  DATA = (typeof DASHBOARD_DATA !== "undefined") ? DASHBOARD_DATA : null;
  if (!DATA) {
    document.body.insertAdjacentHTML("afterbegin", '<div style="padding:10px 20px;color:#f0978f">数据加载失败：data.js 未定义全局 DASHBOARD_DATA（唯一约定），请检查 data.js 是否随 index.html 一并加载。</div>');
    throw new Error("DASHBOARD_DATA undefined");
  }
}
const colorOf = s => ({ done:"#2e9e5b", doing:"#2f7fd3", blocked:"#d64541", risk:"#e08a1e", waiting:"#8e5bd3", todo:"#9aa3ab" }[s] || "#9aa3ab");

/* 按像素截断——中英文混排下按字符数截断会溢出节点框 */
const _meas = document.createElement("canvas").getContext("2d");
function ellipsize(text, maxPx, font) {
  _meas.font = font;
  if (_meas.measureText(text).width <= maxPx) return text;
  let s = text;
  while (s.length > 1 && _meas.measureText(s + "…").width > maxPx) s = s.slice(0, -1);
  return s + "…";
}
const F_TITLE = "600 12.5px -apple-system, 'Segoe UI', 'PingFang SC', 'Microsoft YaHei', sans-serif";
const F_SMALL = "10px -apple-system, 'Segoe UI', 'PingFang SC', 'Microsoft YaHei', sans-serif";

/* 状态色按权重混入面板底色——结果不透明，节点画在边之上，穿行边不透出 */
const PANEL = "#1c2027";
function mixColor(a, b, t) {
  const pa = [1, 3, 5].map(i => parseInt(a.slice(i, i + 2), 16));
  const pb = [1, 3, 5].map(i => parseInt(b.slice(i, i + 2), 16));
  return "#" + pa.map((v, i) => Math.round(v * t + pb[i] * (1 - t)).toString(16).padStart(2, "0")).join("");
}

/* ---------- 统计条 ---------- */
function renderStats() {
  const n = DATA.nodes.filter(x => x.type !== "context");
  const cnt = k => n.filter(x => x.status === k).length;
  const el = document.getElementById("stats");
  el.innerHTML = `<span>✅ <b>${cnt("done")}</b> 已完成</span><span>🔄 <b>${cnt("doing")}</b> 进行中</span>` +
    `<span>🔴 <b>${cnt("blocked")}</b> 卡点</span><span>🟠 <b>${cnt("risk")}</b> 风险</span><span>🟣 <b>${cnt("waiting")}</b> 等待</span><span>⚪ <b>${cnt("todo")}</b> 排队</span>` +
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
  VIEW.nodes.forEach(n => g.setNode(n.id, { width: W, height: H }));
  VIEW.edges.forEach(e => g.setEdge(e.from, e.to));
  dagre.layout(g);
  // 列距随间隙边密度自适应：跨列边的竖直过渡都铺在列间隙里，最挤的间隙决定全局列距
  // （间隙可用宽度 = ranksep − 40；保证相邻竖直段间距 ≥ 12px）
  const xs0 = VIEW.nodes.map(n => g.node(n.id).x).sort((a, b) => a - b);
  const cols0 = [];
  xs0.forEach(x => { if (!cols0.length || x - cols0[cols0.length - 1] > 40) cols0.push(x); });
  const ci0 = x => { let best = 0; cols0.forEach((c, i) => { if (Math.abs(c - x) < Math.abs(cols0[best] - x)) best = i; }); return best; };
  const gapCount = {};
  VIEW.edges.forEach(e => {
    const a = g.node(e.from), b = g.node(e.to);
    if (!a || !b) return;
    const i = ci0(a.x), j = ci0(b.x);
    if (j > i + 1) { const k = i + Math.floor((j - i) / 2); gapCount[k] = (gapCount[k] || 0) + 1; }
  });
  const densest = Math.max(0, ...Object.values(gapCount));
  const ranksep = Math.min(260, Math.max(90, (densest - 1) * 12 + 40));
  if (ranksep > 90) {
    g.setGraph({ rankdir: "LR", nodesep: 22, ranksep, edgesep: 10 });
    dagre.layout(g); // 拓扑不变，仅按新列距重排坐标
  }
  // 纵向吸附到泳道带：同泳道内按 dagre 的 y 排序铺开（泳道内每节点独占一行）。
  // n.x/n.y 与 dagre 同语义=节点中心；边不取 dagre 折线，由 renderGraph 按列间隙平滑布线。
  const laneOrder = DATA.lanes.map(l => l.id);
  const laneGroups = {};
  VIEW.nodes.forEach(n => {
    const li = laneOrder.indexOf(n.lane);
    (laneGroups[li] = laneGroups[li] || []).push({ id: n.id, y: g.node(n.id).y });
  });
  let laneTops = [], yCursor = 0;
  laneOrder.forEach((_, li) => {
    const items = (laneGroups[li] || []).sort((a, b) => a.y - b.y);
    const h = Math.max(items.length * (H + PADY), 120);
    laneTops[li] = yCursor;
    items.forEach((it, i) => {
      const nd = g.node(it.id);
      nd.y = yCursor + 30 + H / 2 + i * (H + PADY); // 节点中心：顶部留 30px 泳道标签高度
    });
    yCursor += h + 40;
  });
  const xs = VIEW.nodes.map(n => g.node(n.id).x);
  const w = Math.max(...xs) + W + 140, h = yCursor + 60;
  VIEW.nodes.forEach(n => { const nd = g.node(n.id); n.x = nd.x; n.y = nd.y; });
  return { w, h, laneTops };
}

/* ---------- 折叠已完成：派生视图（不动 data.js） ----------
 * 隐藏 done 的工作节点（里程碑/上下文保留），每条泳道收成一个「已完成 ×N」汇总节点；
 * 隐藏→保留 的边改道为 汇总节点→保留节点（关键路径标记随之带过去）；
 * 保留→隐藏 的边沿隐藏链改道直连其保留下游。 */
function buildFoldView() {
  const hidden = DATA.nodes.filter(n => n.status === "done" && n.type !== "milestone" && n.type !== "context");
  const hiddenSet = new Set(hidden.map(n => n.id));
  const kept = DATA.nodes.filter(n => !hiddenSet.has(n.id));
  const downMemo = {};
  const keptDown = id => {
    if (!(id in downMemo)) downMemo[id] = (adjacency.down[id] || []).flatMap(t => hiddenSet.has(t) ? keptDown(t) : [t]);
    return downMemo[id];
  };
  const edgeMap = new Map();
  const addEdge = (from, to, label, critical) => {
    const k = from + "→" + to;
    if (edgeMap.has(k)) { const ex = edgeMap.get(k); ex.critical = ex.critical || critical; return; }
    edgeMap.set(k, { from, to, label, critical: !!critical });
  };
  DATA.edges.forEach(e => {
    const uk = !hiddenSet.has(e.from), vk = !hiddenSet.has(e.to);
    if (uk && vk) addEdge(e.from, e.to, e.label, e.critical);
    else if (!uk && vk) addEdge("fold-" + nodesById[e.from].lane, e.to, e.label, e.critical);
    else if (uk) keptDown(e.to).forEach(w => addEdge(e.from, w, undefined, e.critical));
    // 双端均隐藏：由上面的改道规则覆盖；不通向任何保留节点的尾巴自然丢弃
  });
  const byLane = {};
  hidden.forEach(n => (byLane[n.lane] = byLane[n.lane] || []).push(n));
  foldMembers = Object.fromEntries(Object.entries(byLane).map(([lane, ns]) => ["fold-" + lane, ns.map(n => n.id)]));
  const foldNodes = Object.entries(byLane).map(([lane, ns]) => ({
    id: "fold-" + lane, lane, status: "done", type: "fold",
    name: "已完成 ×" + ns.length,
    note: "顶栏「折叠已完成」可展开",
    sub: ns.map(n => ({ name: n.name, status: "done" })),
  }));
  return { nodes: kept.concat(foldNodes), edges: [...edgeMap.values()] };
}
function rebuildView() {
  VIEW = foldOn ? buildFoldView() : { nodes: DATA.nodes, edges: DATA.edges };
  viewById = Object.fromEntries(VIEW.nodes.map(n => [n.id, n]));
  renderGraph();
}

/* ---------- 主图渲染 ---------- */
function renderGraph() {
  const { w, h, laneTops } = layout();
  const svg = d3.select("#graph");
  svg.selectAll("*").remove(); // 不设 viewBox：SVG 用户单位 = CSS 像素，zoom transform 是唯一缩放层
  const root = svg.append("g");
  // 泳道底带
  DATA.lanes.forEach((l, i) => {
    root.append("rect").attr("class", "laneband").attr("x", 0).attr("y", laneTops[i])
      .attr("width", w).attr("height", (laneTops[i + 1] ?? h - 40) - laneTops[i] - 16).attr("rx", 10);
    root.append("text").attr("class", "lanelabel").attr("x", 14).attr("y", laneTops[i] + 22).text(l.name);
  });
  // 边：g.edge（状态类+数据属性）内含 .line 可见线、.lbl 标签、.hit 加宽热区（悬停/点击）。
  // 相邻列走端点扇形分布的贝塞尔 S 曲线（整段在列间隙内）；跨列走「水平—大圆角竖直过渡—水平」，
  // 竖直过渡放列间隙、圆角限制在间隙内、水平段贴源/目标泳道行——均不压任何无关节点。
  // 同源出边沿源右边框扇形散开、同汇入边沿目标左边框扇形散开，同向边不再重叠成一股。
  const defs = svg.append("defs");
  defs.append("marker").attr("id", "arrow").attr("viewBox", "0 0 10 10").attr("refX", 9).attr("refY", 5)
    .attr("markerWidth", 5.5).attr("markerHeight", 5.5).attr("orient", "auto-start-reverse")
    .append("path").attr("d", "M0,1 L9,5 L0,9 z").attr("fill", "context-stroke");
  const cols = [];
  VIEW.nodes.map(n => n.x).sort((a, b) => a - b).forEach(x => { if (!cols.length || x - cols[cols.length - 1] > 40) cols.push(x); });
  const colOf = id => { const x = viewById[id].x; let best = 0; cols.forEach((c, i) => { if (Math.abs(c - x) < Math.abs(cols[best] - x)) best = i; }); return best; };
  const outs = {}, ins = {};
  VIEW.edges.forEach(e => { (outs[e.from] = outs[e.from] || []).push(e); (ins[e.to] = ins[e.to] || []).push(e); });
  const fan = (list, key) => {
    const sorted = [...list].sort((a, b) => key(a) - key(b));
    const map = new Map();
    sorted.forEach((e, i) => map.set(e, sorted.length > 1 ? (i / (sorted.length - 1) - 0.5) * Math.min(H - 10, sorted.length * 9) : 0));
    return map;
  };
  const outOff = new Map(), inOff = new Map();
  Object.values(outs).forEach(list => fan(list, e => (viewById[e.to] || { y: 0 }).y).forEach((v, e) => outOff.set(e, v)));
  Object.values(ins).forEach(list => fan(list, e => (viewById[e.from] || { y: 0 }).y).forEach((v, e) => inOff.set(e, v)));
  const plans = VIEW.edges.filter(e => viewById[e.from] && viewById[e.to]).map(e => ({ e, i: colOf(e.from), j: colOf(e.to) }));
  const gapUse = {};
  plans.forEach(p => { if (p.j > p.i + 1) { const k = p.i + Math.floor((p.j - p.i) / 2); gapUse[k] = (gapUse[k] || 0) + 1; } });
  const gapSeen = {};
  const edgeG = root.append("g");
  plans.forEach(({ e, i, j }) => {
    const s = viewById[e.from], t = viewById[e.to];
    const sx = s.x + W / 2, sy = s.y + (outOff.get(e) || 0), tx = t.x - W / 2, ty = t.y + (inOff.get(e) || 0);
    let d, lx = (sx + tx) / 2;
    if (j <= i + 1 || tx <= sx + 8) {
      const dx = Math.max((tx - sx) * 0.5, 8); // 相邻列：间隙内水平切线 S 曲线
      d = `M${sx},${sy} C${sx + dx},${sy} ${tx - dx},${ty} ${tx},${ty}`;
    } else {
      const k = i + Math.floor((j - i) / 2);
      const gapLo = cols[k] + W / 2 + 6, gapHi = cols[k + 1] - W / 2 - 6;
      const mid = (gapLo + gapHi) / 2;
      const lo = Math.min(gapLo + 14, mid), hi = Math.max(gapHi - 14, mid); // 两端留圆角余量
      const total = gapUse[k], idx = gapSeen[k] = (gapSeen[k] || 0);
      const mx = total > 1 ? lo + (hi - lo) * (idx / Math.max(total - 1, 1)) : mid;
      const r = Math.max(0, Math.min(36, Math.abs(ty - sy) / 2, mx - gapLo, gapHi - mx));
      const dir = ty > sy ? 1 : -1;
      if (Math.abs(ty - sy) < 4) d = `M${sx},${sy} L${tx},${ty}`; // 同行：贴行水平线（安全）
      else if (r < 6) d = `M${sx},${sy} L${mx},${sy} L${mx},${ty} L${tx},${ty}`; // 直角肘形——竖直段仍在间隙内，禁止对角斜穿
      else d = `M${sx},${sy} L${mx - r},${sy} C${mx - r * 0.45},${sy} ${mx},${sy + dir * r * 0.55} ${mx},${sy + dir * r} L${mx},${ty - dir * r} C${mx},${ty - dir * r * 0.55} ${mx + r * 0.45},${ty} ${mx + r},${ty} L${tx},${ty}`;
      lx = mid; // 标签放竖直过渡段中部
    }
    const cls = ["edge", e.critical ? "critical" : "",
      e.status ? "e-" + e.status : (s.status === "done" ? "e-done" : (t.status === "blocked" ? "e-blocked" : "e-doing"))].join(" ");
    const grp = edgeG.append("g").attr("class", cls).attr("data-from", e.from).attr("data-to", e.to);
    grp.append("path").attr("class", "line").attr("d", d);
    if (e.label) grp.append("text").attr("class", "lbl").attr("x", lx).attr("y", (sy + ty) / 2 - 6).attr("text-anchor", "middle").text(e.label);
    grp.append("path").attr("class", "hit").attr("d", d)
      .on("click", () => selectNode(viewById[e.to]))
      .on("mouseenter", () => hoverEdge(e.from, e.to))
      .on("mouseleave", clearEdgeHover);
  });
  // 节点
  const ng = root.append("g");
  VIEW.nodes.forEach(n => {
    const grp = ng.append("g").attr("class", `node ${n.type || ""} ${n.critical ? "crit" : ""}`).attr("transform", `translate(${n.x - W / 2},${n.y - H / 2})`)
      .attr("data-id", n.id);
    grp.append("rect").attr("class", "box").attr("width", W).attr("height", H).attr("rx", n.type === "milestone" ? 20 : 8)
      .attr("fill", mixColor(colorOf(n.status), PANEL, n.type === "fold" ? 0.1 : 0.16)); // 状态底色：缩放概览下也能读出颜色语义
    grp.append("rect").attr("width", 5).attr("height", H).attr("rx", 2).attr("fill", colorOf(n.status)); // 左状态条
    grp.append("text").attr("class", "title").attr("x", 12).attr("y", 20)
      .text(ellipsize(n.name, n.badge ? 136 : 156, F_TITLE));
    if (n.badge) grp.append("text").attr("class", "badge").attr("x", W - 20).attr("y", 20).text(BADGE[n.badge] || "•");
    // 分段子任务条（多色）
    if (n.sub && n.sub.length) {
      const bw = W - 24, x0 = 12, y0 = H - 18;
      let x = x0;
      n.sub.forEach(s => { const seg = bw / n.sub.length;
        grp.append("rect").attr("x", x).attr("y", y0).attr("width", seg - 2).attr("height", 7).attr("rx", 2).attr("fill", colorOf(s.status)); x += seg; });
    }
    if (n.waitOn) grp.append("text").attr("class", "wait").attr("x", 12).attr("y", H - 26).text(ellipsize("等：" + n.waitOn, 156, F_SMALL));
    else if (n.note) grp.append("text").attr("class", "wait").attr("x", 12).attr("y", H - 26).text(ellipsize(n.note, 156, F_SMALL));
    grp.on("click", () => selectNode(n)).on("mouseenter", () => highlight(n.id)).on("mouseleave", clearHighlight);
  });
  // 图例（重渲染前先移除旧例，避免重复堆积）
  d3.select("#legend").remove();
  const lg = d3.select("#graphWrap").append("div").attr("id", "legend");
  lg.html(Object.entries(STATUS).map(([k, v]) => `<span class="it"><span class="sw" style="background:${colorOf(k)}"></span>${v.label}</span>`).join("")
    + `<span class="it">━ 红 = 关键路径</span><span class="it">内嵌小条 = 子任务状态</span>`
    + (foldOn ? `<span class="it">虚线框 = 泳道已完成汇总</span>` : ""));
  applyFilter(); if (criticalOn) applyCritical();
  const zb = d3.zoom().scaleExtent([0.05, 8]).on("zoom", ev => root.attr("transform", ev.transform));
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
  // 折叠汇总节点悬停时，把其成员的上下游链路一并点亮
  const seeds = foldMembers[id] ? [id, ...foldMembers[id]] : [id];
  const keep = new Set(seeds.flatMap(s => [...reach(s, "down"), ...reach(s, "up")]));
  d3.selectAll(".node").classed("dim", function () { return !keep.has(this.getAttribute("data-id")); });
  d3.selectAll(".edge").classed("dim", function () { return !(keep.has(this.getAttribute("data-from")) && keep.has(this.getAttribute("data-to"))); });
}
function clearHighlight() { d3.selectAll(".node").classed("dim", false); d3.selectAll(".edge").classed("dim", false); }

/* ---------- 边悬停：加粗高亮自身（CSS .hov）+ 点亮两端节点，其余边压暗 ---------- */
function hoverEdge(from, to) {
  d3.selectAll("g.edge").classed("dim", function () { return this.getAttribute("data-from") !== from || this.getAttribute("data-to") !== to; });
  d3.selectAll(".node").classed("hl", function () { const id = this.getAttribute("data-id"); return id === from || id === to; });
}
function clearEdgeHover() {
  d3.selectAll("g.edge").classed("dim", false);
  d3.selectAll(".node").classed("hl", false);
  if (criticalOn) applyCritical(); // 恢复关键路径模式的压暗态
}

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
  if (curFilter === "all" || curFilter === "done") VIEW.nodes.forEach(n => { if (n.type === "fold") keep.add(n.id); }); // 汇总节点本身即 done
  d3.selectAll(".node").classed("hide", function () { return !keep.has(this.getAttribute("data-id")); });
  d3.selectAll(".edge").classed("hide", function () { return !(keep.has(this.getAttribute("data-from")) && keep.has(this.getAttribute("data-to"))); });
}
function applyCritical() {
  const critE = DATA.edges.filter(e => e.critical), critN = new Set([...critE.flatMap(e => [e.from, e.to])]);
  Object.entries(foldMembers).forEach(([fid, ms]) => { if (ms.some(m => critN.has(m))) critN.add(fid); }); // 成员在关键路径上则汇总节点也算
  d3.selectAll(".node").classed("dim", function () { return criticalOn && !critN.has(this.getAttribute("data-id")); });
  d3.selectAll(".edge").classed("dim", function () { return criticalOn && !this.classList.contains("critical"); });
}

/* ---------- 甘特辅助视图（泳道 × 状态条） ---------- */
function renderGantt() {
  const el = document.getElementById("gantt");
  el.innerHTML = DATA.lanes.map(l => {
    const ns = DATA.nodes.filter(n => n.lane === l.id);
    return `<div class="g-row"><div class="g-lane">${l.name}</div><div class="g-cells">` +
      ns.map(n => `<div class="g-cell" style="background:${colorOf(n.status)}" title="${n.name}" data-id="${n.id}">${n.name.replace(/[^\u4e00-\u9fa5A-Za-z0-9]/g, "").slice(0, 8)}</div>`).join("") +
      `</div></div>`;
  }).join("");
  el.querySelectorAll(".g-cell").forEach(c => c.onclick = () => selectNode(nodesById[c.dataset.id]));
}

/* ---------- 底部卡点清单（默认折叠成标题栏，按下游影响排序；建议文案来自数据 sug 字段） ---------- */
function renderBlockers() {
  const impactOf = id => downstreamOf(id).length;
  const blks = DATA.nodes.filter(n => n.isBlocker || n.badge === "bolt")
    .map(n => ({ n, imp: impactOf(n.id) })).sort((a, b) => b.imp - a.imp);
  const el = document.getElementById("blockers");
  el.innerHTML = `<h2>⛔ 当前卡点清单（${blks.length} 项 · 按下游影响排序）<span class="tgl">展开 ▾</span></h2>` +
    `<div class="blk-list">` + blks.map((b, i) => {
      const ds = downstreamOf(b.n.id);
      const sug = b.n.sug || (b.n.isBlocker
        ? `建议第 ${i + 1} 位攻克：当前下游影响最大的主动卡点。`
        : `随上游解除自动解锁（等 ${b.n.waitOn || b.n.blockReason || "触发条件"}）。`);
      return `<div class="blk"><b>#${i + 1} ${b.n.name}</b> · 阻塞 <b>${ds.length}</b> 项下游${ds.length ? "（" + ds.slice(0, 6).map(x => nodesById[x].name).join("、") + (ds.length > 6 ? "…" : "") + "）" : ""}<div class="why">${b.n.note || ""}</div><div class="sug">${sug}</div></div>`;
    }).join("") + `</div>`;
  const head = el.querySelector("h2");
  head.onclick = () => {
    const open = el.classList.toggle("open");
    head.querySelector(".tgl").textContent = open ? "收起 ▴" : "展开 ▾";
    if (!ganttOn) fitView(); // 面板高度变化后重新适配
  };
}

/* ---------- 缩放适配 / 全屏 ---------- */
function fitView() {
  if (!VIEW || !VIEW.nodes.length || VIEW.nodes[0].x === undefined) return;
  const svgN = document.getElementById("graph");
  const vw = svgN.clientWidth || 1200, vh = svgN.clientHeight || 700;
  const xs = VIEW.nodes.map(n => n.x), ys = VIEW.nodes.map(n => n.y);
  const x0 = Math.min(...xs) - 30, y0 = Math.min(...ys) - 40;
  const bw = Math.max(...xs) + W + 60 - x0, bh = Math.max(...ys) + H + 60 - y0;
  const k = Math.min(vw / bw, vh / bh, 1.15);
  const tx = (vw - bw * k) / 2 - x0 * k, ty = (vh - bh * k) / 2 - y0 * k;
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
  const btnFold = document.getElementById("btnFold");
  btnFold.onclick = () => { foldOn = !foldOn; btnFold.classList.toggle("on", foldOn); rebuildView(); };
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
  // 全部面板先渲染完（含底部卡点清单），再渲主图——fitView 必须基于最终视口尺寸
  renderStats(); renderPipe(); renderGantt(); renderBlockers(); bindControls();
  rebuildView();
  selectNode(nodesById["k-fix"]); // 默认展示当前关键卡点
  window.addEventListener("resize", () => { if (!ganttOn) fitView(); });
})();

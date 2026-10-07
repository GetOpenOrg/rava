"""
服务器硬件状态检查：CPU 负载 / 频率 / 温度、风扇转速、内存、磁盘与 SMART、内核硬件错误、系统单元。
只读采集（sudo -n 免密时读取 SMART 与内核日志），按阈值标注 [警告] / [严重]。

服务器取自本机集群清单（cluster_config.CONFIG_PATH，与分发脚本同一份）。

用法：
  uv run --group cluster python scripts/cluster/hw_check.py              # 缺省检查 dev
  uv run --group cluster python scripts/cluster/hw_check.py dev ubuntu   # 多台
  uv run --group cluster python scripts/cluster/hw_check.py dev --watch 5   # 每 5 秒一行精简读数（Ctrl-C 结束）
"""
import argparse
import json
import shlex
import sys
import time

import cluster_config as config
from ssh_client import create_ssh_client

# 阈值（摄氏度 / 百分比）
CPU_TEMP_WARN, CPU_TEMP_CRIT = 85, 92      # Ryzen 9000 系 Tjmax 95°C
NVME_TEMP_WARN, NVME_TEMP_CRIT = 70, 80
DIMM_TEMP_WARN = 55                        # SPD5118 自报 high
DISK_WARN, DISK_CRIT = 85, 95
MEM_AVAIL_WARN = 10
RAIL_TOLERANCE = 0.05                      # 3.3V 轨允许偏差

COLLECT = r'''
sec(){ echo "### $1"; }
sec uptime;   cat /proc/uptime; cat /proc/loadavg; nproc
sec model;    grep -m1 "model name" /proc/cpuinfo | cut -d: -f2-
sec board;    cat /sys/class/dmi/id/board_vendor /sys/class/dmi/id/board_name 2>/dev/null | paste -sd" "
sec stat1;    head -1 /proc/stat; sleep 1
sec stat2;    head -1 /proc/stat
sec freq;     cat /sys/devices/system/cpu/cpu*/cpufreq/scaling_cur_freq 2>/dev/null
sec maxfreq;  cat /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq 2>/dev/null
sec governor; cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null
sec pressure; for r in cpu memory io; do echo "$r $(head -1 /proc/pressure/$r 2>/dev/null)"; done
sec iowait
grep procs_blocked /proc/stat | cut -d" " -f2
W=$(sudo -n sh -c 'for t in /proc/[0-9]*/task/[0-9]*; do [ "$(cat $t/wchan 2>/dev/null)" = io_cqring_wait ] && echo "$(cat /proc/${t#/proc/}/../../comm 2>/dev/null)"; done' 2>/dev/null)
echo "$W" | grep -c .
sec iowait_who; echo "$W" | grep . | sort | uniq -c | sort -rn | head -3 | awk '{printf "%s×%s ", $2, $1} END{print ""}'
sec sensors;  sensors -j 2>/dev/null
sec meminfo;  grep -E "^(MemTotal|MemAvailable|SwapTotal|SwapFree):" /proc/meminfo
sec df;       df -P -x tmpfs -x devtmpfs -x squashfs -x overlay -x efivarfs 2>/dev/null | tail -n +2
sec smart
for d in $(lsblk -dno NAME,TYPE 2>/dev/null | awk '$2=="disk"{print $1}'); do
  echo "@@ $d $(lsblk -dno MODEL,TRAN /dev/$d | xargs)"
  sudo -n smartctl -j -H -A /dev/$d 2>/dev/null | tr -d "\n"; echo
done
sec kernel
if sudo -n true 2>/dev/null; then
  sudo -n journalctl -k -b --no-pager -o short-monotonic 2>/dev/null \
    | grep -iE "mce:.*(error|event)|hardware error|edac.*(ce|ue|error)|thermal.*throttl|over.?temp|nvme.*(timeout|reset)|i/o error|ata.*(failed|error)" \
    | grep -viE "decoding enabled|edac mc: ver" | tail -8
else echo "(无 sudo 免密，跳过)"; fi
sec failed;   systemctl --failed --no-legend --plain 2>/dev/null | awk '{print $1}'
sec top;      ps -eo pid,user,pcpu,rss,etime,comm --sort=-pcpu --no-headers | head -8
'''

WATCH = r'''
read -r _ a1 b1 c1 d1 e1 f1 g1 h1 _ < /proc/stat; sleep 1; read -r _ a2 b2 c2 d2 e2 f2 g2 h2 _ < /proc/stat
echo "cpu $((a1+b1+c1+d1+e1+f1+g1+h1)) $((d1+e1)) $((a2+b2+c2+d2+e2+f2+g2+h2)) $((d2+e2))"
echo "load $(cut -d' ' -f1 /proc/loadavg)"
echo "freq $(cat /sys/devices/system/cpu/cpu*/cpufreq/scaling_cur_freq 2>/dev/null | awk '{s+=$1;if($1>m)m=$1}END{printf "%d %d", s/NR/1000, m/1000}')"
echo "mem $(awk '/MemTotal/{t=$2}/MemAvailable/{a=$2}END{printf "%.0f", 100-a*100/t}' /proc/meminfo)"
echo "sensors $(sensors -j 2>/dev/null | tr -d '\n')"
'''


def sections(text: str) -> dict[str, str]:
    out, name, buf = {}, None, []
    for line in text.splitlines():
        if line.startswith("### "):
            if name:
                out[name] = "\n".join(buf).strip()
            name, buf = line[4:].strip(), []
        else:
            buf.append(line)
    if name:
        out[name] = "\n".join(buf).strip()
    return out


def mark(value: float, warn: float, crit: float | None = None) -> str:
    if crit is not None and value >= crit:
        return "  [严重]"
    return "  [警告]" if value >= warn else ""


def io_wait_counts(raw: str) -> tuple[int, int]:
    """(procs_blocked, io_uring 等待线程数)。"""
    nums = [int(x) for x in raw.split() if x.isdigit()]
    return (nums + [0, 0])[0], (nums + [0, 0])[1]


def cpu_busy(stat1: str, stat2: str) -> float:
    a = [int(x) for x in stat1.split()[1:9]]
    b = [int(x) for x in stat2.split()[1:9]]
    total = sum(b) - sum(a)
    idle = (b[3] + b[4]) - (a[3] + a[4])
    return 100.0 * (total - idle) / total if total else 0.0


def sensor_readings(raw: str):
    """解析 sensors -j，返回 (cpu 温度[(名,值)], 其他温度[(芯片,名,值,high)], 风扇[(名,rpm)], 电压{名:值}, 告警[str])。"""
    try:
        chips = json.loads(raw) if raw else {}
    except json.JSONDecodeError:
        return [], [], [], {}, []
    cpu, temps, fans, volts, alarms = [], [], [], {}, []
    for chip, feats in chips.items():
        for feat, vals in feats.items():
            if not isinstance(vals, dict):
                continue
            for key, v in vals.items():
                if key.startswith("temp") and key.endswith("_input"):
                    high = vals.get(key.replace("_input", "_max"))
                    if chip.startswith("k10temp") or chip.startswith("coretemp"):
                        cpu.append((feat, v))
                    elif v > 0:
                        temps.append((chip.split("-")[0], feat, v, high))
                elif key.startswith("fan") and key.endswith("_input"):
                    fans.append((feat, v))
                elif key.startswith("in") and key.endswith("_input"):
                    volts[feat] = v
                elif key.startswith("intrusion") and key.endswith("_alarm") and v:
                    alarms.append(f"{chip} {feat} 机箱开启告警（打开过机箱即会置位，可忽略或在 BIOS 清除）")
    return cpu, temps, fans, volts, alarms


def report(label: str, text: str) -> None:
    s = sections(text)
    W = 64
    print("=" * W)
    up, load, ncpu = (s.get("uptime", "").splitlines() + ["0", "0 0 0", "1"])[:3]
    days = float(up.split()[0]) / 86400
    print(f" {label}  {s.get('model', '').strip()}  |  {s.get('board', '')}")
    print(f" 运行 {days:.1f} 天")
    print("=" * W)

    # CPU
    n = int(ncpu or 1)
    l1, l5, l15 = (float(x) for x in load.split()[:3])
    busy = cpu_busy(s.get("stat1", "cpu 0 0 0 0 0 0 0 0"), s.get("stat2", "cpu 0 0 0 0 0 0 0 0"))
    freqs = [int(x) / 1000 for x in s.get("freq", "").split()]
    maxf = int(s.get("maxfreq") or 0) / 1000
    print("[CPU]")
    print(f"  使用率 {busy:5.1f}%   负载 {l1:.2f} / {l5:.2f} / {l15:.2f}（{n} 线程，1 分钟负载占 {100 * l1 / n:.0f}%）")
    if freqs:
        print(f"  频率 平均 {sum(freqs) / len(freqs):.0f} MHz  最高 {max(freqs):.0f}  最低 {min(freqs):.0f}"
              f"  （标称上限 {maxf:.0f}，调速 {s.get('governor', '?')}）")
    for line in s.get("pressure", "").splitlines():
        parts = line.split()
        if len(parts) >= 3:
            avg10 = float(parts[2].split("=")[1])
            note = mark(avg10, 20, 50)
            if parts[0] == "io" and note:
                # io_uring 等待完成事件的线程计入 iowait / PSI，磁盘空闲时也会显示满压力
                blocked, ring = io_wait_counts(s.get("iowait", ""))
                if ring and ring >= 0.8 * blocked:
                    note = f"  （{ring} 个线程在 io_uring 上等待被计入，非磁盘瓶颈：{s.get('iowait_who', '')}）"
            print(f"  压力 {parts[0]:<6} some avg10={avg10:.2f}%{note}")

    cpu_t, temps, fans, volts, alarms = sensor_readings(s.get("sensors", ""))
    print("[温度]")
    if not cpu_t and not temps:
        print("  （无 sensors 数据：未安装 lm-sensors 或无可读传感器）")
    for name, v in cpu_t:
        print(f"  CPU {name:<12} {v:5.1f}°C{mark(v, CPU_TEMP_WARN, CPU_TEMP_CRIT)}")
    for chip, name, v, high in temps:
        if chip == "nvme":
            m = mark(v, NVME_TEMP_WARN, NVME_TEMP_CRIT)
        elif chip == "spd5118":
            m, chip = mark(v, DIMM_TEMP_WARN), "内存条"
        elif chip.startswith("nct") or chip.startswith("it87"):
            continue   # 主板 SuperIO 的温度探头名称不可靠，CPU 以 k10temp 为准
        else:
            m = mark(v, (high or 90) - 5, high) if high and high < 200 else ""
        print(f"  {chip[:8]:<8} {name:<12} {v:5.1f}°C{m}")

    print("[风扇]")
    spinning = [(f, r) for f, r in fans if r > 0]
    if not fans:
        print("  （无风扇传感器）")
    elif not spinning:
        print("  全部 0 RPM  [严重]")
    else:
        print("  " + "   ".join(f"{f} {r:.0f} RPM" for f, r in spinning)
              + f"   （另有 {len(fans) - len(spinning)} 个接口 0 RPM，多为未接）")
        if cpu_t and max(v for _, v in cpu_t) >= CPU_TEMP_WARN and max(r for _, r in spinning) < 1500:
            print("  CPU 高温但风扇转速偏低：检查 BIOS 风扇曲线  [警告]")

    rails = {k: v for k, v in volts.items() if k in ("VCC", "VSB", "AVSB", "3VCC", "+3.3V")}
    if rails or "VBAT" in volts:
        print("[电压]")
        parts = []
        for k, v in rails.items():
            parts.append(f"{k} {v:.2f}V" + ("[异常]" if abs(v - 3.3) / 3.3 > RAIL_TOLERANCE else ""))
        if "VBAT" in volts:
            parts.append(f"VBAT {volts['VBAT']:.2f}V" + ("[电池偏低]" if volts["VBAT"] < 2.8 else ""))
        print("  " + "   ".join(parts))

    # 内存
    mi = {l.split(":")[0]: int(l.split()[1]) for l in s.get("meminfo", "").splitlines() if ":" in l}
    if mi:
        tot, av = mi["MemTotal"], mi["MemAvailable"]
        sw_t, sw_f = mi.get("SwapTotal", 0), mi.get("SwapFree", 0)
        av_pct = 100 * av / tot
        print("[内存]")
        print(f"  {(tot - av) / 2**20:.1f} / {tot / 2**20:.1f} GiB 已用，可用 {av_pct:.0f}%{'  [警告]' if av_pct < MEM_AVAIL_WARN else ''}"
              f"   swap {(sw_t - sw_f) / 2**20:.1f} / {sw_t / 2**20:.1f} GiB")

    print("[磁盘]")
    for line in s.get("df", "").splitlines():
        p = line.split()
        if len(p) >= 6:
            pct = int(p[4].rstrip("%"))
            print(f"  {p[5]:<20} {int(p[2]) / 2**20:7.1f} / {int(p[1]) / 2**20:7.1f} GiB  {pct:3d}%{mark(pct, DISK_WARN, DISK_CRIT)}")
    smart = s.get("smart", "")
    for block in smart.split("@@ ")[1:]:
        head, _, body = block.partition("\n")
        dev = head.split()[0]
        model = " ".join(head.split()[1:])
        try:
            j = json.loads(body.strip())
        except json.JSONDecodeError:
            print(f"  {dev} {model}: SMART 不可读（无 sudo 免密或设备不支持）")
            continue
        ok = j.get("smart_status", {}).get("passed")
        info = [f"健康 {'通过' if ok else '未通过  [严重]' if ok is False else '未知'}"]
        nv = j.get("nvme_smart_health_information_log")
        if nv:
            info.append(f"磨损 {nv.get('percentage_used', 0)}%")
            info.append(f"写入 {nv.get('data_units_written', 0) * 512000 / 1e12:.2f} TB")
            info.append(f"通电 {nv.get('power_on_hours', 0)} h")
            if nv.get("media_errors"):
                info.append(f"介质错误 {nv['media_errors']}  [严重]")
            if nv.get("critical_warning"):
                info.append(f"严重告警位 {nv['critical_warning']:#x}  [严重]")
            if nv.get("unsafe_shutdowns"):
                info.append(f"非正常断电 {nv['unsafe_shutdowns']} 次")
        else:
            for a in j.get("ata_smart_attributes", {}).get("table", []):
                if a["id"] in (5, 187, 197, 198) and a["raw"]["value"]:
                    info.append(f"{a['name']} {a['raw']['value']}  [警告]")
        print(f"  {dev} {model}: " + "，".join(info))

    print("[硬件错误 / 系统]")
    kern = s.get("kernel", "").strip()
    print("  内核日志（本次启动）：" + ("无 MCE / EDAC / 过热 / 磁盘 I/O 错误" if not kern else ""))
    for line in kern.splitlines():
        print(f"    {line[:110]}")
    for a in alarms:
        print(f"  {a}")
    failed = s.get("failed", "").split()
    print(f"  失败的 systemd 单元：{', '.join(failed) if failed else '无'}")

    print("[CPU 占用前列]")
    for line in s.get("top", "").splitlines():
        p = line.split(None, 5)
        if len(p) == 6:
            print(f"  {p[0]:>7} {p[1]:<8} {float(p[2]):6.1f}% {int(p[3]) / 2**20:6.1f}G {p[4]:>12}  {p[5]}")
    print()


def connect(label: str) -> tuple:
    """按清单标签建连：先直连，清单配置了降级代理且条目非 direct_only 时再试代理。返回 (client, via)。"""
    srv = next((x for x in config.SERVERS if x["label"] == label), None)
    if srv is None:
        return None, ""
    direct = {k: v for k, v in srv.items() if k != "socks5_proxy"}
    methods = [(direct, "")]
    if config.FALLBACK_PROXY and not srv.get("direct_only"):
        proxy = config.FALLBACK_PROXY
        methods.append(({**direct, "socks5_proxy": proxy}, f"代理 {proxy['host']}:{proxy['port']}"))
    for cfg, via in methods:
        client = create_ssh_client(cfg, max_retries=2, verbose=False, timeout=config.CONNECT_TIMEOUT)
        if client is not None:
            return client, via
    return None, ""


def exec_script(client, script: str, timeout: int = 30) -> tuple[int, bytes]:
    """bash -c 执行脚本，返回 (退出码, stdout 字节)。"""
    stdin, stdout, _e = client.exec_command("bash -c " + shlex.quote(script), timeout=timeout)
    stdout.channel.settimeout(timeout)
    stdin.close()
    out = stdout.read()
    return stdout.channel.recv_exit_status(), out


def watch(label: str, client, interval: float) -> None:
    print(f"{label}  时间      CPU%   负载  平均/最高MHz  内存%  CPU°C  风扇RPM")
    while True:
        _, out = exec_script(client, WATCH, timeout=30)
        v = {}
        for line in out.decode(errors="replace").splitlines():
            k, _, rest = line.partition(" ")
            v[k] = rest
        t1, i1, t2, i2 = (int(x) for x in v["cpu"].split())
        busy = 100 * (1 - (i2 - i1) / max(t2 - t1, 1))
        cpu_t, _, fans, _, _ = sensor_readings(v.get("sensors", ""))
        tc = max((x for _, x in cpu_t), default=0)
        fan = " ".join(f"{r:.0f}" for _, r in fans if r > 0) or "-"
        avg, top = v.get("freq", "0 0").split()
        print(f"{time.strftime('%H:%M:%S')}  {busy:5.1f}  {float(v['load']):5.2f}  {avg:>5}/{top:<5}  "
              f"{v['mem']:>4}  {tc:5.1f}{'!' if tc >= CPU_TEMP_WARN else ' '}  {fan}", flush=True)
        time.sleep(max(interval - 1, 0))


def main() -> int:
    ap = argparse.ArgumentParser(description="服务器硬件状态检查")
    ap.add_argument("labels", nargs="*", default=["dev"], help="集群清单中的服务器标签，缺省 dev")
    ap.add_argument("--watch", type=float, metavar="秒", help="持续模式：按间隔输出一行精简读数（仅第一台）")
    args = ap.parse_args()

    rc = 0
    for label in args.labels:
        client, via = connect(label)
        if client is None:
            print(f"{label}: 连接失败")
            rc = 1
            continue
        try:
            if args.watch:
                watch(label, client, args.watch)
            _, out = exec_script(client, COLLECT, timeout=90)
            report(label + (f"（经 {via}）" if via else ""), out.decode(errors="replace"))
        except KeyboardInterrupt:
            return rc
        finally:
            client.close()
    return rc


if __name__ == "__main__":
    sys.exit(main())

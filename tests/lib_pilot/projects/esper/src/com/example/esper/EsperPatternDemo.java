package com.example.esper;

import com.espertech.esper.client.Configuration;
import com.espertech.esper.client.EPServiceProvider;
import com.espertech.esper.client.EPServiceProviderManager;
import com.espertech.esper.client.EPStatement;
import com.espertech.esper.client.EventBean;
import com.espertech.esper.client.time.CurrentTimeEvent;
import com.espertech.esper.client.UpdateListener;

/**
 * Esper CEP（二）：模式匹配 + 虚拟时钟（对应文章第二段 EPL：
 * pattern [A -> timer:within(5 sec) -> B]）。
 * 用 Esper 外部时钟（CurrentTimeEvent）推进虚拟时间，完全脱离墙钟：
 * 场景1 A 后 3 秒 B 到达 → 命中；场景2 A 后 6 秒 B 才到 → 超时不命中。
 *
 * 输出确定性说明：全部时间推进由虚拟事件驱动，与真实时间无关。
 */
public class EsperPatternDemo {

    public static class WithdrawEvent {
        private final String account;

        public WithdrawEvent(String account) {
            this.account = account;
        }

        public String getAccount() {
            return account;
        }
    }

    public static class VerifyEvent {
        private final String account;

        public VerifyEvent(String account) {
            this.account = account;
        }

        public String getAccount() {
            return account;
        }
    }

    public static void main(String[] args) {
        Configuration config = new Configuration();
        config.addEventType("Withdraw", WithdrawEvent.class.getName());
        config.addEventType("Verify", VerifyEvent.class.getName());
        config.getEngineDefaults().getThreading().setInternalTimerEnabled(false); // 关闭内部时钟

        EPServiceProvider epService = EPServiceProviderManager.getDefaultProvider(config);

        EPStatement stmt = epService.getEPAdministrator().createEPL(
                "select * from pattern [ every w=Withdraw -> "
                        + "(Verify(account=w.account) where timer:within(5 sec)) ]");
        // 语义（文章 EPL 的标准写法）：同一账号 Withdraw 后 5 秒内出现 Verify 即命中

        final boolean[] hit = {false};
        stmt.addListener((UpdateListener) (newEvents, oldEvents) -> {
            for (EventBean event : newEvents) {
                hit[0] = true;
                WithdrawEvent w = (WithdrawEvent) event.get("w");
                System.out.println("[命中] Withdraw 后 5 秒内收到 Verify: account=" + w.getAccount());
            }
        });

        // ---------- 场景1：A 在 W 后 3 秒到达（虚拟时间 0s -> 3s） → 命中 ----------
        epService.getEPRuntime().sendEvent(new CurrentTimeEvent(0));
        epService.getEPRuntime().sendEvent(new WithdrawEvent("acc-1"));
        epService.getEPRuntime().sendEvent(new CurrentTimeEvent(3_000));
        epService.getEPRuntime().sendEvent(new VerifyEvent("acc-1"));
        System.out.println("[场景1] 3 秒内验证 -> 命中=" + hit[0]);

        // ---------- 场景2：W 后 6 秒才验证 → 超时不命中 ----------
        final boolean[] hit2 = {false};
        stmt.removeAllListeners();
        stmt.addListener((UpdateListener) (newEvents, oldEvents) -> hit2[0] = true);
        epService.getEPRuntime().sendEvent(new CurrentTimeEvent(10_000));
        epService.getEPRuntime().sendEvent(new WithdrawEvent("acc-2"));
        epService.getEPRuntime().sendEvent(new CurrentTimeEvent(16_000)); // 推进 6 秒
        epService.getEPRuntime().sendEvent(new VerifyEvent("acc-2"));
        System.out.println("[场景2] 6 秒后验证 -> 命中=" + hit2[0] + "（超过 5 秒窗口）");

        epService.destroy();
    }
}

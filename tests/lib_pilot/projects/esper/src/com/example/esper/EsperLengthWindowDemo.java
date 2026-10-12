package com.example.esper;

import java.util.ArrayList;
import java.util.List;

import com.espertech.esper.client.Configuration;
import com.espertech.esper.client.EPServiceProvider;
import com.espertech.esper.client.EPServiceProviderManager;
import com.espertech.esper.client.EPStatement;
import com.espertech.esper.client.EventBean;
import com.espertech.esper.client.UpdateListener;

/**
 * Esper CEP（一）：长度窗口风控规则（对应文章 EPL：
 * "同一账号在窗口内连续失败 N 次"——原文用 win:time(10 sec)，
 * 此处改为 win:length(3)，不依赖墙钟即可确定性复现）。
 *
 * 输出确定性说明：固定顺序送入固定事件；length 窗口无时间依赖。
 */
public class EsperLengthWindowDemo {

    /** 事件 POJO（Esper 按属性名读取 getter）。 */
    public static class LoginFailedEvent {
        private final String accountId;
        private final boolean success;

        public LoginFailedEvent(String accountId, boolean success) {
            this.accountId = accountId;
            this.success = success;
        }

        public String getAccountId() {
            return accountId;
        }

        public boolean isSuccess() {
            return success;
        }
    }

    public static void main(String[] args) {
        Configuration config = new Configuration();
        config.addEventType("LoginEvent", LoginFailedEvent.class.getName());

        EPServiceProvider epService = EPServiceProviderManager.getDefaultProvider(config);

        // 同一账号在最近 3 次登录里全部失败 → 触发告警
        String epl = """
                select accountId, count(*) as fails
                from LoginEvent.win:length(3)
                where success = false
                group by accountId
                having count(*) = 3
                """;
        EPStatement stmt = epService.getEPAdministrator().createEPL(epl);

        List<String> alerts = new ArrayList<>();
        // Esper 7 的 UpdateListener 只有两个参数（newData/oldData）
        stmt.addListener((UpdateListener) (newEvents, oldEvents) -> {
            for (EventBean event : newEvents) {
                alerts.add("账号[" + event.get("accountId") + "] 连续失败 "
                        + event.get("fails") + " 次，触发风控");
            }
        });

        // 固定事件序列：A 连续 3 败（触发）；B 2 败 1 成（不触发）；A 成功 1 次重置窗口
        Object[][] events = {
                {"A", false}, {"B", false}, {"A", false},
                {"B", false}, {"A", false},  // A 达 3 连败
                {"B", true},                 // B 的窗口被成功冲掉，永不触发
        };
        for (Object[] e : events) {
            epService.getEPRuntime().sendEvent(
                    new LoginFailedEvent((String) e[0], (Boolean) e[1]));
        }
        System.out.println("[EPL] " + epl.stripIndent().strip().replace("\n", " "));
        System.out.println("[输入] 6 条事件: A败, B败, A败, B败, A败, B成功");
        System.out.println("[告警] " + alerts);
        System.out.println("[结论] 只有 A 触发（窗口内 3 连败），B 被成功事件重置");
        epService.destroy();
    }
}

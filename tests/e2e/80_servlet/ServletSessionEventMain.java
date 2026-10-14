
import java.util.EventListener;

import jakarta.servlet.http.HttpSessionBindingEvent;
import jakarta.servlet.http.HttpSessionBindingListener;
import jakarta.servlet.http.HttpSessionEvent;
import jakarta.servlet.http.HttpSessionListener;
import jakarta.servlet.http.HttpSession;

/**
 * servlet-api pilot C7：会话事件族（stub Session + 监听器触发，免容器）。
 */
public class ServletSessionEventMain {

    /** 最小 HttpSession stub（只实现本例访问的方法，其余抛 UOE）。 */
    static class StubSession implements HttpSession {
        final java.util.Map<String, Object> attrs = new java.util.LinkedHashMap<>();

        public void setAttribute(String name, Object value) {
            attrs.put(name, value);
        }

        public Object getAttribute(String name) {
            return attrs.get(name);
        }

        public java.util.Enumeration<String> getAttributeNames() {
            return java.util.Collections.enumeration(new java.util.TreeSet<>(attrs.keySet()));
        }

        public void removeAttribute(String name) {
            attrs.remove(name);
        }

        public long getCreationTime() {
            return 0;
        }

        public String getId() {
            return "stub-1";
        }

        public long getLastAccessedTime() {
            return 0;
        }

        public int getMaxInactiveInterval() {
            return 1800;
        }

        public void setMaxInactiveInterval(int i) {
        }

        public boolean isNew() {
            return false;
        }

        public void invalidate() {
        }

        public jakarta.servlet.ServletContext getServletContext() {
            throw new UnsupportedOperationException();
        }

        public HttpSessionContext getSessionContext() {
            throw new UnsupportedOperationException();
        }

        public Object getValue(String name) {
            return getAttribute(name);
        }

        public String[] getValueNames() {
            return attrs.keySet().toArray(new String[0]);
        }

        public void putValue(String name, Object value) {
            setAttribute(name, value);
        }

        public void removeValue(String name) {
            removeAttribute(name);
        }
    }

    interface HttpSessionContext {
    }

    /** 绑定感知对象：进出会话时收到回调。 */
    static class Bound implements HttpSessionBindingListener {
        final java.util.List<String> log;

        Bound(java.util.List<String> log) {
            this.log = log;
        }

        @Override
        public void valueBound(HttpSessionBindingEvent e) {
            log.add("bound:" + e.getName() + "@" + e.getSession().getId());
        }

        @Override
        public void valueUnbound(HttpSessionBindingEvent e) {
            log.add("unbound:" + e.getName());
        }
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        StubSession session = new StubSession();
        var log = new java.util.ArrayList<String>();

        // HttpSessionEvent：create/destroy 生命周期事件
        HttpSessionEvent lifeEvent = new HttpSessionEvent(session);
        var sessionListener = new HttpSessionListener() {
            @Override
            public void sessionCreated(HttpSessionEvent e) {
                log.add("created:" + e.getSession().getId());
            }

            @Override
            public void sessionDestroyed(HttpSessionEvent e) {
                log.add("destroyed:" + e.getSession().getId());
            }
        };
        sessionListener.sessionCreated(lifeEvent);
        sessionListener.sessionDestroyed(lifeEvent);

        // Binding：实现 listener 的对象进出会话自动回调（容器行为在 stub 上模拟协议）
        Bound bound = new Bound(log);
        session.setAttribute("user", bound);     // 容器在此触发 valueBound
        bound.valueBound(new HttpSessionBindingEvent(session, "user"));
        session.removeAttribute("user");
        bound.valueUnbound(new HttpSessionBindingEvent(session, "user"));

        check("event-log", "[created:stub-1, destroyed:stub-1, "
                        + "bound:user@stub-1, unbound:user]",
                log.toString()); // stub 不模拟容器回调，bound/unbound 为手工触发

        // 事件对象元数据
        HttpSessionBindingEvent evt = new HttpSessionBindingEvent(session, "k", "v");
        check("event-session", "stub-1", evt.getSession().getId());
        check("event-name", "k", evt.getName());
        check("event-value", "v", evt.getValue());
        check("event-source", "stub-1", ((HttpSession) evt.getSource()).getId());
        System.out.println("DONE ServletSessionEventMain");
    }
}

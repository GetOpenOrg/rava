public class HigherOrderFunctions {

   public HigherOrderFunctions() {
       first(new AnEventOrCallback() {
           public void call() {
               second();
           }
       });
   }

   public void first(AnEventOrCallback obj) {
       obj.call();
   }

   public void second() {
       System.out.println("Second");
   }

   public static void main(String[] args) {
       new HigherOrderFunctions();
   }
}

interface AnEventOrCallback {
   public void call();
}

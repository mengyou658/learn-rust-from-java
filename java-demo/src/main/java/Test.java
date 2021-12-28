import cn.hutool.core.collection.CollectionUtil;
import cn.hutool.core.math.MathUtil;
import cn.hutool.core.util.ArrayUtil;
import com.alibaba.fastjson.JSON;
import org.apache.lucene.util.RamUsageEstimator;

import java.lang.instrument.Instrumentation;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.nio.CharBuffer;
import java.nio.charset.StandardCharsets;
import java.util.*;

public class Test {
    public static void main(String[] args) {
        int x = 16;
//        int x = 16;
        x = 16;
        x += 1;
        System.out.println("Hello, world!: " + x);
        Test.sayHello();
    }

    public static void sayHello() {
        System.out.println("Hello");
        boolean bool = false;
        // 1 字节
        byte i8_ = (byte) 0;
        byte u8_ = (byte) 0 & 0xFF;
        // 2 字节
        short i16_ = 0;
        short u16_ = 0 & 0xFFFF;
        // 4 字节
        int i32_ = 0;
        int u32_ = 0 & 0xFFFFFFFF;
        // 8 字节
        long i64_ = 0;
        long u64_ = 0 & 0xFFFFFFFFFFFFFFFFL;
        // 16 字节
        // 没有这个标识，BigInteger可以存取任意大小的数值
        // usize Java没有

        // 进制
        // 2 进制
        int jinzhi_2 = 0b1111;
        // 8进制
        int jinzhi_8 = 0144;
        // 16进制
        int jinzhi_16 = 0x133;
        // 字节
        byte jinzhi_byte = 'a';
        char jinzhi_char = 'a';

        // 浮点
        float f_32 = 2.0F;
        double f_64 = 2.0D;
        assert 0.1 + 0.2 == 0.3;

        // 字符
        // utf8 1-4字节
        jinzhi_char = '中';

        String s = new String(new char[]{jinzhi_char});
        System.out.println(Arrays.toString(s.getBytes(StandardCharsets.UTF_8)));
        System.out.println(String.format("字符'中'占用了%s字节的内存大小", RamUsageEstimator.shallowSizeOf(s) / 8));

        // java 没有 元组

        // 字符串
        String 字符切片 = "str";
        StringBuilder 字符串 = new StringBuilder("str");

        // 数组
        int[] 数组 = new int[10];
        int[] 数组1 = new int[]{1, 2, 3, 4, 5};
        // 动态数组Vector
        List<String> 集合 = new ArrayList<>();
        List<Integer> 集合1 = Arrays.asList(1,2,3,4);
        // set
        Set<String> set = new HashSet<>();
        // map
        Map<String, String> map = new HashMap<>();

    }

    int add_with_extra(int x, int y) {
        x += 1;
        y += 5;
        return x + y;
    }

    class User {
        public boolean active;
        public String username;
        public String email;
        public Long sign_in_count;

        User testNewUser() {
            User user = new User();
            user.active = true;
            user.sign_in_count = 1L;
            user.email = "someone@example.com";
            user.username = "someusername123";
            return user;
        }
    }

    // 枚举
    enum PokerSuit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
    }

    // 数组
    void java_array() {
        int[] a = {1, 2, 3, 4, 5};
        int[] b = new int[5];
        Arrays.fill(b, 3);
    }

    // 流程控制
    void test_control() {
        boolean condition = true;
        int number = 0;
        if (condition) {
            number = 5;
        } else {
            number = 6;
        }
        System.out.println("The value of number is: " + number);
        for (int i = 0; i < 5; i++) {
            System.out.println(i);
        }
        int[] a = {4, 3, 2, 1};
        for (int i = 0; i < a.length; i++) {
            System.out.println(String.format("第%d个元素是%d", i + 1, a[i]));
        }
        PokerSuit dire = PokerSuit.Clubs;
        switch (dire) {
            case Clubs:
                System.out.println("Clubs");
                break;
            case Spades:
            case Hearts:
                System.out.println("Spades or Hearts");
                break;
            default:
                System.out.println("Diamonds");
                break;
        }
    }
}

class Circle<M> {
    public double x;
    public double y;
    public double radius;

    public Circle(double x, double y, double radius) {
        this.x = x;
        this.y = y;
        this.radius = radius;
    }

    public double area() {
        return Math.PI * (this.radius * this.radius);
    }

    public <T extends Number> BigDecimal add(T a, T b) {
        return BigDecimal.valueOf(a.doubleValue()).add(BigDecimal.valueOf(b.doubleValue()));
    }
}

// interface
interface Summary {
    String summarize();
}

class Post implements Summary {

    private String title;
    private String author;
    private String content;

    @Override
    public String summarize() {
        return String.format("文章%s, 作者是%s", this.title, this.author);
    }

    public void test_panic() {
        throw new RuntimeException("crash and burn");
    }
}
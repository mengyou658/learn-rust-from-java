import com.alibaba.fastjson.JSON;
import org.apache.lucene.util.RamUsageEstimator;

import java.lang.instrument.Instrumentation;
import java.math.BigInteger;
import java.nio.CharBuffer;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

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
        // utf8 2-4字节
        jinzhi_char = '中';

        String s = new String(new char[]{jinzhi_char});
        System.out.println(Arrays.toString(s.getBytes(StandardCharsets.UTF_8)));
        System.out.println(String.format("字符'中'占用了%s字节的内存大小", RamUsageEstimator.shallowSizeOf(s) / 8));

        // java 没有 元组

        // 字符串
        String 字符切片 = "str";
        StringBuilder 字符串 = new StringBuilder("str");

        // 数组
        List<String> 数组 = new ArrayList<>();

    }

    int add_with_extra(int x, int y) {
        x += 1;
        y += 5;
        return x + y;
    }
}

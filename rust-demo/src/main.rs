fn main() {
    let x: usize = 16;
    let x = 16_usize;
    let mut x = 16_usize;
    x += 1;
    println!("Hello, world!: {}", x);
    say_hello();
}

fn say_hello() {
    println!("Hello");
    let bool_: bool = false;
    // 1 字节
    let i8_: i8 = 0;
    let u8_: u8 = 0;
    // 2 字节
    let i16_: i16 = 0;
    let u16_: u16 = 0;
    // 4 字节
    let i32_: i32 = 0;
    let u32_: u32 = 0;
    // 8 字节
    let i64_: i64 = 0;
    let u64_: u64 = 0;
    // 16 字节
    let i128_: i128 = 0;
    let u128_: u128 = 0;
    // usize根据机器不同
    // 32位机器
    let usize_32: usize = 0;
    // 64位机器
    let usize_64: usize = 0;

    // 进制
    // 2 进制
    let jinzhi_2 = 0b1111;
    // 8进制
    let jinzhi_8 = 0o133;
    // 16进制
    let jinzhi_8 = 0x133;
    // 字节
    let jinzhi_byte = b'a';
    // char
    let jinzhi_byte = 'a';
    let jinzhi_char = b"abc";

    // 浮点
    let f_32: f32 = 2.0;
    let f_64 = 2.0;
    let f_64: f64 = 2.0;
    // 断言0.1 + 0.2与0.3相等
    // 下面执行会失败 具体解释参考 https://course.rs/basic/base-type/numbers.html
    // assert!(0.1 + 0.2 == 0.3);
    assert!((0.1f64 + 0.2f64 - 0.3f64).abs() < 0.00001f64);


    // 字符
    // unicode 4字节
    let jinzhi_char = '中';
    println!("{:?}", jinzhi_char.to_string().as_bytes());
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&jinzhi_char));
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&jinzhi_char.to_string()) / 8);

    // 元组
    let tuple = (1, 2, "string");

    // 字符串
    let 字符切片 = "str";
    let 字符串 = String::from("str");

    // 数组
    let 数组: Vec<f64> = Vec::new();

    let y = {
        let x = 3;
        x + 1
    };


}

// rust 规定 蛇形命名法(snake case)
fn add_with_extra(mut x: i32, mut y: i32) -> i32 {
    x += 1; // 语句
    y += 5; // 语句
    x + y // 表达式 没有冒号默认最后的表达式是返回值
}

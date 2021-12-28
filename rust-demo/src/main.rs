use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::net::{AddrParseError, IpAddr, SocketAddr};

fn main() {
    let x: usize = 16;
    let x = 16_usize;
    let mut x = 16_usize;
    x += 1;
    println!("Hello, world!: {}", x);
    say_hello();

    testNewUser();

    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);

    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);

    let a : i32 = 10;
    println!("{}", a.summarize());
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
    // 固定不可变
    let 字符切片 = "str";
    // 不固定，可以增删数据
    let 字符串 = String::from("str");

    // 数组
    let 数组 = [0;10];
    let 数组 = [1,2,3,4,5];
    // 动态数组Vector
    let 集合:Vec<i32> = Vec::new();
    let 集合 = Vec::<f64>::new();
    let 集合= vec![1,2,3,4];
    // set
    let set = HashSet::<String>::new();
    // map
    let map = HashMap::<String, String>::new();

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
    // return x + y;
}

fn tuple(s: String) -> (String, usize) {
    let tup = (500, 6.4, "str");
    let (x, y, z) = tup;
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("The value of y is: {}", y);
    (s, x as usize)
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn testNewUser() -> User {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // let user2 = User {
    //     email: String::from("test@test.com"),
    //     ..user1
    // };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{}", user1.active);
    // println!("{}", user1.username); // 报错 move 了 无法再访问
    // println!("{:?}", user1); // 报错 partially moved here

    user2
}
// 元组结构体，结构体字段可以没有名字
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

// 枚举
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn print_suit(card: PokerSuit) {
    println!("{:?}",card);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 简单的Option定义
enum OptionDef<T> {
    Some(T),
    None,
}
// option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
// 数组
fn rust_array() {
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5];
}

// 流程控制
fn test_control() {
    let condition = true;
    // if 可以有返回值
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
    for i in 1..=5 {
        println!("{}",i);
    }
    let a = [4,3,2,1];
    // `.iter()`方法把`a`数组变成一个迭代器
    for (i,v) in a.iter().enumerate() {
        println!("第{}个元素是{}",i+1,v);
    }
    let dire = PokerSuit::Clubs;
    match dire {
        PokerSuit::Clubs => println!("Clubs"),
        PokerSuit::Spades | PokerSuit::Hearts => {
            println!("Spades or Hearts");
        },
        _ => println!("Diamonds"),
    };
}


struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
        a + b
    }
}

// trait
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}
impl Post {
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        10
    }
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}
// 这里Java没有这个功能 scala有这个功能
impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("这是一个测试{}", &self)
    }
}
// 错误处理
fn test_panic() {
    panic!("crash and burn");
}
fn test_result() -> Result<SocketAddr, AddrParseError> {
    "127.0.0.1".parse::<SocketAddr>()
}
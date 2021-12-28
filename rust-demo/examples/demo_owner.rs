fn main() {
    {                            // s 在这里无效, 它尚未声明
        let s = "hello";   // 从此处起，s 是有效的
        let x = s;         // 使用浅拷贝Copy（发生在栈上）
        println!("{} {}", s, x); // 使用 s
    }                            // 此作用域已结束，s 不再有效

    {                            // s 在这里无效, 它尚未声明
        let s = String::from("hello");   // 从此处起，s 是有效的
        let x = s;         // 转移所有权
        // println!("{} {}", s, x); // 这里s 被move到x了，s不再有效，不能使用
    }                            // 此作用域已结束，s 不再有效

    {                            // s 在这里无效, 它尚未声明
        let s = String::from("hello");   // 从此处起，s 是有效的
        let x = s.clone();         // 使用深拷贝Copy
        println!("{} {}", s, x); // 使用 s
    }                            // 此作用域已结束，s 不再有效

    {
        let mut x = 5;
        let y = &x;
        let mut z = &mut x; // 可变借用只能有一个，且有了可变借用后，不能在使用不可变借用
        // let mut z1 = &mut x; // 第二个可变借用
        // assert_eq!(5, x); // 可变借用只能有一个
        // assert_eq!(5, *y); // 有了可变借用后，不能在使用不可变借用
        assert_eq!(5, *z);
        // assert_eq!(5, *z1);
    }

    {
        let mut x = 5;
        let y = &x;
        {
            let mut z1 = &mut x; // 可变借用超过大括号就不存在了
        }
        let mut z = &mut x; // 可变借用只能有一个，且有了可变借用后，不能在使用不可变借用
        // assert_eq!(5, x); // 可变借用只能有一个
        // assert_eq!(5, *y); // 有了可变借用后，不能在使用不可变借用
        assert_eq!(5, *z);
        // assert_eq!(5, *z1);
    }
}

fn test(x : &mut i32) {
    println!()
}
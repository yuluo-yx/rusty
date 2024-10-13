fn main() {

    let mut s1 = String::from("hello");
    
    // 字符串拼接
    s1.push_str(", world");

    println!("{}", s1);

    // s1 所有权转移 s2
    let s2 = s1;

    // 下述语句报错
    // 因为 s1 的所有权已经转移给了 s2 ，rust 禁止使用无效引用
    // println!("{}", s1);

    // 在 rust 中，所有权转移时，拷贝指针，长度和容量而不拷贝数据听起来想
    // 浅拷贝，但是因为 rust 同时使得 s1 也无效了。因此此操作称为移动 remove，
    // 而不是浅拷贝。上述代码理解为 s1 被移动到了 s2
    // 这样就解决了 s1 不在指向任何数据，只有 s2 是有效的，当 s2 离开
    // 内存，就会立即释放内存。为此，rust 称呼 let a = b 为变量绑定
    println!("{}", s2);

    // 下述代码仅仅是将 str 变量的引用存储在了 x 中，并没有持有所有权
    // 之后将 x 赋值给 y，也只是复制了引用地址。
    let x: &str = "hello, world";
    let y = x;
    println!("{}, {}", x, y);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x = {}, 引用 y = {}", x, y);

    let s3 = String::from("hello, world!");
    println!("{}", s3);
    let len = calcuate_length(&s3);
    println!("The length of '{}' is {}.", s3, len);

    // 通过函数修改 s4 的值
    let mut s4 = String::from("test");
    // 创建一个可变的引用 &mut s4
    change_string(&mut s4);
    println!("s4 is '{}'", s4)
}

/**
 * 无需像上章一样：先通过函数参数传入所有权，然后再通过函数返回来传出所有权，代码更加简洁
 * calculate_length 的参数 s 类型从 String 变为 &String
 * 
 * 这里的 & 为引用，允许使用值，而不是获取所有权。
 */
fn calcuate_length(str: &String) -> usize {

    str.len()
}

/**
 * 即通过传入 &mut 来将引用变得可变
 * 
 */
fn change_string(str: &mut String) {
    str.push_str(", change");
}

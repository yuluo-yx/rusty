fn main() {

    // 字符串
    string()

}

fn string() -> () {
    let name = "yuluo";
    greet(name);

    // 切片 slice
    // [开始索引..结束索引]
    println!("slice 1: {}", &name[1..2]);

    let chinese = "你好，牧生";
    // 汉字在 UTF-8 编码中占三个字节，因此在 [0..2]/[..2] 时代码崩溃
    println!("汉字截取：{}", &chinese[0..3]);

    // 字符串字面量类型 -> &str
    let _s = "yuluo";
    // 等同于
    let _s :&str = "yuluo";

    // 对象形式创建
    let _s1 = String::from("yuluo");

    // rust 在语言层面只有一个字符串类型 str 在标准库中提供了多个实现
    // str 不可修改，没有所有权 String 是一个可改变有所有权的 UTF8 编码字符串
    // String, OsString, CsString CsStr ...

    // &str -> string
    let _s2 = String::from("yuluo");
    let _s3 = "yuluo".to_string();

    // String -> &str 通过解引用
    let _s4 = &String::from("yuluo");

    // 字符串索引
    let _s5 = "yuluo";
    // 报错：let s5_idx = s5[0];
    // 因为 rust 底层是一个字节数组且是 UTF8(u8) 类型，对于汉字来说，占三个字节 [0] 取不到任何东西
    // 所以 rust 提供了不同的 String 展现形式。rust 中 String 类型取值时无法保证复杂度时 O(1)

    string_oper()
}

// 字符串操作
fn string_oper() -> () {

    let mut s = "hello, ".to_string();

    // 追加字符串
    s.push_str("world");

    // 追加字符
    s.push('!');

    println!("追加后的字符串：{}", s);

    // 插入
    s.insert_str(s.len(), " I Like rust");
    s.insert(s.len(), '!');
    println!("插入 -> {}", s);

    // 替换
    // 返回新的字符串
    s = s.replace("rust", "RUST");
    println!("替换 -> {}", s);
    // String 和 &str 类型 replacen(target, new, count)
    // 只 String 类型 replace_range([开始索引..结束索引], new)

    // 删除 仅适用于 String 类型
    // pop() remove() truncate() clear()
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误 不是合法的字符边界，汉字占三个
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    // clear 相当于 truncate 参数为 0 时
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // 连接 使用 + 或者 += 字符串 要求右边的参数必须为字符串的切片引用（Slice）类型
    let s_concatenate = String::from("hello, ");
    let res = s_concatenate + "rust";
    let mut res = res + "  !!!";
    res += " - 123";
    println!("连接 -> {}", res);

    // format! 连接
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);

    println!();

    // 字符串转义
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 操作 UTF8 字符串
    println!("中国人字符遍历");
    for c in "中国人".chars() {
        print!("{} \n", c);
    }

    println!("中国人字节：");
    for b in "中国人".bytes() {
        print!("{} ", b);
    }
}

// (name: String) 报错
// 这里要使用字符串的切片类型
// (name: &str) 正确
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

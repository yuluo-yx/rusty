fn main() {

    // rust 函数
    println!("{}", plus_six(6));

    // 在 main 函数中看作为 main 函数的返回表达式，可以不用加 ；
    // 加入 ；返回值就是默认单元类型 ()
    println!("{}", plus_or_minus(6));

    dead_end()
}

// rust 中函数使用最后一条表达式来返回一个值，也可以使用 return 提前返回
fn plus_six(x: i32) -> i32 {

    // 表达式，加 ；号成为语句
    x + 6
}

// 使用 return 返回
fn plus_or_minus(x: i32) -> i32 {

    if x < 5 {
        return x - 5
    }

    x + 5
}

// 永不返回的发散函数 diverge function
fn dead_end() -> ! {

    panic!("到了穷途末路，此函数发散，不返回")
}

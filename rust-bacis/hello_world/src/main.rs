// fn main() {
//     println!("Hello, world!");
// }

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];

    // rust 不能自动遍历集合类型，需要转换为 迭代器 类型，
    // 后面 for 会隐式转换
    for region in regions.iter() {
        // ! （宏 操作符） 理解成一种特殊类型的函数
        // {} 编译器自动推导类型
        println!("{}", &region);
    }

}

fn main() {
    greet_world();
}

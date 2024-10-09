fn main() {
    // let guess = "42".parse().expect("Not a number");

    // java 不支持运算符重载
    println("数值类型");

    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    // 下述代码会崩溃
    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // rust 对未定义的数学行为统一返回 NaN （not a number）
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为");
    }

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);

    println("位运算");
    // 运算符	说明
    // & 位与	相同位置均为1时则为1，否则为0
    // | 位或	相同位置只要有1时则为1，否则为0
    // ^ 异或	相同位置不相同则为1，相同则为0
    // ! 位非	把位中的0和1相互取反，即0置为1，1置为0
    // << 左移	所有位向左移动指定位数，右位补0
    // >> 右移	所有位向右移动指定位数，带符号移动（正数补0，负数补1）
    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);

    println("序列类型");
    // rust 提供的生成连续数组的方式
    for i in 1..=10 {
        print!("{} ", i);
    }
    println!();
    for i in 'a'..='e' {
        print!("{} ", i);
    }

    println("字符类型");
    // rust 支持 Unicode 编码
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("{}, {}, {}, {}", &c, &z, &g, &heart_eyed_cat);

    let x = '中';
    println!("字符'中'占用了 {} 字节的内存大小",std::mem::size_of_val(&x));

    println("单元类型")
    // 单元类型就是 () fn main() 的返回值就是 单元类型
    // 没有返回值的函数叫发散函数

}

fn println(input: &str) {
    println!("分割线输出：============= {} ===============", &input);
}

const MAX_POINTS: u32 = 10;

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("the value of x is {}", x);
    // 这个时候直接将x赋值为6会报错
    // 声明变量的时候直接声明的是不可变的类型
    x = 6;
    println!("{}", x);
    println!("{}", MAX_POINTS);

    let _m = 13;
    let _m = 14;
    let m = 18;
    println!("{}", m);

    let spaces = "  ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // 将"42"解析成某个数字类型，这里需要指明
    let aa: u32 = "42".parse().expect("Not a number");
    println!("{}", aa);

    let mm = 2.0;
    println!("{}", mm);

    let t = true;
    println!("{}", t);

    let emo = '🙁';
    println!("{}", emo)
}

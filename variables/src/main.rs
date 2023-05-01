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
    println!("{}", m)
}

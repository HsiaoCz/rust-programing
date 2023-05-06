fn main() {
    let mut s = String::from("Hello");
    s.push_str(",World");
    println!("{}", s);

    let s1 = String::from("hello");
    // let s2 = s1;
    // 这里会报错，s1已经被释放了
    println!("{}", s1);
    // 使用clone可以
    let s2 = s1.clone();
    println!("{},{}", s1, s2);
}

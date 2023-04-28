// io在标准库里面 std表示标准库
// rust会默认导入prelude这个模块，如果需要使用得模块不在这个模块里，需要显式地导入
// use ::表示导入
use std::io;

fn main() {
    println!("Hello, world!");

    println!("猜测一个数");

    // let 声明变量

    let foo = 1;
    let bar = foo; // 所有的变量都是不可变的

    // 想要使变量可以改变需要在声明的时候加上mut
    let mut hello = 1;
    hello = 2;

    // 声明一个可变的变量
    // String::new()会返回字符串的一个新的实例
    // String由标准库提供，utf8编码,String::new()表明new是string的关联函数
    // 关联函数就是针对类型本身实现，而不是具体的对象实现
    // 类似java中的静态方法
    // new函数会创建一个空白的字符串

    let mut guess = String::new();

    // stdin()这个函数会返回一个实例，Stdin，它是一个句柄，用来处理终端中的标准输入
    // read_line()获取用户的输入，将用户的输入，赋值给一个空的字符串的变量
    // 这个字符串需要时可变的，所以前面有一个mut修饰
    // &取地址符号，含义和go的取地址有点相似
    // 引用默认是不可变的
    // read_line()返回一个枚举类型的返回值
    // io::Result Ok,Err oK表示操作成功，Err表示失败
    // expect()假如io.Result返回的值为Err 那么expect()这个方法会中断当前的程序
    // 并将传入的字符串信息显示出来，但是如果这个io.Result返回的是OK,expect这个方法会将ok中返回的内容作为结果返回给用户
    io::stdin().read_line(&mut guess).expect("无法读取");
    // {}是一个占位符，多个占位符会按顺序展开
    println!("你猜测得数是: {}", guess)
}

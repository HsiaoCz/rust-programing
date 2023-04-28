# Rust

so how to use Rust

rust:

- 安全

- 无需GC

- 易于维护，调试，代码比较高效

rust特别擅长的领域

- 高性能web Service
- WebAssembly
- 命令行工具
- 网络编程
- 嵌入式设备
- 系统编程

rust的缺点：难学

rustc --version 看看是不是安装好了
rustup doc


rust的编译命令:

rustc main.rs

## 1、hello World

```rust
// fn 定义函数，没有返回值。没有参数
// main(){} rust可执行程序最先运行的代码
// rust的缩进是四个空格，而不是tab
// println!是rust的宏(macro)
// 如果是函数就没有!
// Hello World是字符串，是println!的参数
// 代码以;号结尾，但是不要似乎也行
fn main() {
    println!("Hello World!")
}
```

这里注意还有一个感叹号


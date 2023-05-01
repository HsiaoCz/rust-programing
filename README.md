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

- 编译和运行是单独的两步，rust程序在运行之前，必须先编译
命令为rustc源文件名,rustc main.rs

- 编译成功后会生成一个二进制文件，windowns下会额外生成一个.pdb文件，里面包含调试信息

- rust是ahead-of-time编译的语言

- rustc只适合简单的rust程序


## 2、cargo

cargo是rust的构建系统和包管理工具

使用cargo new hello_cargo创建项目

- 项目名称为hello_cargo
- 会创建一个新的目录hello_cargo
  - Cargo.toml 
  - src 这里是源代码
    - main.rs 项目的入口
  - 初始化了一个新的Git仓库  .gitignore
   - 可以使用其他的VCS或不使用VCS:cargo new 的时候使用 --vcs这个flag

cargo.toml

```rust
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- 是cargo的配置文件格式
- [package]，是一个区域标题，表示下方内容是用来配置包的
  - name 项目名
  - version 项目版本
  - authors 项目作者
  - edition 使用的rust版本

- [dependencies]，另一个区域的开始，它会列出项目的依赖项

- 在rust里面，代码的包称为crate


src/main.rs

- cargo 生成的main.rs在src目录下
- 而Cargo.toml在项目目录下
- 顶层目录可以放置:README、许可信息、配置文件和其他与程序源码无关的文件
- 如果创建项目时没有使用cargo，也可以把项目转化为使用cargo
  - 把源代码文件移动到src下
  - 创建cargo.toml并填写相应的配置

cargo build构建项目

- cargo build

  - 创建可执行文件
  - 运行可执行文件

- 第一次运行cargo build会在顶层目录下生成cargo.lock文件
  - 该文件负责追踪项目依赖的精确版本
  - 不需要手动修改该文件

cargo run 

构建并且运行项目

- cargo run 编译代码+执行结果
  - 如果之前编译成功过，并且源代码没有改变，那么就会直接运行二进制文件

cargo check

- cargo check 要比cargo build快得多

cargo build --release

为发布构建

- 编译时会进行优化
  - 代码会运行的更快，但是编译时间更长
  - 会在target/release而不是target/debug生成可执行文件

cargo update

可以升级依赖
在cargo.toml里修改依赖

## 3、变量与可变性

声明变量使用let关键字
默认情况下，变量是不可变的(lmmutable)


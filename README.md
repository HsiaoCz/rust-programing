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
声明变量时，在变量前面加上mut，就可以使变量可变

### 3.1、变量与常量

常量在绑定值以后也是不可变的，但是它与不可变得变量有许多区别
- 不可以使用mut，常量永远都是不可变的
- 声明常量使用const关键字，它的类型必须被标注
- 常量可以在任何作用域内进行声明，包括全局作用域
- 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能运行时才能计算出的值

在程序运行期间，常量在其声明的作用域内一直有效
命名规范：rust里常量使用全大写字母，每个单词之间用下划线分开，例如：-MAX_POINT

### 3.2、Shadowing隐藏

可以使用相同的名字声明新的变量，新的变量就会shadow之前声明的同名变量
- 在后续的代码中这个变量名代表的就是新的变量

shadow和把变量标记为mut是不一样的

- 如果不使用let关键字，那么重新给非mut的变量赋值会导致编译时错误
- 而使用let声明的同名新变量，也是不可变的
- 使用let声明的同名新变量可以和原来的类型不同

### 3.3、复合类型

- 复合类型可以将多个值放在一个类型里面
- rust提供了两种基础的复合类型，元组tuple,数组

**tuple**

- tuple可以将多个类型的多个值放在一个类型里
- tuple的长度是固定的，一旦声明就无法改变

**创建tuple**

- 在小括号里，将值用逗号隔开
- tuple中的每个位置都对应一个类型,tuple中各元素的类型不必相同

**获取tuple的元素值**

- 可以使用模式匹配来解构(destructure)一个Tuple来获取元素的值

**访问tuple的元素值**

- 使用.标记法

**数组**

- 数组也可以将多个值放在一个类型里面
- 数组中的每个元素类型必须相同
- 数组的长度是固定的

**声明数组**

- 在中括号中，用逗号分开

**数组的用处**

- 如果想让你的数据存放在stack(栈)上而不是heap(堆)上，或者想保证有固定数量的元素，这时使用数组更有好处

- 数组没有vector灵活
  - vector和数组类似，它由标准库提供
  - vector的长度是可以改变的

**数组的类型**

- 数组的类型以这种形式表示：[类型：长度]
  - 例如：let a:[i32:5]=[1,2,3,4,5];
 
**另一种声明数组的方式**

- 如果数组的每个元素值都相同，那么可以在：
  - 在中括号里指定初始值
  - 然后是一个";"
  - 后边跟着数组的长度
例如：let a=[3;5];它就相当于;let a=[3,3,3,3,3];

**访问数组的元素**

- 数组是在stack上分配的单个块的内存
- 可以使用索引来访问数组的元素
- 如果访问的索引超出了数组的范围，那么：
  - 编译会通过
  - 运行会报错
    - rust不允许其继续访问相应地址的内存

### 3.4、函数

- 声明函数使用fn关键字
- 依照惯例，针对函数和变量名，rust使用snake case命名规范：
  - 所有字母都是小写的，单词之间使用下划线分开

**函数的参数**

- paramenters,arguments
- 函数的签名里，必须声明每个参数的类型

**函数体中的语句和表达式**

- 函数由一系列语句组成，可选的由一个表达式结束
- Rust是一个基于表达式的语言
- 语句是执行一些动作的指令
- 表达式会计算产生一个值
- 函数的定义也是语句
- 语句不返回值，所以不可以使用let将一个语句赋给一个变量

**函数的返回值**

- 在->符号后边声明函数返回值的类型，但是不可以为返回值命名
- 在rust里面，返回值就是函数体里面最后一个表达式的值
- 若想提前返回，需要使用return关键字，并指定一个值
  - 大多数函数都是默认使用最后一个表达式作为返回值

### 3.5、控制流

- if 表达式允许您根据条件来执行不同的代码分支
- 条件必须式bool类型
- if 表达式中，与条件相关联的代码块就叫做分支
- 可选的，在后边可以加上一个else表达式

**使用else if处理多重条件**

- 如果使用了多余一个else if，那么最好使用match来重构代码

**在let语句中使用if**

- 因为if是一个表达式，所以可以将它放在let语句中等号的右边


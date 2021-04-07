# Rust Note

## cargo

- cargo build [--release] 编译 可以使用--release来优化编译【生产模式】
- cargo run 编译并运行
- carge check 校验是否可以正常编译
- carge doc --open 打开所有依赖的文档

## 变量

rust中默认使用let声明变量，且rust中默认变量不可变，如果需要可变，则使用mut关键字
```rust
let foo = 5;
let mut foo = 5;
```

## 数据类型

标量：代表一个单独值：整形，浮点型，布尔型，字符类型

- 整形：默认为 i32
    长度	有符号	无符号
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    arch	isize	usize   // 依赖于平台，32位系统为32位，64系统为64位
    
    如果发生整形溢出, 如：`let t: u8 = 256`在debug模式会发生panic，如果在其他模式下不会检测移除，而进行二进制补码包装，将256 -> 0
- 浮点型：
    分为f32 / f64: 单精度和双精度浮点，默认为f64
- 布尔型：
    bool: true / false
- 字符类型：
    char: 'a'
    大小：四个字节，代表一个Unicode标量值

复合类型：
    元组，数组
    元组是一个将多个其他类型符合的方式
```rust
fn main() {
    let tup: (i32, f64, char) = (500, 10.1,'a');
    // let (x,y,z) = tup;
    let x = x.0
    let y = x.1
    let z = x.2
}
```
    数组：
    特点：长度不可以，数据类型需要一致
```rust
    let months = ["January", "February"];
    let a: [i32, 5] = [1,2,3,4,5]
    let b: [0, 5]; // [0,0,0,0,0]
```

## 控制流

- if

可以控制赋值, 但是分支返回类型需要相同
```rust
fn main() {
    let codition = true;
    let num = if codition {
        5
    }else{
        // ‘6’ // error[E0308]: if and else have incompatible types
        6
    };
    println!("number is {}", num);
}
```
- 循环
loop: 无线循环，且可以支持返回值
```rust
loop{
    // do...
}

let mut count = 0;
let result = loop {
    count += 1;
    if count == 10 {
        break count;
    }
};
println!("result: {}", result); // 10
```
while: 条件循环（普通）

for循环遍历

```rust
fn main() {
    let arr = [1,2,3,4];
    for el in arr.iter() {
        println!("current is {}", el);
    }

    for el in (1..4).rev() { // (1..4) => [1,2,3]
        println!("remaining: {}", el);
    }
}
```

## 函数

```rust
fn main() {
    let result = sum_them(1, 2);
    println!("result：{}", result);
}

fn sum_them(a: i32, b: i32) -> i32 {
    a + b
}
```


## 所有权（ownership）模型

### 规则

- Rust 中每一个值都有一个被重围所有者的变量
- 值再任意时刻，只有一个所有者
- 所有者离开作用域，值将被废弃

问题：

如果变量离开作用域就释放，对于堆内存，垃圾回收可能出现两次？

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);
```
如上，离开作用域，s1引用的堆会被二次释放，造成隐式的安全问题。
所以这里Rust使用了变量的移动，即，当一个引用被另一个引用引用时候，会发生变量的移动。
上面的代码s1变量将被废弃，无法使用。
```rust
// ^^ value used here after move
```
那么如果确实需要使用深拷贝，则可以通过通用函数clone
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1:{}, s2:{}", s1, s2);
```
所有权模型demo：
```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    let (s2, len) = takes_ownership(s);     // s 的值移动到函数里 ...
                                            // ... 所以到这里不再有效
                                            // 函数将返回值移动给s2,len，通过元组可以返回多个值

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
    let len = some_string.len()
    (some_string,len)
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

还有问题，如果一个复杂类型被传入函数，则移交所有权，若原上下文仍然需要使用此变量应该怎么办呢？
Rust引入了引用的概念：
```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生
let mut s2 = String::from("hello");
let len = calculate_length2(&s2);
fn calculate_length2(s: &mut String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生
```

### 悬垂引用（Dangling References）
简单来说，就是指针的内存被错误分配给其他持有者，即此指针指向内存可能已经释放，但是仍然被返回所有权。
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    s
    // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！

// 正确写法
fn no_dangle() -> String {
   let s = String::from("hello");
   s // 直接移交String所有权
}


```







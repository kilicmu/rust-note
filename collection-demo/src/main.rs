use std::collections::HashMap;
fn main() {
    // 创建vector
    // 1. 构造函数创建
    //let v: Vec<i32> = Vec::new();
    // 2. 宏创建
    // let v = vec![1,2,3];

    // 基本操作
    // 1) push
    // let mut v = vec![];
    // v.push(1);

    // let first: &i32 = &v[0];
    // println!("first value: {}", first);

    // match v.get(1) {
    //     Some(first) => println!("first value by get: {}", first),
    //     None => println!("none element")
    // }

    // 2) 遍历
    // let mut v = vec![1, 2, 3];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", i);
    // }
    // println!("{:#?}", v);

    // 3） 不同元素存储
    // #[derive(Debug)]
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String)
    // }

    // let v = vec![
    //     SpreadsheetCell::Int(10),
    //     SpreadsheetCell::Float(19.9),
    //     SpreadsheetCell::Text(String::from("hello"))
    // ];

    // for i in &v {
    //     // match i {
    //     //     SpreadsheetCell::Int(i) => println!("{}", i),
    //     //     SpreadsheetCell::Float(f) => println!("{}", f),
    //     //     SpreadsheetCell::Text(t) => println!("{}", t)
    //     // }
    //     if let SpreadsheetCell::Text(t) = i {
    //         println!("{}", t);
    //     }else if let SpreadsheetCell::Int(i) = i {
    //         println!("int: {}", i);
    //     }
    // }

    // 字符串、
    // 1. 字符串更新
    // let mut s1 = String::from("hello");
    // let s2 = "world";
    // s1.push(' ');    //  向s1后面添加字符
    // s1.push_str(s2); // 拼接s2 -> s1 后面，并让出s2所有权
    // println!("{}", s1);

    // 2. 字符串的格式化
    // let s1 = String::from("hello");
    // let s2 = String::from("herin");

    // // let s3 = s1 + " " + &s2; // s1让出所有权，s2未让出所有权
    // // println!("{}", s1); // ======> error
    // println!("{}", s2);
    // // println!("{}", s3);
    // let s4 = format!("{}{}{}", &s1, " ", &s2);
    // println!("{}", s4);

    // 3. 注意：不支持索引访问
    // 原理：使用Vec<u8> 存储UTF-8，可能造成访问错误预期结果问题，比如特殊字符可能占用两个u8存储，使用索引会非预期

    // 4. 遍历
    // for c in "नमस्ते".chars() { // 遍历Unicode标量
    //     println!("{}", c);
    // }

    // for b in "नमस्ते".bytes() { // 遍历原始字节
    //     println!("{}", b);
    // }


    // map 哈希表
    // 1. 定义
    // 直接定义：
    // let mut map: HashMap<&str, i32> = HashMap::new();
    // collect构建：
    // let people_names = vec![String::from("herin"), String::from("kilic")];
    // let scores = vec![11, 22];
    // let mut map: HashMap<_, _> = people_names.iter().zip(scores.iter()).collect();
    
    // if let Option::Some(result) = map.get(&String::from("herin")) {
    //     println!("{}", result);
    // }

    // 对于所有权，如String类型，会被map拿走所有权，如i32这种实现Copy的可以被拷贝进哈希map

    // 2. 遍历
    // for (key, val) in map {  // hashmap是无序的，所以遍历也是无序的
    //     println!("key: {}, value: {}", key, val);
    // }

    // 3. 更新
    // map.insert("herin", 12);
    // map.entry("herin").or_insert(0);
    // map.entry("koko").or_insert(100);
    // for (key, val) in map {
    //     println!("{}={}", key, val);
    // }

    // demo：统计字符数量
    let text = "hello world wonderful world";

    let mut counter:HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = counter.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", counter);
}

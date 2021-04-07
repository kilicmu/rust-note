
// fn main() {
//     let result = sum_them(1, 2);
//     println!("result：{}", result);
// }

// fn sum_them(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn main() {
//     let codition = true;
//     let num = if codition {
//         5
//     }else{
//         6
//     };
//     println!("number is {}", num);
// }

// fn main() {
//     let mut count = 0;
//     let result = loop {
//         count += 1;
//         if count == 10 {
//             break count;
//         }
//     };
//     println!("result: {}", result);
// }

// fn main() {
//     let arr = [1,2,3,4];
//     for el in arr.iter() {
//         println!("current is {}", el);
//     }
//     for el in (1..4).rev() {
//         println!("remaining: {}", el);
//     }
// }

fn main() {
    let ret = fib(12);
    println!("result {}", ret)
}

fn fib(n :i32) -> i64 {
    if n == 1 || n == 2 {
        return 1
    }

    return fib(n - 1) + fib(n - 2)
}
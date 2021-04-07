// cargo test
#[cfg(test)] // 指定此mod只有在cargo test才会被编译
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn panic_test() {
        panic!("test panic");
    }

    // 常用宏
    #[test]
    fn assert_demo() {
        let one = 1;
        let two = 2;
        assert!(one < two); // false -> panic
        assert_eq!(one,one); // == -> panic
        assert_ne!(one,two); // != -> panic
        assert!(one > two, "{} less then {}", one, two); // define panic description
    }

    #[test]
    #[should_panic]
    fn should_panic_demo() {
        default_panic();
    }

    #[test]
    #[should_panic(expected = "expect panic")]
    fn should_expect_panic_demo() {
        // default_panic(); //  => failed
        expect_panic(); //  => pass
    }

    #[test]
    fn test_by_result() -> Result<(), String> {
        if 1 + 1 == 2 {
            Ok(())
        }else{
            Err(String::from("error"))
        }
    }

    // 给二进制测试文件传参：-- 使用--分割
    // 测试默认放在多个线程中，如果其中测试互相依赖，需要串行执行，则需要控制单线程执行：cargo test -- --test-threads=1
    // 测试中的println! 默认会被忽略，可以通过 cargo test -- --nocapture
    // 通过 cargo test test_fn_name 来指定测试用例, 支持，使用模糊用例名称，测试多个用例 demo: cargo test panic
    #[test]
    #[ignore] // 默认会被忽略，如果需要单独执行它，使用 cargo test -- --ignored
    fn ignored_fn() {
        assert_eq!(1,2);
    }
    // 一般在tests文件下做集成测试
}

fn default_panic() {
    panic!("test panic")
}

fn expect_panic() {
    panic!("expect panic")
}

pub fn add_two(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试函数
    #[test]
    fn it_works() {
        let result = add(2, 2);

        // 成功的场景
        assert_eq!(result, 4);

        /*
        // 失败的场景
        assert_eq!(result, 5);
        */

        /*
        // 自定义失败信息
        assert_eq!(result, 5, "my result is: {}, my assert is: {} ", result, 5);
         */
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        // 测试panic成功的场景
        Guess::new(200);

        // 测试panic失败的场景
        // Guess::new(30);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200")]
    fn greater_than_100_expected() {
        // 测试panic成功的场景
        Guess::new(200);

        // 测试panic失败的场景
        // Guess::new(201);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // ...省略非核心代码
    }

    #[test]
    fn it_works_assert() {
        // 以下断言的错误信息只包含给定表达式的返回值
        assert!(true);

        fn some_computation() -> bool {
            true
        }

        assert!(some_computation());

        // 使用自定义报错信息
        let x = true;
        assert!(x, "x wasn't true!");

        // 使用格式化的自定义报错信息
        let a = 3;
        let b = 27;
        assert!(a + b == 30, "a = {}, b = {}", a, b);
    }

    #[test]
    fn it_works_assert_ne() {
        let a = 3;
        let b = 1 + 3;
        assert_ne!(a, b, "{} not equal {}", a, b);
    }
}

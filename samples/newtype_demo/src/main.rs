/// 类型别名和`newtype`模式
fn main() {
    // 类型别名
    type KM = f64; //KM只是f64的一个别名

    fn double_in_km(km: KM) -> KM {
        km * 2.0
    }

    let doubled = double_in_km(5.0);

    println!("{}", doubled); // => 10

    // `newtype`模式
    // 注意，Rust中没有直接的newtype语法，但可以通过仅包含一个字段的struct来实现
    struct Kilometers(f64);

    impl Kilometers {
        // 实现一些方法
        fn new(value: f64) -> Kilometers {
            Kilometers(value)
        }

        fn to_meters(&self) -> f64 {
            self.0 * 1000.0
        }
    }

    fn distance_in_km(km: Kilometers) -> Kilometers {
        Kilometers(km.0 * 2.0)
    }

    let distance = Kilometers::new(5.0);
    let doubled = distance_in_km(distance);
    println!("{}", doubled.to_meters()); // => 10000
}

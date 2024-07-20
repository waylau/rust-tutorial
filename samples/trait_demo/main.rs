/// 特征
fn main() {
    // 定义动物特征
    pub trait Animal {
        // 叫唤
        fn sound(&self) -> String;

        // 默认实现
        fn leg(&self) -> String {
            String::from("I have four limbs.")
        }
    }

    // 定义结构体
    pub struct Lion {
        // ...省略字段名
    }

    pub struct Sheep {
        // ...省略字段名
    }

    // 实现特征
    impl Animal for Lion {
        fn sound(&self) -> String {
            format!("Lion sounds: {}", "roar")
        }
    }

    impl Animal for Sheep {
        fn sound(&self) -> String {
            format!("Sheep sounds: {}", "baa")
        }
    }

    let lion = Lion {};
    let sheep = Sheep {};

    println!("{}", lion.sound());
    println!("{}", sheep.sound());

    println!("{}", lion.leg());
    println!("{}", sheep.leg());

    // 特征作为参数
    fn make_sound(animal: &impl Animal) {
        println!("Animal sound. {}", animal.sound());
    }

    // 特征作为参数
    make_sound(&lion);
    make_sound(&sheep);

    // 返回实现了特征的类型
    fn get_animal() -> impl Animal {
        Lion {}
    }

    println!("{}", get_animal().sound());
    println!("{}", get_animal().leg());
}

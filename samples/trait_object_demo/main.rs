/// 特征对象
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

    // 创建了一个 Vec，其中包含了两个不同类型的对象，并将它们转换为特征对象
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(lion), Box::new(sheep)];

    // 遍历 Vec 时，可以通过特征对象调用相应的 sound、leg 方法，实现了灵活的动态分发
    for animal in animals.iter() {
        println!("{}", animal.sound());
        println!("{}", animal.leg());
    }

    // Self 与 self
    trait Draw {
        fn draw(&self) -> Self;
    }

    #[derive(Clone, Debug)]
    struct Button;
    impl Draw for Button {
        fn draw(&self) -> Self {
            return self.clone();
        }
    }

    let button = Button;
    let newb = button.draw();
    dbg!("{}", newb);
}

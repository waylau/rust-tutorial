trait Animal {
    fn speak(&self);
}

struct Dog {
    name: String,
}

impl Dog {
    fn new(name: &str) -> Dog {
        Dog {
            name: name.to_string(),
        }
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says Woof!", self.name);
    }
}

// 工厂函数
fn create_animal(kind: &str, name: &str) -> Box<dyn Animal> {
    match kind {
        "dog" => Box::new(Dog::new(name)),
        // 可以添加更多种类的动物
        _ => panic!("Unknown animal kind"),
    }
}

trait Strategy {
    fn execute(&self) -> String;
}

struct ConcreteStrategyA {}

impl Strategy for ConcreteStrategyA {
    fn execute(&self) -> String {
        "Strategy A is executed".to_string()
    }
}

// 使用策略
fn use_strategy<T: Strategy>(strategy: &T) -> String {
    strategy.execute()
}

fn main() {
    let my_pet = create_animal("dog", "Buddy");
    my_pet.speak();

    let strategy = ConcreteStrategyA {};
    println!("{}", use_strategy(&strategy));
}

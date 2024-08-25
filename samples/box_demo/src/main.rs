/// Box堆对象分配
fn main() {
    // 使用`Box<T>`在堆上储存数据
    let b = 5;
    println!("b: {b}");

    let b = Box::new(5);
    println!("b: {b}");

    // 避免栈上数据的拷贝
    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，
    // 因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());

    // 创建特征对象
    trait Draw {
        fn draw(&self);
    }
    
    struct Button {
        id: u32,
    }
    
    impl Draw for Button {
        fn draw(&self) {
            println!("draw button: {}", self.id)
        }
    }
    
    struct Select {
        id: u32,
    }
    
    impl Draw for Select {
        fn draw(&self) {
            println!("draw select: {}", self.id)
        }
    }
 
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    }
}

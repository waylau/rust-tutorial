/// 迭代器
fn main() {
    // 使用 counter
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // 使用 `map`
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // 对每个数字乘以 2
    let doubled = numbers.iter().map(|x| x * 2);

    // 打印结果
    for num in doubled {
        println!("map: {}", num);
    }

    // 使用 `filter`
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // 筛选偶数
    let even = numbers.iter().filter(|x| *x % 2 == 0);

    // 打印结果
    for num in even {
        println!("filter: {}", num);
    }

    // 使用 `fold`
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // 计算总和
    let sum = numbers.iter().fold(0, |acc, x| acc + x);

    println!("fold: {}", sum);

    // 使用 `collect`
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // 筛选偶数并收集到新的 Vec 中
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| *x % 2 == 0).collect();

    println!("collect: {:?}", even_numbers);

    // 使用链式调用
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // 筛选偶数，乘以 2，收集到 Vec 中
    let doubled_even: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .collect();

    println!("doubled_even: {:?}", doubled_even);
}

// 定义结构体
struct Counter {
    count: usize,
}

// 添加 new() 方法
impl Counter {
    fn new() -> Counter {
        // 计数器从0开始
        Counter { count: 0 }
    }
}

// 给  `Counter` 实现 `Iterator`
impl Iterator for Counter {
    type Item = usize;

    // 只需要实现 next() 方法
    fn next(&mut self) -> Option<Self::Item> {
        // 计数器累加
        self.count += 1;

        // 检查是否计数完成
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

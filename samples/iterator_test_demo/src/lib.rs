// 指定了对外部 crate 的依赖关系
extern crate rand;

// for循环的测试
pub fn sum_for(x: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

// 迭代器的测试
pub fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

#[cfg(test)]
mod tests {
    use rand::{Rng,thread_rng};
    use std::time::Instant;
    use super::*;

    #[test]
    fn it_works() {
        // 构建动态数组
        const LEN: usize = 1024*1024;
        fn rand_array(cnt: u32) -> Vec<f64> {
            let mut rng = thread_rng();
            (0..cnt).map(|_| rng.gen::<f64>()).collect()
        }
        
        // 测试for循环
        let start = Instant::now();
        let samples = rand_array(LEN as u32);
        sum_for(&samples);
        let end = start.elapsed().as_millis();
        println!("sum_for cost {:?} miliseconds", end); // 单位：毫秒

        // 测试迭代器
        let start = Instant::now();
        let samples = rand_array(LEN as u32);
        sum_iter(&samples);
        let end = start.elapsed().as_millis();
        println!("sum_iter cost {:?} miliseconds", end); // 单位：毫秒
    }
 
}
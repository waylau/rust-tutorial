pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn it_works() {
        let start = Instant::now();

        let mut result = 0;
        for i in 1..1000001 {
            result = add(result, i);
        }

        let end = start.elapsed().as_millis();
        println!("result:{} , cost {:?} miliseconds", result, end); // 单位：毫秒
    }
}

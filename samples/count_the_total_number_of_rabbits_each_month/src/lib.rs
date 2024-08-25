pub fn get_rabbits_by_month(month: usize)  -> usize{
    // 三个月以上才能生育
    if month >= 3 {
        return get_rabbits_by_month(month - 1) + get_rabbits_by_month(month - 2);
    } else {
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn it_works() {
        // 测试
        let start = Instant::now();
        let sum = get_rabbits_by_month(50); // 50个月
        let end = start.elapsed().as_millis();  // 单位：毫秒
        println!("sum: {}, cost: {:?} miliseconds", sum, end);
    }
}

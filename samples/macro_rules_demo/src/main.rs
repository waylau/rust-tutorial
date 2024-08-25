/// 声明式宏
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<usize> = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let result = add!(2, 3);
    println!("result: {}", result); // => result: 5

    let vec1 = vec![];
    let vec2 = vec![1];
    let vec3 = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec1); // => []
    println!("{:?}", vec2); // => [1]
    println!("{:?}", vec3); // => [1, 2, 3, 4, 5]
}

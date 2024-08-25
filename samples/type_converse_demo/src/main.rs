/// 类型转换
fn main() {
    // 基本类型之间的转换
    let x: i32 = 5;
    let y: f64 = x as f64; // 将i32类型的x转换为f64类型的y
    println!("y = {}", y); // => y = 5.0

    let a: i8 = 127;
    let b: u8 = a as u8; // 将i8类型的a转换为u8类型的b
    println!("b = {}", b); // => b = 127

    // 注意：从有符号类型转换为无符号类型，当原值为负数时会发生溢出
    let c: i8 = -1;
    let d: u8 = c as u8; // 转换发生溢出
    println!("d = {}", d); // => d = 255 (因为-1的二进制补码在u8中表示为255)

    // 整数类型转换的二进制解释
    let a: i16 = 300; // 16位整数
    let b: i8 = a as i8; // 截断高位，保留低位
    println!("16位整数 {} 转为 8位整数: {}, 其二进制为: {:08b}", a, b, b);
    // => 16位整数 300 转为 8位整数: -52, 其二进制为: 11011000

    let c: u8 = 45; // 8位无符号整数
    let d: u16 = c as u16; // 使用0填充高位
    println!("8位无符号整数 {} 转为16位整数的二进制为: {:016b}", c, d);
    // => 8位无符号整数 45 转为16位整数的二进制为: 0000000000101101

    let e: i8 = -121; // 8位有符号整数
    let f: i16 = e as i16; // 使用符号位填充高位
    println!("8位有符号整数 {} 转为16位整数的二进制为: {:016b}", e, f);
    // => 8位有符号整数 -121 转为16位整数的二进制为: 1111111100001111

    // 使用From和Into特征
    let my_str = "hello";

    // 以下三个转换都依赖于一个事实：String 实现了 From<&str> 特征
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    // 这里需要显式地类型标注
    let string3: String = my_str.into();
    println!("string1: {string1}");
    println!("string2: {string2}");
    println!("string3: {string3}");

    // 自定义结构体之间的转换
    struct Point2D {
        x: i32,
        y: i32,
    }

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    // 为Point3D实现From<Point2D>
    impl From<Point2D> for Point3D {
        fn from(p: Point2D) -> Self {
            Point3D {
                x: p.x,
                y: p.y,
                z: 0,
            }
        }
    }

    let p2d = Point2D { x: 1, y: 2 };
    println!("Point2D: ({}, {})", p2d.x, p2d.y);
    // => Point2D: (1, 2)

    let p3d: Point3D = p2d.into(); // 使用into()方法进行转换
    println!("Point3D: ({}, {}, {})", p3d.x, p3d.y, p3d.z);
    // => Point3D: (1, 2, 0)

 
    let p2d = Point2D { x: 1, y: 2 };
    let p3d = Point3D::from(p2d); // 使用from()方法进行转换
    println!("Point3D: ({}, {}, {})", p3d.x, p3d.y, p3d.z);
    // => Point3D: (1, 2, 0)


    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

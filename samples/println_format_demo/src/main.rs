/// print!、println!和format!
fn main() {
    // print!和println!
    println!("Hello"); // => "Hello"
    println!("Hello, {}!", "world"); // => "Hello, world!"
    println!("The number is {}", 1); // => "The number is 1"
    println!("{:?}", (3, 4)); // => "(3, 4)"
    println!("{value}", value = 4); // => "4"
    println!("{} {}", 1, 2); // => "1 2"
    println!("{:04}", 42); // => "0042" 前面用0补齐

    // format!
    let s = "hello";
    let s1 = format!("{}, world", s);
    print!("{}", s1);

    // eprintln!
    eprintln!("Error: Could not complete task");

    // 位置参数
    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
    println!("{0}, {1}. {1}, {0}", "Alice", "Bob"); // => Alice, Bob. Bob, Alice
    println!("{1}{}{0}{}", 1, 2); // => 2112

    // 具名参数
    println!("{argument}", argument = "rust"); // => "rust"
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"

    /*
    println!("{abc} {1}", abc = "def", 2); // 错误！带名称的参数必须放在不带名称参数的后面
    */

    // 格式化参数，精度
    let pi = 3.1415926;

    // Display
    println!("{:.2}", pi); // => 3.14
                           // Debug
    println!("{:.2?}", pi); // => 3.14
                            // 通过参数来设定精度, 相当于{:.4}
    println!("{:.1$}", pi, 4); // => 3.1416

    let s = "waylau.com";

    // 保留字符串前三个字符
    println!("{:.3}", s); // => way

    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值
    println!("{:.*}", 3, s); // => way

    // 格式化参数，字符串填充
    // 以下全部输出 "x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("{:5}!", "x");
    // 使用参数5来指定宽度
    println!("{:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("{1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("{:width$}!", "x", width = 5);

    // 格式化参数，数字填充
    // 宽度是5
    println!("Hello {:5}!", 5); //  => Hello     5!
                                // 显式的输出正号
    println!("Hello {:+}!", 5); //  => Hello +5!
                                // 宽度5，使用0进行填充
    println!("Hello {:05}!", 5); //  => Hello 00005!
                                 // 负号也要占用一位宽度
    println!("Hello {:05}!", -5); //  => Hello -0005!

    // 格式化参数，对齐
    // 以下全部都会补齐5个字符的长度
    // 左对齐
    println!("Hello {:<5}!", "x"); //  => Hello x    !
                                   // 右对齐
    println!("Hello {:>5}!", "x"); //  => Hello     x!
                                   // 居中对齐
    println!("Hello {:^5}!", "x"); //  => Hello   x  !

    // 对齐并使用指定符号填充
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x"); //  => Hello x&&&&!

    // 格式化参数，进制
    // 二进制
    println!("{:#b}!", 27); // => 0b11011!
                            // 八进制
    println!("{:#o}!", 27); // => 0o33!
                            // 十进制
    println!("{}!", 27); // => 27!
                         // 小写十六进制
    println!("{:#x}!", 27); // => 0x1b!
                            // 大写十六进制
    println!("{:#X}!", 27); // => 0x1B!

    // 不带前缀的十六进制
    println!("{:x}!", 27); // => 1b!

    // 使用0填充二进制，宽度为10
    println!("{:#010b}!", 27); // => 0b00011011!

    // 格式化参数，指数
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9

    // 格式化参数，指针地址
    let v = vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x22520b97810，该地址与你输出的不同

    // 格式化参数，转义
    // "{{" 转义为 '{'   "}}" 转义为 '}'   "\"" 转义为 '"'
    println!(" Hello \"{{World}}\" "); // => Hello "{World}"
}

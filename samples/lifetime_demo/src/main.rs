/// 悬垂引用
fn main() {
	let r; // 未初始化

    {
        let x = 5;
        r = &x;
    } // x在此处被释放

    println!("r: {r}"); // 悬垂引用
}

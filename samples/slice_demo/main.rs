/// slice
fn main() {
    // 字符串slice
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("part1: {part1}, part2: {part2}");

    let s = String::from("waylau.com");

    let s1 = &s[0..2];
    let s2 = &s[..2];
    println!("s1: {s1}, s2: {s2}");

    let s = String::from("waylau.com");

    let len = s.len();

    let s1 = &s[3..len];
    let s2 = &s[3..];
    println!("s1: {s1}, s2: {s2}");

    let s = String::from("waylau.com");

    let len = s.len();

    let s1 = &s[0..len];
    let s2 = &s[..];
    println!("s1: {s1}, s2: {s2}");

    // 其他slice
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

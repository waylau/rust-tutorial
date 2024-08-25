/// 使用组合器

fn main() {
    // or()和and()
    let some1 = Some("some1");
    let some2 = Some("some2");
    let none: Option<&str> = None;

    let ok1: Result<&str, &str> = Ok("ok1");
    let ok2: Result<&str, &str> = Ok("ok2");
    let error1: Result<&str, &str> = Err("error1");
    let error2: Result<&str, &str> = Err("error2");

    assert_eq!(some1.or(some2), some1); // Some1 or Some2 = Some1
    assert_eq!(some1.or(none), some1); // Some or None = Some
    assert_eq!(ok1.or(ok2), ok1); // Ok1 or Ok2 = Ok1
    assert_eq!(ok1.or(error1), ok1); // Ok or Err = Ok

    assert_eq!(some1.and(some2), some2); // Some1 and Some2 = Some2
    assert_eq!(some1.and(none), none); // Some and None = None
    assert_eq!(none.and(some1), none); // None and Some = None
    assert_eq!(none.and(none), none); // None1 and None2 = None1

    assert_eq!(error1.and(ok1), error1); // Err and Ok = Err
    assert_eq!(error1.and(error2), error1); // Err1 and Err2 = Err1

    // or_else()和and_then()
    fn sq(x: u32) -> Result<u32, u32> {
        Ok(x * x)
    }
    fn err(x: u32) -> Result<u32, u32> {
        Err(x)
    }

    assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
    assert_eq!(Ok(2).or_else(err).or_else(sq), Ok(2));
    assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
    assert_eq!(Err(3).or_else(err).or_else(err), Err(3));

    fn nobody() -> Option<&'static str> {
        None
    }
    fn vikings() -> Option<&'static str> {
        Some("vikings")
    }

    assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
    assert_eq!(None.or_else(vikings), Some("vikings"));
    assert_eq!(None.or_else(nobody), None);

    let arr_2d = [["A0", "A1"], ["B0", "B1"]];

    let item_0_1 = arr_2d.get(0).and_then(|row| row.get(1));
    assert_eq!(item_0_1, Some(&"A1"));

    let item_2_0 = arr_2d.get(2).and_then(|row| row.get(0));
    assert_eq!(item_2_0, None);

    fn sq_then_to_string(x: u32) -> Result<String, &'static str> {
        x.checked_mul(x)
            .map(|sq| sq.to_string())
            .ok_or("overflowed")
    }

    assert_eq!(Ok(2).and_then(sq_then_to_string), Ok(4.to_string()));
    assert_eq!(Ok(1_000_000).and_then(sq_then_to_string), Err("overflowed"));
    assert_eq!(
        Err("not a number").and_then(sq_then_to_string),
        Err("not a number")
    );

    // filter()
    // 判断是否是偶数
    fn is_even(n: &i32) -> bool {
        n % 2 == 0
    }

    assert_eq!(None.filter(is_even), None);
    assert_eq!(Some(3).filter(is_even), None);
    assert_eq!(Some(4).filter(is_even), Some(4));

    // map()和map_err()
    let maybe_some_string = Some(String::from("waylau.com"));

    // 将字符串映射为长度
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(10));

    let x: Option<&str> = None;
    assert_eq!(x.map(|s| s.len()), None);

    fn stringify(x: u32) -> String {
        format!("error code: {x}")
    }

    let x: Result<u32, u32> = Ok(2);
    // 将Err中的值进行改变
    assert_eq!(x.map_err(stringify), Ok(2));

    let x: Result<u32, u32> = Err(13);
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));

    // map_or()和map_or_else()
    let x = Some(String::from("waylau.com"));
    assert_eq!(x.map_or(42, |v| v.len()), 10);

    let x: Option<&str> = None;
    assert_eq!(x.map_or(42, |v| v.len()), 42);

    let k = 21;

    let x = Some(String::from("waylau.com"));
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 10);

    let x: Option<&str> = None;
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);

    // ok_or()和ok_or_else()
    let x = Some("waylau.com");
    assert_eq!(x.ok_or(0), Ok("waylau.com"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));

    let x = Some("waylau.com");
    assert_eq!(x.ok_or_else(|| 0), Ok("waylau.com"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or_else(|| 0), Err(0));
}

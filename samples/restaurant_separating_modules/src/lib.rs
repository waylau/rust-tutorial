mod front_of_house;

// 使用 use 关键字创建一个短路径 hosting，
// 然后就可以在作用域中的任何地方使用这个更短的名字 hosting
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
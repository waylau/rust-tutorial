/// 实战：通用max函数
fn main() {
    let cit1 = Citation {
        author: "Waylau",
        year: 2011,
    };
    let cit2 = Citation {
        author: "Baumann",
        year: 2010,
    };
    let cit3 = Citation {
        author: "Baumann",
        year: 2019,
    };

    dbg!("max(cit1, cit2): {}", max(cit1, cit2));
    dbg!("max(cit2, cit3): {}", max(cit2, cit3));
    dbg!("max(cit1, cit3): {}", max(cit1, cit3));
}

// 定义特征
trait GreaterThan {
    /// 比较前者是否比后者大
    fn greater_than(&self, other: &Self) -> bool;
}

// 定义结构体
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Citation {
    author: &'static str,
    year: u32,
}

impl GreaterThan for Citation {
    fn greater_than(&self, other: &Self) -> bool {
        // 先比较author，则比较year
        if self.author > other.author {
            true
        } else if self.author < other.author {
            false
        } else {
            self.year > other.year
        }
    }
}

fn max<T: GreaterThan>(l: T, r: T) -> T {
    if l.greater_than(&r) {
        l
    } else {
        r
    }
}

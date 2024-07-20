/// 实战：通用min函数
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

    dbg!("min(cit1, cit2): {}", min(cit1, cit2));
    dbg!("min(cit2, cit3): {}", min(cit2, cit3));
    dbg!("min(cit1, cit3): {}", min(cit1, cit3));
}

// 定义特征
trait LessThan {
    /// 比较前者是否比后者小
    fn less_than(&self, other: &Self) -> bool;
}

// 定义结构体
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Citation {
    author: &'static str,
    year: u32,
}

impl LessThan for Citation {
    fn less_than(&self, other: &Self) -> bool {
        // 先比较author，则比较year
        if self.author < other.author {
            true
        } else if self.author > other.author {
            false
        } else {
            self.year < other.year
        }
    }
}

fn min<T: LessThan>(l: T, r: T) -> T {
    if l.less_than(&r) {
        l
    } else {
        r
    }
}

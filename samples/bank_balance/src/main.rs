/// 银行存取款
fn main() {
    use std::cell::Cell;

    // 定义银行结构体
    struct Bank {
        // 使用Cell存储余额，因为余额是内部可变的
        balance: Cell<i32>,
    }

    impl Bank {
        // 创建一个新的银行对象
        fn new() -> Bank {
            // 初始化余额为0
            Bank {
                balance: Cell::new(0),
            }
        }

        // 存款
        fn deposit(&self, amount: i32) {
            // 修改余额
            self.balance.set(self.balance.get() + amount);
        }

        // 取款
        fn withdraw(&self, amount: i32) -> bool {
            // 如果余额充足，则修改余额并返回true
            if self.balance.get() >= amount {
                self.balance.set(self.balance.get() - amount);
                true
            // 否则返回false
            } else {
                false
            }
        }
    }

    // 创建一个新的银行
    let bank = Bank::new();
    // 存款100元
    bank.deposit(100);
    // 取款50元
    bank.withdraw(50);
    // 余额应该是50元
    println!("balance: {}", bank.balance.get());
}

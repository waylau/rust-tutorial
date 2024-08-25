/// 实现简单的栈数据结构

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

/// 定义栈的数据结构
struct MyStack {
    data: *mut i32,
    top: isize,
    capacity: isize,
}

impl MyStack {
    /// 创建一个新的空栈
    fn new(capacity: isize) -> Result<Self, String> {
        if capacity <= 0 {
            return Err("Capacity must be greater than zero".to_string());
        }

        let layout = Layout::array::<i32>(capacity as usize).unwrap();
        let data = unsafe { alloc(layout) } as *mut i32;

        Ok(MyStack {
            data,
            top: -1,
            capacity,
        })
    }

    /// 向栈中添加一个元素
    fn push(&mut self, value: i32) -> Result<(), String> {
        if self.top + 1 >= self.capacity {
            return Err("Stack overflow".to_string());
        }

        unsafe {
            ptr::write(self.data.offset(self.top + 1), value);
            self.top += 1;
        }

        Ok(())
    }

    /// 从栈顶移除并返回一个元素
    fn pop(&mut self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Stack underflow".to_string());
        }

        unsafe {
            self.top -= 1;
            Ok(ptr::read(self.data.offset(self.top + 1)))
        }
    }

    /// 查看栈顶的元素但不移除
    fn peek(&self) -> Result<&i32, String> {
        if self.is_empty() {
            return Err("Stack is empty".to_string());
        }

        unsafe {
            Ok(&*self.data.offset(self.top))
        }
    }

    /// 判断栈是否为空
    fn is_empty(&self) -> bool {
        self.top == -1
    }
}

impl Drop for MyStack {
    /// 当栈被丢弃时释放内存
    fn drop(&mut self) {
        unsafe {
            dealloc(self.data as *mut u8, Layout::array::<i32>(self.capacity as usize).unwrap());
        }
    }
}

/// 测试用例
fn main() {
    let mut stack = match MyStack::new(5) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    assert!(stack.is_empty());

    match stack.push(1) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    match stack.push(2) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    assert_eq!(stack.peek().unwrap(), &2);
    assert_eq!(stack.pop().unwrap(), 2);
    assert_eq!(stack.pop().unwrap(), 1);
    assert!(stack.is_empty());
}
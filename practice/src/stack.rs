use crate::main;
use std::{fmt, slice};

struct TopNode<T> {
    pub len: usize,
    pub next: Option<Box<StructNode<T>>>,
}

struct StructNode<T> {
    pub value: T,
    pub next: Option<Box<StructNode<T>>>,
}

impl<T> StructNode<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }

    fn with_next(value: T, next: Option<Box<StructNode<T>>>) -> Self {
        Self { value, next }
    }
}

#[cfg(test)]
impl<T> fmt::Display for StructNode<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "value: {}\nnext: {}",
            self.value,
            match self.next {
                Some(_) => "more",
                None => "none",
            }
        )
    }
}

#[test]
fn struct_node_test() {
    let node = StructNode::new(12);
    println!("{}", node);
    assert!(false);
}

pub struct Stack<T> {
    list: TopNode<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            list: TopNode { len: 0, next: None },
        }
    }

    pub fn push(&mut self, value: T) {
        self.list.len += 1;
        let new_node = Box::new(StructNode::with_next(value, self.list.next.take()));
        self.list.next.replace(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.list.next.take() {
            Some(mut node) => {
                self.list.len -= 1;
                if let Some(next_box) = node.next.take() {
                    self.list.next.replace(next_box);
                }
                Some(node.value)
            }
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.list.len
    }

    pub fn else_f(&self) {}
}

trait IntoStack<T> {
    fn into_stack(self) -> Stack<T>;
}

impl<T> IntoStack<T> for Vec<T> {
    fn into_stack(self) -> Stack<T> {
        let mut stack = Stack::new();
        for i in self {
            stack.push(i);
        }
        stack
    }
}

macro_rules! stack {
    [$($x : expr),+] => {
        vec![$($x),+].into_stack();
    };
}

///error
/// ```
/// impl <T> Stack<T>{
///    pub fn map(&self) {}
///    pub fn map(&mut self) {
///    pub fn map(&mut self, f : fn(T)->T) {
///        let mut p = &mut self.list.next;
///        while let Some(node) = p {
///            node.value = f(node.value);
///            p = &mut node.next;
///        }
///    }
/// }
/// ```
impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_pop_test() {
        let mut stack = Stack::new();
        stack.push(12);
        let a = stack.pop().unwrap();
        let b = stack.pop();
        assert_eq!(12, a);
        assert_eq!(b, None)
    }

    #[test]
    fn iter_test() {
        let v = [1, 2, 3];
        let iter = v.iter();
        let into_iter = vec![1, 2, 3].into_iter();
        let v_iter = vec![1, 2, 3].iter();
    }
    #[test]
    fn stack_iter_test() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        for i in stack {
            println!("{}", i);
        }
    }

    #[test]
    fn stack_macro_test() {
        let stack = stack![1, 2, 3, 4];
        for i in stack {
            println!("{}", i);
        }
    }

    #[test]
    fn stack_closure_test() {
        let mut v = vec![1, 2, 3];
        let v = v.into_iter().map(|x| 2 * x).collect::<Vec<i32>>();
        println!("{:?}", v);
        assert!(false);
    }

    #[test]
    fn map_test() {
        //        let mut s = stack![1,2,3,4,5];
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        s.push(5);
        s.push(6);
        //        s.map(|x| 2*x);
        //        s.map();
        //        s.else_f();
        for i in s {
            println!("{}", i);
        }
    }
}

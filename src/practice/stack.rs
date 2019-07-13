use std::fmt;
use crate::main;

struct TopNode<T> {
    pub len : usize,
    pub next : Option<Box<StructNode<T>>>
}

struct StructNode<T> {
    pub value : T,
    pub next : Option<Box<StructNode<T>>>
}

impl <T> StructNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next : None
        }
    }

    fn with_next( value: T, next : Option<Box<StructNode<T>>>) -> Self{
        Self {
            value,
            next
        }
    }
}

impl <T> fmt::Display for StructNode<T>
    where T: fmt::Display
{
    #[cfg(test)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,"value: {}\nnext: {}",self.value, match self.next {
            Some(_) => "more",
            None => "none"
        })
    }
}

#[test]
fn struct_node_test() {
    let node = StructNode::new(12);
    println!("{}", node);
    assert!(false);
}

pub struct Stack<T> {
    list : TopNode<T>
}

impl <T> Stack<T> {
    fn new() -> Self{
        Self {
            list : TopNode {
                len : 0,
                next : None
            }
        }
    }

    fn push(&mut self, value : T) {
        self.list.len += 1;
        let new_node = Box::new(StructNode::with_next(value, self.list.next.take()));
        self.list.next.replace(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.list.len -= 1;
        match self.list.next.take() {
            Some(mut node) => {
                if let Some(next_box) = node.next.take() {
                    self.list.next.replace(next_box);
                }
                Some(node.value)
            },
            None => None
        }
    }

    fn len(&self) -> usize {
        self.list.len
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

}

use std::rc::Rc;

pub struct HeaderNode<T> {
    pub size : usize,
    pub next : Option<Box<LinkNode<T>>>
}

pub struct LinkNode<T> {
    pub value : T,
    pub next : Option<Box<LinkNode<T>>>
}

impl <T> LinkNode<T> {
    fn new(value : T) -> LinkNode<T>{
        LinkNode {
            value,
            next : None
        }
    }

    fn with_next(value: T, next : Option<Box<LinkNode<T>>>) -> LinkNode<T> {
        LinkNode {
            value,
            next
        }
    }
}

pub struct LinkList<T> {
    header : Box<HeaderNode<T>>,
}

impl <T> LinkList<T> {
    fn new() -> LinkList<T> {
        LinkList {
            header : Box::new(HeaderNode {
                size : 0,
                next: None
            })
        }
    }

    fn push(&mut self, value : T) {
        let new_node = Box::new(LinkNode::new(value));
        let mut p = &mut self.header.next;
        while let Some(node) = p {
            p =  &mut node.next;
        }
        p.replace(new_node);
        self.header.size += 1;
    }

    fn get(&self, index : usize) -> Option<&T>{
        let mut p = &self.header.next;
        for i in 0..index {
            match p {
                Some(node) => p = &node.next,
                None => return None
            }
        }
        let a = &p.as_ref().unwrap().value;
        Some(a)
    }

    fn len(&self) -> usize {
        return self.header.size
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::LinkedList;

    #[test]
    fn new_test() {
        let mut a = LinkList::new();
        a.push(12);
        a.push(13);

        for i in 0..a.len() {
            let mes = a.get(i).unwrap();
            println!("{}", mes);
        }

        assert!(false);
    }

    #[test]
    fn std_linkedlist_test() {
        let mut l = LinkedList::new();
        l.push_back(12);
        l.push_back(14);
        let mut v  = Vec::with_capacity(l.len());
        for i in l.iter() {
            v.push(i.clone());
        }
        assert_eq!(v, vec![12,14])
    }
}
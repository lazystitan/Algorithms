struct Bag<T> {
    bag : Vec<T>,
}

impl<T> Bag<T> {
    pub fn new() -> Bag<T> {
        let bag = Vec::new();

        Bag {
            bag,
        }
    }

    pub fn add(&mut self, item : T) {
        self.bag.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty()
    }

    pub fn size(&self) -> usize {
        self.bag.len()
    }
}

impl<T> IntoIterator for Bag<T> {
    type Item = (T);
    type IntoIter = (std::vec::IntoIter<T>);

    fn into_iter(self) -> Self::IntoIter {
        self.bag.into_iter()
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut bag = Bag::new();
        bag.add(5);
        bag.add(3);

        for i in bag {
            eprintln!("{}",i);
        }

        assert!(false);
    }
}
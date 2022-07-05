struct MapTest<T>(T);

///
///不能，因为Iterator中有map会冲突
///```
///     impl <T> MapTest<T> {
///         pub fn map(&self) {}
///     }
///
///     a.map();
///```

impl<T> Iterator for MapTest<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let a = MapTest(4);
    }
}

use super::sort_trait::SortTrait;
use std::fmt::Display;

pub struct InsertSort;

impl <T> SortTrait<T> for InsertSort
    where T:PartialEq + PartialOrd
{
    fn sort(vector: &mut Vec<T>) {
        let n = vector.len();
        for i in 1..n {
            for  j in (1..(i+1)).rev() {
                if Self::less(&(vector[j]), &(vector[j-1]) ) {
                    Self::exchange(vector, j, j-1);
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut vector = vec![1,4,9,7,6,8,5,2,3];
        InsertSort::sort(&mut vector);
        eprintln!("{:?}",vector);
        assert!(InsertSort::is_sorted(&vector));
        assert!(false);
    }

}
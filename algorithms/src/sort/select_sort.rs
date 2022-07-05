use super::sort_trait::SortTrait;

pub struct SelectSort;

impl<T> SortTrait<T> for SelectSort
where
    T: PartialEq + PartialOrd,
{
    fn sort(vector: &mut Vec<T>) {
        let n = vector.len();
        for i in 0..n {
            let mut min = i;
            for j in (i + 1)..n {
                if Self::less(&vector[j], &vector[min]) {
                    min = j;
                }
            }
            Self::exchange(vector, i, min);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut vector = vec![2, 3, 1, 4, 9, 7, 6, 8];
        SelectSort::sort(&mut vector);
        eprintln!("{:?}", vector);
        assert!(SelectSort::is_sorted(&vector));
    }
}

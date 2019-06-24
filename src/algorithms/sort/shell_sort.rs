use super::sort_trait::SortTrait;
use std::fmt::Display;

pub struct ShellSort;

impl <T : Display> SortTrait<T> for ShellSort
    where T : PartialOrd + PartialEq
{
    fn sort(vector: &mut Vec<T>) {
        let n = vector.len();
        let mut h = 1;
        while h < n / 3 {
            h = 3*h + 1;
        }

        while h >= 1 {
            for i in h..n {
                for j in (h..(i+1)).rev().step_by(h) {
                    if Self::less(&vector[j], &vector[j-h]) {
                        Self::exchange(vector, j, j-h);
                    } else {
                        break;
                    }
                }
            }
            h = h/3;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut vector = vec![1,4,9,7,6,8,5,2,3];
        ShellSort::sort(&mut vector);
        eprintln!("{:?}",vector);
        assert!(ShellSort::is_sorted(&vector));
//        assert!(false);
    }

}
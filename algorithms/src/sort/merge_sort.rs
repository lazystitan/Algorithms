use crate::sort::SortTrait;
use std::fmt::{Debug, Display};

pub struct MergeSort;

impl<T> SortTrait<T> for MergeSort
where
    T: PartialEq + PartialOrd + Clone,
{
    fn sort(vector: &mut Vec<T>) {
        let mut aux = Vec::with_capacity(vector.len());
        for e in &*vector {
            aux.push(e.clone());
        }
        Self::merge_sort(vector, 0, vector.len() - 1, &mut aux);
    }
}

impl MergeSort {
    fn merge<T>(vector: &mut Vec<T>, low: usize, mid: usize, high: usize, aux: &mut Vec<T>)
    where
        T: PartialOrd + PartialEq + Clone,
    {
        let mut i = low; //index for left
        let mut j = mid + 1; //index for right
        for k in low..high + 1 {
            //auxiliary vector
            aux[k] = vector[k].clone();
        }
        for k in low..high + 1 {
            if i > mid {
                //left was run out
                vector[k] = aux[j].clone();
                j += 1;
            } else if j > high {
                //right was run out

                vector[k] = aux[i].clone();
                i += 1;
            } else if Self::less(&aux[j], &aux[i]) {
                //right is less than left
                vector[k] = aux[j].clone();
                j += 1;
            } else {
                //left is less than right
                vector[k] = aux[i].clone();
                i += 1;
            }
        }
    }

    fn merge_sort<T>(vector: &mut Vec<T>, low: usize, high: usize, aux: &mut Vec<T>)
    where
        T: PartialOrd + PartialEq + Clone,
    {
        if high <= low {
            return;
        }
        let mid = (low + high) / 2;
        Self::merge_sort(vector, low, mid, aux); //left
        Self::merge_sort(vector, mid + 1, high, aux); //right
        Self::merge(vector, low, mid, high, aux);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut vector = vec![
            67, 7, 234, 270, 12, 3532, 45, 12, 75, 2, 1, 31, 643, 1257, 15, 72, 326, 345, 76,
        ];
        let len = vector.len();
        MergeSort::sort(&mut vector);
        eprintln!("{:?}", vector);
        assert!(MergeSort::is_sorted(&vector));
        assert_eq!(vector.len(), len);
    }
}

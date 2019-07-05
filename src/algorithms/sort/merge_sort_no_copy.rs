use crate::algorithms::sort::SortTrait;
use std::fmt::{Display, Debug};

pub struct MergeSortNC;

impl <T> SortTrait<T> for MergeSortNC
    where T:PartialEq + PartialOrd
{
    fn sort(vector: &mut Vec<T>) {
        let len = vector.len();
        let mut new_vector = Vec::with_capacity(len);
        for _ in 0..len {
            new_vector.insert(0, vector.pop());
        }
        let mut aux = Vec::with_capacity(len);
        for i in 0..len {
            aux.push(None);
        }
        Self::merge_sort(&mut new_vector, 0, len - 1, &mut aux);
        for _ in 0..len {
            vector.insert(0, new_vector.pop().unwrap().unwrap());
        }
    }
}

impl MergeSortNC {
    fn merge<T>(vector: &mut Vec<Option<T>>, low: usize, mid: usize, high: usize, aux : &mut Vec<Option<T>>)
        where T:PartialOrd + PartialEq
    {
        let mut i = low; //index for left
        let mut j = mid + 1; //index for right
        for k in low..high + 1 {
            //auxiliary vector
//            let some = (&mut vector[k]).take().unwrap();
            aux[k] = (&mut vector[k]).take();
        }
        for k in low..high + 1 {
            if i > mid {
                //left was run out
                vector[k] = (&mut aux[j]).take();
                j += 1;
            } else if j > high {
                //right was run out

                vector[k] = (&mut aux[i]).take();
                i += 1;
            } else if Self::less(&aux[j], &aux[i]) {
                //right is less than left
                vector[k] = (&mut aux[j]).take();
                j += 1;
            } else {
                //left is less than right
                vector[k] = (&mut aux[i]).take();
                i += 1;
            }
        }
    }

    fn merge_sort<T>(vector : &mut Vec<Option<T>>, low : usize, high : usize, aux : &mut Vec<Option<T>>)
        where T:PartialOrd + PartialEq
    {
        if high <= low {
            return;
        }
        let mid = (low + high) / 2;
        Self::merge_sort(vector, low, mid, aux); //left
        Self::merge_sort(vector, mid+1, high, aux); //right
        Self::merge(vector, low, mid, high, aux);
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut vector = vec![67, 7 ,234,270,12,3532,45,12,75,2,1,31,643,1257,15,72,326,345,76];
        let len = vector.len();
        MergeSortNC::sort(&mut vector);
        eprintln!("{:?}",vector);
        assert!(MergeSortNC::is_sorted(&vector));
        assert_eq!(vector.len(), len);
    }

}
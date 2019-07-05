use crate::algorithms::sort::SortTrait;
use std::fmt::{Display, Debug};
use std::cmp::min;

pub struct MergeBUSort;

impl <T> SortTrait<T> for MergeBUSort
    where T:PartialEq + PartialOrd
{
    fn sort(vector: &mut Vec<T>) {
        let length = vector.len();

        let mut new_vector = Vec::with_capacity(length);
        for _ in 0..length {
            new_vector.insert(0, vector.pop());
        }

        let mut aux = Vec::with_capacity(length);
        for i in 0..length {
            aux.push(None);
        }

        let mut size = 1;
        while size < length {
            let mut low = 0;
            while low < length - size {
                Self::merge(&mut new_vector,
                            low, //低位
                            low + size -1,//中间
                            min(low + size + size -1, length - 1),//高位
                            &mut aux);
                low = size + size + low;
            }
            size = size + size;//每次成二
        }


        for _ in 0..length {
            vector.insert(0, new_vector.pop().unwrap().unwrap());
        }
    }
}

impl MergeBUSort {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut vector = vec![67, 7 ,234,270,12,3532,45,12,75,2,1,31,643,1257,15,72,326,345,76];
        let len = vector.len();
        MergeBUSort::sort(&mut vector);
        eprintln!("{:?}",vector);
        assert!(MergeBUSort::is_sorted(&vector));
        assert_eq!(vector.len(), len);
    }

}
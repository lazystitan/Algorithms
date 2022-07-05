use crate::sort::SortTrait;
use core::borrow::Borrow;

pub struct QuickSort;

impl<T> SortTrait<T> for QuickSort
where
    T: PartialOrd + PartialEq,
{
    fn sort(vector: &mut Vec<T>) {
        let length = vector.len() as i32;
        let mut new_vector = Vec::with_capacity(length as usize);
        for i in 0..length {
            new_vector.push(vector.pop());
        }

        //there should have a function that can eliminate dependency of inputs
        //StdRandom.shuffle(vector);

        Self::quick_sort(&mut new_vector, 0, length - 1);

        for i in 0..length {
            vector.insert(0, new_vector.pop().unwrap().unwrap());
        }
    }
}

impl QuickSort {
    fn quick_sort<T>(vector: &mut Vec<Option<T>>, low: i32, high: i32)
    where
        T: PartialOrd + PartialEq,
    {
        if low >= high {
            return;
        }
        let j = Self::partition(vector, low, high);
        let mut j2 = 0;
        println!("j:{}", j);
        Self::quick_sort(vector, low, j - 1);
        Self::quick_sort(vector, j + 1, high);
    }

    fn partition<T>(vector: &mut Vec<Option<T>>, low: i32, high: i32) -> i32
    where
        T: PartialOrd + PartialEq,
    {
        let mut i = low;
        let mut j = high + 1;
        ///     right but two complex:
        /// ```
        ///     let mut value = &mut Some((&mut vector[low as usize]).take().unwrap());
        ///     //......other code
        ///     (&mut vector[low as usize]).replace(value.take().unwrap());
        /// ```
        ///     error eg:
        /// ```
        ///     let mut value = vector[low as usize];
        /// ```
        ///     use &mut vector[low as usize] get mutable borrow from vector which is &mut Option,
        ///     then use take to move out T
        let mut value = (&mut vector[low as usize]).take();
        i += 1;
        j -= 1;
        loop {
            //find the element that is bigger than value(the first element of this part) from left
            while Self::less(&vector[i as usize], &value) {
                if i == high {
                    break;
                }
                i += 1;
            }
            //find the element that is less than value from right
            //or find the smaller element ready for exchange with first element when j is less than is
            while Self::less(&value, &vector[j as usize]) {
                if j == low {
                    break;
                }
                j -= 1;
            }
            if i >= j {
                break;
            }
            Self::exchange(vector, i as usize, j as usize);
        }
        //put value back to vector
        (&mut vector[low as usize]).replace(value.unwrap());
        //exchange value with vector[j]
        Self::exchange(vector, low as usize, j as usize);
        j
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
        QuickSort::sort(&mut vector);
        eprintln!("{:?}", vector);
        assert!(QuickSort::is_sorted(&vector));
        assert_eq!(vector.len(), len);
    }
}

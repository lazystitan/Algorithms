pub trait SortTrait<T>
where
    T: PartialEq + PartialOrd,
{
    fn sort(vector: &mut Vec<T>);

    fn less(a: &T, b: &T) -> bool {
        a < b
    }

    fn exchange(vector: &mut Vec<T>, i: usize, j: usize) {
        vector.swap(i, j);
    }

    fn is_sorted(vector: &Vec<T>) -> bool {
        for i in 1..vector.len() {
            if Self::less(&vector[i], &vector[i - 1]) {
                return false;
            }
        }
        true
    }
}

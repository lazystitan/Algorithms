mod sort_trait;
mod select_sort;
mod insert_sort;
mod shell_sort;
mod merge_sort;
mod merge_sort_no_copy;
mod merge_bu_sort;
mod quick_sort;

pub use sort_trait::SortTrait;

pub use select_sort::SelectSort;

pub use insert_sort::InsertSort;

pub use shell_sort::ShellSort;

pub use merge_sort::MergeSort;
pub use merge_sort_no_copy::MergeSortNC;
pub use merge_bu_sort::MergeBUSort;

pub use quick_sort::QuickSort;

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::time::SystemTime;

    #[test]
    fn time_test_purely_random() {
        let capacity = 1_0000;
        let mod_number = 2 * capacity;
        let mut vector = Vec::with_capacity(capacity);
        let mut rng = thread_rng();
        println!("Initial vector");
        for _ in 0..capacity {
            vector.push(rng.gen_range(0, mod_number));
        }
        println!("Vector ready");
        println!("Clone vector...");
        let mut vector1 = vector.clone();
        let mut vector2 = vector.clone();
        println!("Done");

        println!("Start");
        let start = SystemTime::now();
        SelectSort::sort(&mut vector);
        let since_the_epoch = start.elapsed().unwrap();
        println!("Done, SelectSort duration {}", since_the_epoch.as_millis() );

        println!("Start");
        let start = SystemTime::now();
        InsertSort::sort(&mut vector1);
        let since_the_epoch = start.elapsed().unwrap();
        println!("Done, InsertSort duration {}", since_the_epoch.as_millis() );

        println!("Start");
        let start = SystemTime::now();
        ShellSort::sort(&mut vector2);
        let since_the_epoch = start.elapsed().unwrap();
        println!("Done, ShellSort duration {}", since_the_epoch.as_millis() );

        assert!(false);

    }

    #[test]
    fn time_test_already_ordered() {
        let capacity = 1_0000;
        let mod_number = 2 * capacity;
        let mut vector = Vec::with_capacity(capacity);
        let mut rng = thread_rng();
        println!("Initial vector");
        for i in 0..capacity {
            vector.push(i);
        }
        println!("Vector ready");
        println!("Clone vector...");
        let mut vector1 = vector.clone();
        let mut vector2 = vector.clone();
        println!("Done");

        println!("Start");
        let start = SystemTime::now();
        SelectSort::sort(&mut vector);
        let since_the_epoch = start.elapsed().unwrap();
        println!("Done, SelectSort duration {}", since_the_epoch.as_millis() );

        println!("Start");
        let start = SystemTime::now();
        InsertSort::sort(&mut vector1);
        let since_the_epoch = start.elapsed().unwrap();
        println!("Done, InsertSort duration {}", since_the_epoch.as_millis() );

        println!("Start");
        let start = SystemTime::now();
        ShellSort::sort(&mut vector2);
        let since_the_epoch = start.elapsed().unwrap();
        println!("Done, ShellSort duration {}", since_the_epoch.as_millis() );

        assert!(false);
    }
}
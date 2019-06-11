pub trait Search {
    fn search(&self, text: &String) -> usize;
}
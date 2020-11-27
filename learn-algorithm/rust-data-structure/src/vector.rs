trait VecADT<T> {
    type Rank;
    type Type;

    fn copy_from(&mut self, a: *const T, lo: Self::Rank, hi: Self::Rank);
    fn expand(&mut self);
    fn shrink(&mut self);
    fn bubble(&mut self, lo: Self::Rank, hi: Self::Rank) -> bool;
    fn bubble_sort(&mut self, lo: Self::Rank, hi: Self::Rank);
    fn max(&self, lo: Self::Rank, hi: Self::Rank) -> Self::Type;
    fn selection_sort(&mut self, lo: Self::Rank, hi: Self::Rank);
    fn merge(&mut self, lo: Self::Rank, mi: Self::Rank, hi: Self::Rank);
    fn merge_sort(&mut self, lo: Self::Rank, hi: Self::Rank);
    fn partition(&mut self, lo: Self::Rank, hi: Self::Rank) -> Self::Rank;
    fn quick_sort(&mut self, lo: Self::Rank, hi: Self::Rank);
    fn heap_sort(&mut self, lo: Self::Rank, hi: Self::Rank);

    fn size(&self) -> Self::Rank;
    fn is_empty(&self) -> bool;
    fn disordered(&self) -> bool;
    fn find(e: &T) -> Self::Rank;
    fn find_scope(e: &T, lo: Self::Rank, hi: Self::Rank);
    fn search(e: &T) -> Self::Rank;
    fn search_scope(e: &T, lo: Self::Rank, hi: Self::Rank);
    

}
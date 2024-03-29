#![allow(dead_code)]
/**
 * (5 kyu) (SKIPPED. Does not work for some reason)
 * https://www.codewars.com/kata/515bb423de843ea99400000a/train/rust
 *
 * For this exercise you will be strengthening your page-fu mastery.
 * You will complete the PaginationHelper class, which is a utility
 * class helpful for querying paging information related to an array.
 *
 * The class is designed to take in an array of values and an integer
 * indicating how many items will be allowed per each page.
 * The types of values contained within the collection/array are not relevant.
 */
use std::cmp::Ordering;

struct PaginationHelper<T> {
    items: Vec<T>,
    page_size: usize,
}

impl<T> PaginationHelper<T> {
    fn new(collection: Vec<T>, items_per_page: usize) -> Self {
        PaginationHelper {
            items: collection,
            page_size: items_per_page,
        }
    }

    fn item_count(&self) -> usize {
        self.items.len()
    }

    fn page_count(&self) -> usize {
        let has_part_page = self.items.len() % self.page_size != 0;

        self.items.len() / self.page_size + (if has_part_page { 1 } else { 0 })
    }

    fn page_item_count(&self, page_index: usize) -> Option<usize> {
        let last_page_index = self.page_count() - 1;

        match page_index.cmp(&last_page_index) {
            Ordering::Less => Some(self.page_size),
            Ordering::Equal => Some(self.items.len() % self.page_size),
            Ordering::Greater => None,
        }
    }

    fn page_index(&self, item_index: usize) -> Option<usize> {
        if item_index >= self.items.len() {
            None
        } else {
            Some((item_index - (item_index % self.page_size)) / self.page_size)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PaginationHelper;

    #[test]
    fn test_item_count() {
        let helper = PaginationHelper::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 3);
        assert_eq!(helper.item_count(), 11);
    }

    #[test]
    fn test_page_count() {
        let helper = PaginationHelper::new(vec!['a', 'b', 'c', 'd', 'e', 'f'], 4);
        assert_eq!(helper.page_count(), 2);
    }

    #[test]
    fn test_page_item_count() {
        let helper = PaginationHelper::new((1..=24).collect(), 10);
        assert_eq!(helper.page_item_count(1), Some(10));
        assert_eq!(helper.page_item_count(2), Some(4));
        assert_eq!(helper.page_item_count(0), Some(10));
    }

    #[test]
    fn test_page_index() {
        let helper = PaginationHelper::new((1..=24).collect(), 10);
        assert_eq!(helper.page_index(40), None);
        assert_eq!(helper.page_index(22), Some(2));
    }
}

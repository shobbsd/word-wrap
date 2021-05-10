use std::convert::TryInto;

struct PaginationHelper<T> {
    items: Vec<T>,
    page_limit: u32,
}

impl<T> PaginationHelper<T> {
    fn new(items: Vec<T>, page_limit: u32) -> Self {
        Self { items, page_limit }
    }

    fn page_count(&self) -> u32 {
        let num_of_items: u32 = self.items.len().try_into().unwrap();
        let page_limit = num_of_items / self.page_limit;
        if num_of_items.overflowing_rem(self.page_limit).0 > 0 {
            return page_limit + 1;
        }
        page_limit
    }

    fn item_count(&self) -> u32 {
        self.items.len().try_into().unwrap()
    }

    fn page_item_count(&self, page_index: u32) -> i32 {
        let num_of_items: u32 = self.items.len().try_into().unwrap();
        match page_index > (self.page_count() - 1) {
            true => -1,
            false => {
                if page_index == self.page_count() - 1 {
                    return num_of_items.overflowing_rem(self.page_limit).0 as i32;
                }
                self.page_limit as i32
            }
        }
    }

    fn page_index(&self, item_index: i32) -> i32 {
        let i32_page_limit = self.page_limit as i32;
        match (item_index > self.item_count() as i32, item_index < 0) {
            (true, _) => -1,
            (_, true) => -1,
            _ => {
                if item_index < i32_page_limit {
                    return 0;
                }

                let page_index = item_index / i32_page_limit;
                page_index
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_page_count() {
        let pag_helper = PaginationHelper::new(vec!['a', 'b', 'c', 'd', 'e', 'f'], 4);
        assert_eq!(pag_helper.page_count(), 2)
    }

    #[test]
    fn returns_item_count() {
        let pag_helper = PaginationHelper::new(vec!['a', 'b', 'c', 'd', 'e', 'f'], 4);
        assert_eq!(pag_helper.item_count(), 6)
    }

    #[test]
    fn returns_page_item_count() {
        let pag_helper = PaginationHelper::new(vec!['a', 'b', 'c', 'd', 'e', 'f'], 4);
        assert_eq!(pag_helper.page_item_count(0), 4);
        assert_eq!(pag_helper.page_item_count(1), 2);
        assert_eq!(pag_helper.page_item_count(2), -1);
    }

    #[test]
    fn returns_page_index() {
        let pag_helper = PaginationHelper::new(vec!['a', 'b', 'c', 'd', 'e', 'f'], 4);
        assert_eq!(pag_helper.page_index(5), 1);
        assert_eq!(pag_helper.page_index(2), 0);
        assert_eq!(pag_helper.page_index(20), -1);
        assert_eq!(pag_helper.page_index(10), -1);
    }
}

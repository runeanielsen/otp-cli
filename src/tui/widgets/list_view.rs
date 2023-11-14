pub struct ListView<T> {
    pub line_items: Vec<LineItem<T>>,
    pub current_index: usize,
    pub selected_callback: Box<dyn Fn(&T)>,
}

impl<T> ListView<T> {
    pub fn new(line_items: Vec<LineItem<T>>, selected_callback: Box<dyn Fn(&T)>) -> Self {
        Self {
            line_items,
            current_index: 0,
            selected_callback,
        }
    }

    pub fn set_line_items(&mut self, line_items: Vec<LineItem<T>>) {
        self.line_items = line_items;
    }

    pub fn select_next(&mut self) {
        // Do not select next if we are at the end of the list.
        if self.current_index < self.max_index() {
            self.current_index = self.current_index.saturating_add(1);
        }
    }

    pub fn select_prev(&mut self) {
        // Mark both the old and the new selected index to make sure they're redrawn.
        self.current_index = self.current_index.saturating_sub(1);
    }

    pub fn mark_selected_line_item(&mut self) {
        self.unmark_all_line_items();
        self.line_items[self.current_index].mark();
    }

    pub fn get_selected_line_item(&self) -> &LineItem<T> {
        &self.line_items[self.current_index]
    }

    fn unmark_all_line_items(&mut self) {
        for line_item in &mut self.line_items {
            if line_item.is_marked() {
                line_item.unmark();
            }
        }
    }

    fn max_index(&self) -> usize {
        self.line_items.len().saturating_sub(1)
    }
}

pub struct LineItem<T> {
    text: String,
    value: T,
    marked: bool,
}

impl<T> LineItem<T> {
    pub fn new(text: &str, value: T) -> Self {
        Self {
            text: text.to_string(),
            value,
            marked: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn unmark(&mut self) {
        self.marked = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _initialiation_of_list_view_correct_state() {
        let list_view: ListView<i32> = ListView::new(
            vec![
                LineItem::new("First", 1),
                LineItem::new("Second", 2),
                LineItem::new("Third", 3),
            ],
            Box::new(|_| {}),
        );

        // Checks that the initial line items are not being modified on initialization.
        assert_eq!(list_view.line_items[0].text, "First");
        assert_eq!(list_view.line_items[0].value, 1);
        assert_eq!(list_view.line_items[1].text, "Second");
        assert_eq!(list_view.line_items[1].value, 2);
        assert_eq!(list_view.line_items[2].text, "Third");
        assert_eq!(list_view.line_items[2].value, 3);

        // Make sure that the initial current_index is of value 0.
        assert_eq!(list_view.current_index, 0);
    }

    #[test]
    fn can_set_new_line_items() {
        let mut list_view = ListView::new(
            vec![
                LineItem::new("First", 1),
                LineItem::new("Second", 2),
                LineItem::new("Third", 3),
            ],
            Box::new(|_| {}),
        );

        list_view.set_line_items(vec![
            LineItem::new("NewFirst", 11),
            LineItem::new("NewSecond", 22),
            LineItem::new("NewThird", 33),
        ]);

        // Asserts that the list items has been updated to the new values.
        assert_eq!(list_view.line_items[0].text, "NewFirst");
        assert_eq!(list_view.line_items[0].value, 11);
        assert_eq!(list_view.line_items[1].text, "NewSecond");
        assert_eq!(list_view.line_items[1].value, 22);
        assert_eq!(list_view.line_items[2].text, "NewThird");
        assert_eq!(list_view.line_items[2].value, 33);
    }

    #[test]
    fn can_select_next() {
        let mut list_view = ListView::new(
            vec![
                LineItem::new("First", 1),
                LineItem::new("Second", 2),
                LineItem::new("Third", 3),
            ],
            Box::new(|_| {}),
        );

        list_view.select_next();

        // The selected index was incremented once, meaning the current index
        // should be 1.
        assert_eq!(list_view.current_index, 1);
    }

    #[test]
    fn can_select_previous() {
        let mut list_view = ListView::new(
            vec![
                LineItem::new("First", 1),
                LineItem::new("Second", 2),
                LineItem::new("Third", 3),
            ],
            Box::new(|_| {}),
        );

        // Increase the selected item multiple times to make sure,
        // so we can make sure that the selected index has changed.
        list_view.select_next();
        list_view.select_next();

        // Select the previos index.
        list_view.select_prev();

        // The selected index was incremented two times, and decrement one,
        // meaning that the current index should be 1.
        assert_eq!(list_view.current_index, 1);
    }

    #[test]
    fn when_at_the_end_of_index_do_not_increment_on_select_next() {
        let mut list_view = ListView::new(
            vec![
                LineItem::new("First", 1),
                LineItem::new("Second", 2),
                LineItem::new("Third", 3),
            ],
            Box::new(|_| {}),
        );

        // Increment until we are at the end of the list.
        list_view.select_next();
        list_view.select_next();
        list_view.select_next();

        // Increment once more.
        list_view.select_next();

        // Assert that we are still at the end of the list.
        assert_eq!(list_view.current_index, 2);
    }

    #[test]
    fn when_at_the_start_of_the_list_do_not_decrement_on_select_prev() {
        let mut list_view = ListView::new(
            vec![
                LineItem::new("First", 1),
                LineItem::new("Second", 2),
                LineItem::new("Third", 3),
            ],
            Box::new(|_| {}),
        );

        // Select previous to validate that the selected index do not decrement.
        list_view.select_prev();

        // Assert that we are still at the end of the list.
        assert_eq!(list_view.current_index, 0);
    }

    #[test]
    fn can_mark_the_selected_line_item() {
        let mut list_view = ListView::new(
            vec![
                LineItem::new("First", 1),
                LineItem::new("Second", 2),
                LineItem::new("Third", 3),
            ],
            Box::new(|_| {}),
        );

        list_view.mark_selected_line_item();

        // Makes sure that the item has been marked.
        assert!(list_view.get_selected_line_item().is_marked());

        // Make sure that other items are not marked.
        // Only one item in the list be marked at a time.
        assert!(!list_view.line_items[1].is_marked());
        assert!(!list_view.line_items[2].is_marked());
    }

    #[test]
    fn can_unmark_the_selected_line_item() {
        let mut list_view = ListView::new(
            vec![
                LineItem::new("First", 1),
                LineItem::new("Second", 2),
                LineItem::new("Third", 3),
            ],
            Box::new(|_| {}),
        );

        // We mark so that we can assert that it has been unmarked after.
        list_view.mark_selected_line_item();

        // Makes sure that the item has been marked.
        assert!(list_view.get_selected_line_item().is_marked());

        // Unmark all items.
        list_view.unmark_all_line_items();

        // Assert that all lines items has been unmarked.
        for line_item in list_view.line_items {
            assert!(!line_item.is_marked());
        }
    }
}

pub struct ListView<T> {
    pub line_items: Vec<LineItem<T>>,
    pub current_index: usize,
    pub selected_callback: Box<dyn FnMut(&T)>,
}

impl<T> ListView<T> {
    pub fn new(line_items: Vec<LineItem<T>>, selected_callback: Box<dyn FnMut(&T)>) -> Self {
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
            self.line_items[self.current_index].mark_as_modified();
            self.current_index = self.current_index.saturating_add(1);
            self.line_items[self.current_index].mark_as_modified();
        }
    }

    pub fn select_prev(&mut self) {
        // Mark both the old and the new selected index to make sure they're redrawn.
        self.line_items[self.current_index].mark_as_modified();
        self.current_index = self.current_index.saturating_sub(1);
        self.line_items[self.current_index].mark_as_modified();
    }

    pub fn mark_selected_line_item(&mut self) {
        self.unmark_all_line_items();
        self.line_items[self.current_index].mark();
    }

    fn unmark_all_line_items(&mut self) {
        for line_item in &mut self.line_items {
            if line_item.marked {
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
    modified: bool,
    marked: bool,
}

impl<T> LineItem<T> {
    pub fn new(text: &str, value: T) -> Self {
        Self {
            text: text.to_string(),
            value,
            modified: true,
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
        self.mark_as_modified();
    }

    pub fn unmark(&mut self) {
        self.marked = false;
        self.mark_as_modified();
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn mark_as_modified(&mut self) {
        self.modified = true;
    }

    pub fn mark_as_unmodified(&mut self) {
        self.modified = false;
    }
}

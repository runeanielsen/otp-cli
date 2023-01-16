use std::io::Write;

use crossterm::{
    cursor,
    event::{Event, KeyCode},
    queue,
    style::{self, Stylize},
};

pub struct ListView<T> {
    line_items: Vec<LineItem<T>>,
    current_index: usize,
    selected_callback: Box<dyn FnMut(&T)>,
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

    pub fn display<W>(&mut self, w: &mut W)
    where
        W: Write,
    {
        for (index, line) in self.line_items.iter_mut().enumerate() {
            if line.modified {
                if index == self.current_index {
                    queue!(w, style::PrintStyledContent(line.text.clone().blue())).unwrap();
                } else {
                    queue!(w, style::Print(line.text.clone())).unwrap();
                };

                // After we have drawed it, we set it to no longer modified
                // because we do not want to draw it again.
                line.modified = false;
            }

            queue!(w, cursor::MoveToNextLine(1)).unwrap();
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        if event == &Event::Key(KeyCode::Char('j').into()) {
            self.select_next();
        } else if event == &Event::Key(KeyCode::Char('k').into()) {
            self.select_prev();
        } else if event == &Event::Key(KeyCode::Enter.into()) {
            (self.selected_callback)(&self.line_items[self.current_index].value);
        }
    }

    fn select_next(&mut self) {
        // Do not select next if we are at the end of the list.
        if self.current_index < self.max_index() {
            // Mark the last selection for redraw.
            self.line_items[self.current_index].modified = true;

            self.current_index = self.current_index.saturating_add(1);

            // Mark the new selection for redraw.
            self.line_items[self.current_index].modified = true;
        }
    }

    fn select_prev(&mut self) {
        // Mark the last selection for redraw.
        self.line_items[self.current_index].modified = true;

        self.current_index = self.current_index.saturating_sub(1);

        // Mark the new selection for redraw.
        self.line_items[self.current_index].modified = true;
    }

    fn max_index(&self) -> usize {
        self.line_items.len().saturating_sub(1)
    }
}

pub struct LineItem<T> {
    text: String,
    value: T,
    modified: bool,
}

impl<T> LineItem<T> {
    pub fn new(text: &str, value: T) -> Self {
        Self {
            text: text.to_string(),
            value,
            modified: true,
        }
    }
}

#[cfg(test)]
mod tests {}

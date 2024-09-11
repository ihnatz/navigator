use crate::config::{Menu, MenuItem};

pub struct State<'a> {
    pub current_cursor: usize,
    pub current_item_id: usize,
    pub menu: &'a Menu,
}

impl State<'_> {
    pub fn current_item(&self) -> &MenuItem {
        &self.menu.items[self.current_item_id]
    }

    pub fn next_level(&self) -> impl Iterator<Item = &MenuItem> + '_ {
        self.current_item()
            .next_level
            .iter()
            .map(|&idx| &self.menu.items[idx])
    }

    pub fn move_down(&mut self) {
        let max = self.next_level().count() - 1;
        self.current_cursor = self.current_cursor.saturating_add(1).min(max);
    }

    pub fn move_up(&mut self) {
        self.current_cursor = self.current_cursor.saturating_sub(1).max(0);
    }
}

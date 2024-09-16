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

    pub fn is_terminating(&mut self) -> bool {
        self.pressed_item().value.is_some()
    }

    pub fn go_inside(&mut self) {
        self.current_item_id = self.pressed_item_id();
        self.current_cursor = 0;
    }

    pub fn go_outside(&mut self) {
        let cursor_at = self.parent_item().next_level.iter().position(|&x| x == self.current_item_id).unwrap_or(0);
        self.current_item_id = self.current_item().parent_id;
        self.current_cursor = cursor_at;
    }

    pub fn pressed_item(&self) -> &MenuItem {
        &self.menu.items[self.pressed_item_id()]
    }

    pub fn parent_item(&self) -> &MenuItem {
        &self.menu.items[self.current_item().parent_id]
    }

    fn pressed_item_id(&self) -> usize {
        self.current_item().next_level[self.current_cursor]
    }
}

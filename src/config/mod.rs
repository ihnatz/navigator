use serde_json::Value;
use std::env;
use std::fs::File;

#[derive(Debug)]
struct MenuItem {
    title: String,
    value: Option<String>,
    next_level: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct Menu {
    items: Vec<MenuItem>,
}

impl Menu {
    #[must_use]
    pub fn read_config() -> Result<Menu, Box<dyn std::error::Error>> {
        let config_path =
            env::var("NAVIGATOR_CONFIG").expect("Can't find value of NAVIGATOR_CONFIG");
        let file = File::open(config_path)?;
        let value: Value = serde_json::from_reader(file)?;

        let mut menu = Menu::default();
        let root = MenuItem {
            title: "root".to_string(),
            value: None,
            next_level: Vec::new(),
        };
        menu.items.push(root);

        menu.parse_json(&value, 0);
        menu.print_menu(None, None);

        Ok(menu)
    }

    fn parse_json(&mut self, value: &Value, parent_index: usize) {
        match value {
            Value::Object(map) => {
                for (key, val) in map {
                    let new_item = MenuItem {
                        title: key.to_string(),
                        value: None,
                        next_level: Vec::new(),
                    };
                    let new_index = self.items.len();
                    self.items.push(new_item);
                    self.items[parent_index].next_level.push(new_index);
                    self.parse_json(&val, new_index)
                }
            }
            Value::String(s) => {
                self.items[parent_index].value = Some(s.to_string());
            }
            _ => {}
        }
    }

    fn print_menu(&self, node: Option<&MenuItem>, level: Option<usize>) {
        if self.items.len() == 0 {
            return;
        }
        let current_node = node.unwrap_or(self.items.first().unwrap());
        let indent = level.unwrap_or(0) * 2;
        let mut item = " ".repeat(indent) + &current_node.title;

        if let Some(value) = &current_node.value {
            item = format!("{item} : {value}");
        }

        println!("{}", item);
        for &subitem_idx in &current_node.next_level {
            let subitem = &self.items[subitem_idx];
            self.print_menu(Some(subitem), Some(level.unwrap_or(0) + 1));
        }
    }
}

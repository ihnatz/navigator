use serde_json::Value;
use std::env;
use std::fmt;
use std::fs::File;

#[derive(Debug)]
pub struct MenuItem {
    pub title: String,
    pub value: Option<String>,
    pub next_level: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct Menu {
    pub items: Vec<MenuItem>,
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
}

const INDENTATION_SPACES: usize = 2;

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_level(
            f: &mut fmt::Formatter<'_>,
            items: &[MenuItem],
            node: &MenuItem,
            level: usize,
        ) -> fmt::Result {
            let indent = " ".repeat(level * INDENTATION_SPACES);
            let mut item = format!("{indent}{}", node.title);

            if let Some(ref value) = node.value {
                item = format!("{item} : {value}");
            }

            writeln!(f, "{}", item)?;

            for &subitem_idx in &node.next_level {
                let subitem = &items[subitem_idx];
                print_level(f, items, subitem, level + 1)?;
            }

            Ok(())
        }

        if let Some(first_item) = self.items.first() {
            print_level(f, &self.items, first_item, 0)?;
        }

        Ok(())
    }
}

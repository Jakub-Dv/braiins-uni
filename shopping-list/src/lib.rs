use std::cmp::max;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

type Amount = u64;

pub struct ShoppingList {
    path: Box<Path>,
    items: Vec<(String, Amount)>,
    save_on_drop: bool,
}

impl ShoppingList {
    pub fn from<S: AsRef<OsStr> + ?Sized>(p: &S) -> Result<Self, io::Error> {
        let path = Box::from(Path::new(p.as_ref()));
        let file = match Self::get_file(&path) {
            Ok(f) => f,
            Err(err) => return Err(err),
        };

        Ok(Self {
            path,
            items: Self::parse_file(&file),
            save_on_drop: false,
        })
    }

    fn get_file(path: &Path) -> Result<File, io::Error> {
        return OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path);
    }

    fn parse_file(file: &File) -> Vec<(String, Amount)> {
        let mut items: Vec<(String, Amount)> = Vec::new();
        for line in BufReader::new(file).lines() {
            match line {
                Ok(l) => {
                    let split_string: Vec<&str> = l.split(':').collect();
                    if split_string.len() < 2 {
                        // Skip invalid lines
                        continue;
                    } else {
                        let amount = match split_string[1].parse::<u64>() {
                            Ok(am) => am,
                            Err(_) => continue,
                        };
                        items.push((split_string[0].to_string(), amount));
                    }
                }
                Err(_) => continue,
            }
        }
        items
    }

    pub fn insert<I: ToString>(&mut self, item: I, amount: Amount) {
        self.items.push((item.to_string(), amount))
    }

    pub fn update<I: ToString>(&mut self, item: I, amount: Amount) {
        match self.items.iter_mut().find(|(i, _)| i == &item.to_string()) {
            Some(mut tuple) => tuple.1 = amount,
            None => self.insert(item, amount),
        }
    }

    pub fn remove<I: ToString>(&mut self, item: I) {
        let index = match self.items.iter().position(|(i, _)| i == &item.to_string()) {
            Some(i) => i,
            None => return,
        };
        self.items.remove(index);
    }

    pub fn get<I: ToString>(&self, item: I) -> Option<Amount> {
        self.items
            .iter()
            .find(|(i, _)| i == &item.to_string())
            .map(|(_, a)| *a)
    }

    pub fn save(&self) -> Result<(), String> {
        self.save_to(&self.path.as_ref())
    }

    pub fn save_to<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Result<(), String> {
        let file = match Self::get_file(&Box::from(Path::new(path.as_ref()))) {
            Ok(f) => f,
            Err(_) => return Err("Could not save list".to_string()),
        };
        let mut content = String::new();
        self.items
            .iter()
            .for_each(|(item, amount)| content.push_str(&format!("{}:{}\n", item, amount)));

        let mut buffer = BufWriter::new(file);
        match buffer.write(content.as_bytes()) {
            Ok(_) => match buffer.flush() {
                Ok(_) => Ok(()),
                Err(err) => Err(format!(
                    "Could not write to file: {}, err: {}",
                    Path::new(path.as_ref()).to_str().unwrap(),
                    err
                )),
            },
            Err(_) => Err(format!(
                "Could not write to file: {}",
                Path::new(path.as_ref()).to_str().unwrap()
            )),
        }
    }

    pub fn save_on_drop(&mut self) {
        self.save_on_drop = true
    }

    pub fn sort(&mut self) {
        self.items.sort_by(|(a, _), (b, _)| a.cmp(b))
    }

    pub fn dedup(&mut self) {
        self.sort();
        self.items.dedup_by(|(i1, a1), (i2, a2)| {
            if i1 == i2 {
                *a2 += *a1;
                true
            } else {
                false
            }
        })
    }
}

impl Drop for ShoppingList {
    fn drop(&mut self) {
        if self.save_on_drop {
            self.save().expect("Failed to save to file");
        }
    }
}

impl Display for ShoppingList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_item_len = match self
            .items
            .iter()
            .max_by(|(i1, _), (i2, _)| i1.len().cmp(&i2.len()))
        {
            Some((item, _)) => item.len(),
            None => 0,
        };
        let max_amount_len = match self.items.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)) {
            Some((_, val)) => format!("{}", val).len(),
            None => 0,
        };

        let item_len = max("Item name".len(), max_item_len);
        write!(f, "{:width$}|", "Item name", width = item_len).expect("Failed to print");
        let quantity_len = max("Quantity".len(), max_amount_len as usize);
        writeln!(f, "{:width$}", "Quantity", width = quantity_len).expect("Failed to print");
        writeln!(f, "{:->width$}", "", width = item_len + quantity_len + 1)
            .expect("Failed to print");

        self.items.iter().for_each(|(item, value)| {
            writeln!(
                f,
                "{:item_width$}|{:amount_width$}",
                item,
                value,
                item_width = item_len,
                amount_width = quantity_len
            )
            .expect("Failed to print")
        });
        Ok(())
    }
}

impl PartialEq<Self> for ShoppingList {
    fn eq(&self, other: &Self) -> bool {
        if self.path == other.path {
            true
        } else {
            self.items == other.items
        }
    }
}

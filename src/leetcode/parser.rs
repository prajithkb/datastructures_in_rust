//! Parser, a utility for parsing leet code inputs
use atoi::FromRadix10SignedChecked;
use std::{array::IntoIter, collections::VecDeque, fmt::Display, fs::File, io::Read};

/// A parser to consume inputs from Leet code
#[derive(Default)]
pub struct LeetCodeParser {
    arguments: VecDeque<Item>,
}

/// a parsed item.
/// There are only two kinds, a single item or a collection - separated by `,` between `[` and `]`
#[derive(Debug, PartialEq)]
pub enum Item {
    Single(String),
    Collection(Vec<Item>),
}

/// Error during conversion
#[derive(Debug)]
pub enum ItemConversionError {
    FailedToConvert,
    InvalidFormat(String),
}

impl Display for ItemConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

fn err(index: usize, ch: char, line: &str) -> ItemConversionError {
    ItemConversionError::InvalidFormat(format!(
        "Invalid format at index {} character {} for line {}",
        index, ch, line
    ))
}

impl Item {
    fn from_str(s: &str) -> Result<Item, ItemConversionError> {
        Item::parse(s.to_string())
    }
    /// Converts this instance into a collection of items, the shape of the items are decided by the mapper function
    fn into_vec<F, B>(self, mapper: F) -> Result<Vec<B>, ItemConversionError>
    where
        F: FnMut(Item) -> Result<B, ItemConversionError>,
    {
        self.into_iter()?.map(mapper).collect()
    }

    /// Converts this instance into an iterator
    fn into_iter(self) -> Result<std::vec::IntoIter<Item>, ItemConversionError> {
        if let Item::Collection(items) = self {
            Ok(items.into_iter())
        } else {
            Err(ItemConversionError::FailedToConvert)
        }
    }

    /// Converts this item into number
    fn into_num<B: FromRadix10SignedChecked>(self) -> Result<B, ItemConversionError> {
        atoi::atoi::<B>(self.as_bytes()?).ok_or(ItemConversionError::FailedToConvert)
    }

    /// Converts the item using the supplied mapper function
    fn map<F, B>(self, mut mapper: F) -> Result<B, ItemConversionError>
    where
        F: FnMut(Item) -> Result<B, ItemConversionError>,
    {
        mapper(self)
    }

    /// Converts it into string, fails for a collection
    fn into_string(self) -> Result<String, ItemConversionError> {
        if let Item::Single(item) = self {
            Ok(item)
        } else {
            Err(ItemConversionError::FailedToConvert)
        }
    }

    /// Converts it into slice of bytes (u8), fails for a collection
    fn as_bytes(&self) -> Result<&[u8], ItemConversionError> {
        if let Item::Single(item) = self {
            Ok(item.as_bytes())
        } else {
            Err(ItemConversionError::FailedToConvert)
        }
    }

    /// Creates an Item from a given String
    fn parse(line: String) -> Result<Item, ItemConversionError> {
        // A stack of collections.
        // We need a stack to keep track of recursive collections
        let mut items: VecDeque<Item> = VecDeque::new();
        for (i, ch) in line.char_indices() {
            match ch {
                // Start of a new collection, create a new one and add it to items stack
                '[' => items.push_back(Item::Collection(vec![])),
                ']' => {
                    // end of the current collection
                    if let Some(item) = items.pop_back() {
                        if let Some(Item::Collection(items)) = items.back_mut() {
                            items.push(item);
                        } else {
                            return Err(err(i, ch, &line));
                        }
                    } else {
                        return Err(err(i, ch, &line));
                    }
                }
                ',' => {
                    let v = items.pop_back();
                    if let Some(item) = v {
                        if let Item::Collection(items) = items.back_mut().expect("items") {
                            items.push(item);
                        } else {
                            return Err(err(i, ch, &line));
                        }
                    } else {
                        return Err(err(i, ch, &line));
                    }
                }
                _ => {
                    if let Some(Item::Single(chars)) = items.back_mut() {
                        chars.push(ch);
                    } else {
                        // create a new string and add it
                        let mut characters = String::new();
                        characters.push(ch);
                        items.push_back(Item::Single(characters));
                    }
                }
            }
        }
        Ok(items.pop_back().expect("Empty Item"))
    }
}

impl LeetCodeParser {
    pub fn new() -> Result<Self, ItemConversionError> {
        let mut file =
            File::open("/Users/kprajith/workspace/rust/datastructures-in-rust/src/input.txt")
                .expect("Open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Read");
        let arguments: Result<VecDeque<Item>, _> = contents
            .split('\n')
            .into_iter()
            .map(Item::from_str)
            .collect();
        Ok(LeetCodeParser::new_with_arguments(arguments?))
    }

    pub fn new_with_arguments(arguments: VecDeque<Item>) -> Self {
        LeetCodeParser { arguments }
    }

    pub fn next_item(&mut self) -> Option<Item> {
        self.arguments.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::parser::Item;

    use super::ItemConversionError;

    #[test]
    fn items_initialized_correctly() -> Result<(), ItemConversionError> {
        let item: Item = Item::from_str("[12,23,[1,2]]")?;
        assert_eq!(
            item,
            Item::Collection(vec![
                Item::Single("12".into()),
                Item::Single("23".into()),
                Item::Collection(vec![Item::Single("1".into()), Item::Single("2".into())])
            ])
        );
        let item: Item = Item::from_str("[[12,23,[1,2]]]")?;
        assert_eq!(
            item,
            Item::Collection(vec![Item::Collection(vec![
                Item::Single("12".into()),
                Item::Single("23".into()),
                Item::Collection(vec![Item::Single("1".into()), Item::Single("2".into())])
            ])])
        );
        let item: Item = Item::from_str("[12,23]")?;
        assert_eq!(
            item,
            Item::Collection(vec![Item::Single("12".into()), Item::Single("23".into())])
        );
        let item: Item = Item::from_str("1234")?;
        assert_eq!(item, Item::Single("1234".into()));
        let item: Item = Item::from_str("[[12,23],[45,78]]")?;
        assert_eq!(
            item,
            Item::Collection(vec![
                Item::Collection(vec![Item::Single("12".into()), Item::Single("23".into())]),
                Item::Collection(vec![Item::Single("45".into()), Item::Single("78".into())])
            ])
        );
        Ok(())
    }

    #[test]
    fn invalid_format_fails() {
        let item = Item::from_str("12,23,[1,2]]");
    }

    #[test]
    fn converstion_works() -> Result<(), ItemConversionError> {
        let item: Item = Item::from_str("[12,23,1,2]")?;
        let v: Vec<i32> = item.into_vec(Item::into_num)?;
        assert_eq!(v, vec![12, 23, 1, 2]);
        let item: Item = Item::from_str("[[12,23],[1,2]]")?;
        let v: Vec<Vec<i32>> = item.into_vec(|i| i.into_vec(Item::into_num))?;
        assert_eq!(v, vec![vec![12, 23], vec![1, 2]]);
        let item: Item = Item::from_str("1234")?;
        let v: i32 = item.map(Item::into_num)?;
        assert_eq!(v, 1234);
        let item: Item = Item::from_str("1234")?;
        assert_eq!(item.into_string()?, "1234".to_string());
        Ok(())
    }
}

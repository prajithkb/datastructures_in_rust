//! Parser, a utility for parsing leet code inputs
use atoi::FromRadix10SignedChecked;
use std::{collections::VecDeque, fmt::Display, fs::File, io::Read};

/// A parser to consume inputs from Leet code
#[derive(Default)]
pub struct LeetCodeParser {
    arguments: VecDeque<Item>,
}

/// a parsed item.
/// There are only two kinds, a single item or a collection - separated by `,` between `[` and `]`
#[derive(Debug, PartialEq)]
pub enum Item {
    /// A single item (String)
    Single(String),
    /// A collection of strings
    Collection(Vec<Item>),
}

/// Error during conversion
#[derive(Debug, PartialEq)]
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
    let mut highlighted_string = String::new();
    for (i, c) in line.char_indices() {
        if i == index {
            highlighted_string.push('{');
            highlighted_string.push(c);
            highlighted_string.push('}');
        } else {
            highlighted_string.push(c);
        }
    }
    ItemConversionError::InvalidFormat(format!(
        "Invalid character `{}` at index {} => {}",
        ch, index, highlighted_string
    ))
}

fn add_new_string(ch: char, items: &mut VecDeque<Item>) {
    items.push_back(Item::Single(ch.to_string()))
}

fn append_to_collection(
    items: &mut VecDeque<Item>,
    index: usize,
    ch: char,
    line: &str,
    number_of_collections: &mut i32,
) -> Result<(), ItemConversionError> {
    if let Some(item) = items.pop_back() {
        match items.back_mut() {
            Some(Item::Collection(prev_items)) => {
                if *number_of_collections <= 0 {
                    return Err(err(index, ch, line));
                }
                prev_items.push(item);
            }
            _ => return Err(err(index, ch, line)),
        }
    } else {
        return Err(err(index, ch, line));
    }
    Ok(())
}

fn create_or_append_to_string(
    items: &mut VecDeque<Item>,
    index: usize,
    ch: char,
    line: &str,
    number_of_collections: &mut i32,
) -> Result<(), ItemConversionError> {
    match items.back_mut() {
        // If the last item is a string, add to it
        Some(Item::Single(chars)) => chars.push(ch),
        // If empty, create a new string
        None => add_new_string(ch, items),
        Some(Item::Collection(_)) => {
            if *number_of_collections <= 0 {
                return Err(err(index, ch, line));
            }
            add_new_string(ch, items);
        }
    };
    Ok(())
}

impl Item {
    fn from_str(s: &str) -> Result<Item, ItemConversionError> {
        Item::parse(s.to_string())
    }
    /// Converts this instance into a collection of items, the shape of the items are decided by the mapper function
    pub fn into_vec<F, B>(self, mapper: F) -> Result<Vec<B>, ItemConversionError>
    where
        F: FnMut(Item) -> Result<B, ItemConversionError>,
    {
        self.into_iterator()?.map(mapper).collect()
    }

    /// Converts this instance into an iterator
    pub fn into_iterator(self) -> Result<std::vec::IntoIter<Item>, ItemConversionError> {
        match self {
            Item::Collection(items) => Ok(items.into_iter()),
            _ => Err(ItemConversionError::FailedToConvert),
        }
    }

    /// Converts this item into number
    pub fn into_num<B: FromRadix10SignedChecked>(self) -> Result<B, ItemConversionError> {
        atoi::atoi::<B>(self.as_bytes()?).ok_or(ItemConversionError::FailedToConvert)
    }

    /// Converts the item using the supplied mapper function
    pub fn map<F, B>(self, mut mapper: F) -> Result<B, ItemConversionError>
    where
        F: FnMut(Item) -> Result<B, ItemConversionError>,
    {
        mapper(self)
    }

    /// Converts it into string, fails for a collection
    pub fn into_string(self) -> Result<String, ItemConversionError> {
        if let Item::Single(item) = self {
            Ok(item)
        } else {
            Err(ItemConversionError::FailedToConvert)
        }
    }

    /// Converts it into slice of bytes (u8), fails for a collection
    pub fn as_bytes(&self) -> Result<&[u8], ItemConversionError> {
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
        let mut number_of_collections = 0;
        for (i, ch) in line.char_indices() {
            dbg!(&ch, &items);
            match ch {
                // Start of a new collection, create a new one and add it to items stack
                '[' => {
                    items.push_back(Item::Collection(vec![]));
                    number_of_collections += 1;
                }
                // Append operation for a collection
                ']' => {
                    append_to_collection(&mut items, i, ch, &line, &mut number_of_collections)?;
                    number_of_collections -= 1;
                }
                // do something to mark this collection as closed.
                ',' => append_to_collection(&mut items, i, ch, &line, &mut number_of_collections)?,
                // This is a string
                _ => create_or_append_to_string(
                    &mut items,
                    i,
                    ch,
                    &line,
                    &mut number_of_collections,
                )?,
            }
        }
        if number_of_collections > 0 {
            return Err(ItemConversionError::InvalidFormat(format!(
                "Collection not closed for {}",
                line
            )));
        }
        items.pop_back().ok_or_else(|| {
            ItemConversionError::InvalidFormat(format!(
                "Invalid format for line={}, cannot be empty",
                line
            ))
        })
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
        assert_eq!(
            item,
            Err(ItemConversionError::InvalidFormat(
                "Invalid character `,` at index 2 => 12{,}23,[1,2]]".into()
            ))
        );

        let item = Item::from_str("[12,23,1]2]]");
        assert_eq!(
            item,
            Err(ItemConversionError::InvalidFormat(
                "Invalid character `2` at index 9 => [12,23,1]{2}]]".into()
            ))
        );
        let item = Item::from_str("[12,23,,]");
        assert_eq!(
            item,
            Err(ItemConversionError::InvalidFormat(
                "Invalid character `,` at index 7 => [12,23,{,}]".into()
            ))
        );
        let item = Item::from_str("[[]");
        assert_eq!(
            item,
            Err(ItemConversionError::InvalidFormat(
                "Collection not closed for [[]".into()
            ))
        );
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

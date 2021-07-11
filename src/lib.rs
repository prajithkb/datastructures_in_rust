//! My attempt to implement advanced data structures in Rust

use std::{fs::File, io::Read};

pub mod intervals;

/// A parser to consume inputs from Leet code
#[derive(Default)]
pub struct LeetCodeParser {
    arguments: Vec<String>,
}

impl LeetCodeParser {
    pub fn new() -> Self {
        let mut file =
            File::open("/Users/kprajith/workspace/rust/datastructures-in-rust/src/input.txt")
                .expect("Open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Read");
        let arguments: Vec<String> = contents
            .split('\n')
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        LeetCodeParser { arguments }
    }

    pub fn next_arg_as_string(&mut self) -> Option<String> {
        if self.arguments.is_empty() {
            return None;
        }
        let v = self.arguments.remove(0);
        Some(v)
    }

    pub fn next_arg_as_num<T: atoi::FromRadix10SignedChecked>(&mut self) -> Option<T> {
        if let Some(arg) = self.next_arg_as_string() {
            return Some(atoi::atoi::<T>(arg.as_bytes()).expect("atoi"));
        }
        None
    }

    pub fn next_arg_as_vec_of_strings(&mut self) -> Option<Vec<String>> {
        if self.arguments.is_empty() {
            return None;
        }
        let items = self.arguments.remove(0);
        let result: Vec<String> = items[1..items.len() - 1]
            .split(',')
            .map(|s| s.to_string())
            .collect();
        Some(result)
    }

    pub fn next_arg_as_vec_of_nums<T: atoi::FromRadix10SignedChecked>(&mut self) -> Option<Vec<T>> {
        if let Some(v) = self.next_arg_as_vec_of_strings() {
            let result: Vec<T> = v
                .into_iter()
                .map(|i| atoi::atoi::<T>(i.as_bytes()).expect("atoi"))
                .collect();
            return Some(result);
        }
        None
    }
}

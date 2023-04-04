use std::{cmp, ops::Range};

use crossterm::cursor::MoveRight;

use crate::helpers::completion::{keywords, seperators};

use super::output::print_below_current;

#[derive(Debug)]
pub struct CommandBuffer {
    pub contents: String,
    pub index: usize,
}

impl CommandBuffer {
    pub fn new() -> CommandBuffer {
        CommandBuffer { contents: String::new(), index: 0 }
    }
    pub fn str_contents(&self) -> &str {
        &self.contents
    }

    pub fn push(&mut self, ch: char) {
        self.contents.push(ch);
        self.index += 1
    }

    pub fn push_str(&mut self, string: &str) {
        self.contents.push_str(string);
        self.index += string.len()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.contents.starts_with(s)
    }

    pub fn contains(&self, s: &str) -> bool {
        self.contents.contains(s)
    }

    pub fn clear(&mut self) {
        self.contents.clear();
        self.index = 0
    }

    pub fn pop(&mut self) -> Option<char> {
        if let Some(c) = self.contents.pop() {
            self.index -= 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn delete(&mut self) -> Option<char> {
        if self.index <= 0 || self.index > self.contents.len() {
            None
        } else {
            //remove character before
            self.index -= 1;
            Some(self.contents.remove(self.index))
        }
    }

    pub fn move_left(&mut self) -> bool {
        if self.index == 0 {
            false
        } else {
            self.index -= 1;
            true
        }
    }

    pub fn move_right(&mut self) -> bool {
        if self.index == self.contents.len() {
            false
        } else {
            self.index += 1;
            true
        }
    }

    pub fn distance_from_end(&self) -> usize {
        self.contents.len() - self.index
    }

    pub fn insert(&mut self, c: char) {
        self.contents.insert(self.index, c);
        self.index += 1
    }

    pub fn insert_str(&mut self, s: &str) {
        self.contents.insert_str(self.index, s);
        self.index += s.len()
    }

    pub fn get_word_index(&self) -> usize {
        let (left, _) = self.contents.split_at(self.index);
        let left_words = left.split_ascii_whitespace().count();
        if left_words == 0 {
            0
        } else {
            if left.ends_with(" ") {
                left_words
            } else {
                left_words - 1
            }
        }
    }

    pub fn str_contents_after_index(&self) -> &str {
        self.contents.split_at(self.index).1
    }

    pub fn words_current_and_after(&self) -> String {
        let (left, right) = self.contents.split_at(self.index);
        let word_left = if left.ends_with(" ") {
            ""
        } else {
            left.split_ascii_whitespace().last().unwrap_or("")
        };
        format!("{}{}", word_left, right)
    }

    pub fn get_context_and_after(&mut self) -> String {
        let keywords = keywords();
        let (current, _) = self.get_current_word();
        if keywords.iter().any(|k| k == &&current) {
            return self.words_current_and_after()
        }
        let index = self.index;
        for _ in 0..self.index {
            self.index -= 1;
            let (current, _) = self.get_current_word();
            if keywords.iter().any(|k| k == &&current) {
                let to_return = self.words_current_and_after();
                self.index = index;
                return to_return
            }
        }
        return self.contents.clone()
    }

    pub fn get_last_context(&mut self) -> String {
        let index = self.index;
        self.index = self.contents.len();
        let last_context = self.get_context_and_after();
        self.index = index;
        return last_context
    }

    //returns the amount needed to move right
    pub fn replace_current_word(&mut self, new: &str) -> (usize, usize) {
        let range = self.get_current_word().1;
        self.index = range.0 + new.len();
        self.contents.replace_range(Range {start: range.0, end: range.1}, new);
        (range.0, self.index)
    }

    pub fn get_current_word(&self) -> (String, (usize, usize)) {
        let (left, right) = self.contents.split_at(self.index);
        let left_part = if left.ends_with(" ") {
            ""
        } else {
            left.split_ascii_whitespace().last().unwrap_or("")
        };
        let right_part = if right.starts_with(" ") {
            ""
        } else {
            right.split_ascii_whitespace().next().unwrap_or("")
        };
        (format!("{}{}", left_part, right_part).to_owned(), (self.index - left_part.len(), self.index + right_part.len()))
    }

}
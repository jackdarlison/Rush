use std::ops::Range;

use crate::helpers::completion::keywords;

/// `CommandBuffer` holds a string and an index posistion within the string
#[derive(Debug)]
pub struct CommandBuffer {
    pub contents: String,
    pub index: usize,
}

impl CommandBuffer {
    /// Creates a new empty `CommandBuffer`
    pub fn new() -> CommandBuffer {
        CommandBuffer { contents: String::new(), index: 0 }
    }

    /// Returns the full contents of the `CommnadBuffer`
    pub fn str_contents(&self) -> &str {
        &self.contents
    }

    /// Appends a character 
    pub fn push(&mut self, ch: char) {
        self.contents.push(ch);
        self.index += 1
    }

    /// Appends a string
    pub fn push_str(&mut self, string: &str) {
        self.contents.push_str(string);
        self.index += string.len()
    }

    /// Checks the contents starts with a given pattern
    pub fn starts_with(&self, s: &str) -> bool {
        self.contents.starts_with(s)
    }

    /// Checks the contents contains a given pattern
    pub fn contains(&self, s: &str) -> bool {
        self.contents.contains(s)
    }

    /// Clears the `CommnadBuffer`
    pub fn clear(&mut self) {
        self.contents.clear();
        self.index = 0
    }

    /// Pops a character of the end of the contents, returning it if successful
    pub fn pop(&mut self) -> Option<char> {
        if let Some(c) = self.contents.pop() {
            self.index -= 1;
            Some(c)
        } else {
            None
        }
    }

    /// Deletes a character at the index posistion within the string
    pub fn delete(&mut self) -> Option<char> {
        if self.index <= 0 || self.index > self.contents.len() {
            None
        } else {
            //remove character before
            self.index -= 1;
            Some(self.contents.remove(self.index))
        }
    }

    /// Moves the index left one step
    pub fn move_left(&mut self) -> bool {
        if self.index == 0 {
            false
        } else {
            self.index -= 1;
            true
        }
    }

    /// Moves the index right one step
    pub fn move_right(&mut self) -> bool {
        if self.index == self.contents.len() {
            false
        } else {
            self.index += 1;
            true
        }
    }

    /// Retuns the distances of the index from the end of the contents
    pub fn distance_from_end(&self) -> usize {
        self.contents.len() - self.index
    }

    /// Inserts a character at the index posistion
    pub fn insert(&mut self, c: char) {
        self.contents.insert(self.index, c);
        self.index += 1
    }

    /// Inserts a string at the index posisiton
    pub fn insert_str(&mut self, s: &str) {
        self.contents.insert_str(self.index, s);
        self.index += s.len()
    }

    /// Retuns the word index of the current word the index is in
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

    /// Returns the contents after the index
    pub fn str_contents_after_index(&self) -> &str {
        self.contents.split_at(self.index).1
    }

    /// Returns the contents of the current word the index is in and after
    pub fn words_current_and_after(&self) -> String {
        let (left, right) = self.contents.split_at(self.index);
        let word_left = if left.ends_with(" ") {
            ""
        } else {
            left.split_ascii_whitespace().last().unwrap_or("")
        };
        format!("{}{}", word_left, right)
    }

    /// Returns the current context and after
    /// 
    /// The current context is defined as the words until and including the previous statement keyword from the current index
    pub fn get_context_and_after(&mut self) -> String {
        let keywords = keywords();
        let (current, _) = self.get_current_word();
        if keywords.iter().any(|k| *k == current) {
            return self.words_current_and_after()
        }
        let index = self.index;
        for _ in 0..self.index {
            self.index -= 1;
            let (current, _) = self.get_current_word();
            if keywords.iter().any(|k| *k == current) {
                let to_return = self.words_current_and_after();
                self.index = index;
                return to_return
            }
        }
        return self.contents.clone()
    }

    /// Returns the last context within the contents
    /// 
    /// The last context is defined as the words until and including the previous statement keyword from the end of the contents 
    pub fn get_last_context(&mut self) -> String {
        let index = self.index;
        self.index = self.contents.len();
        let last_context = self.get_context_and_after();
        self.index = index;
        return last_context
    }

    /// Replaces the current word the index in in with a new word
    /// 
    /// Returns the start and end indices of the new word
    pub fn replace_current_word(&mut self, new: &String) -> (usize, usize) {
        let range = self.get_current_word().1;
        self.index = range.0 + new.len();
        self.contents.replace_range(Range {start: range.0, end: range.1}, &new);
        (range.0, self.index)
    }

    /// Returns the current word that the index is in and its start and end indices
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
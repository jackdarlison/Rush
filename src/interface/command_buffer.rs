use std::cmp;

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

    pub fn str_contents_after_index(&self) -> &str {
        self.contents.split_at(self.index).1
    }
}
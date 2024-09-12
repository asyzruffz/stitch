use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::Chars;

use crate::projects::project::Project;
use crate::compilation::errors::CompilerError;

#[derive(Debug, Default)]
pub struct Source {
    pub path: Rc<str>,
    pub filename: Rc<str>,
    pub hash: Rc<[u8]>,
}

impl Source {
    pub fn new(path: &str, filename: &str, hash: &[u8]) -> Result<Self, CompilerError> {
        Ok(Self {
            path: path.into(),
            filename: filename.into(),
            hash: hash.into(),
        })
    }

    pub fn content(&self) -> Result<Rc<str>, CompilerError> {
        Ok(fs::read_to_string(self.full_path()?)?.as_str().into())
    }

    pub fn full_path(&self) -> Result<PathBuf, CompilerError> {
        let source_directory = Project::get_source_dir(false)?;
        let full_path = source_directory.join(self.path.as_ref());
        Ok(full_path)
    }
}

pub struct SourceBuffer<'a> {
    iter: Chars<'a>,
    peeked: Option<Option<char>>,
    sub: String,
}

impl<'a> From<Chars<'a>> for SourceBuffer<'a> {
    fn from(iter: Chars<'a>) -> Self {
        Self { iter, peeked: None, sub: String::new() }
    }
}

impl SourceBuffer<'_> {
    pub fn peek(&mut self) -> Option<&char> {
        let iter = &mut self.iter;
        self.peeked.get_or_insert_with(|| iter.next()).as_ref()
    }

    pub fn next_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        match self.next() {
            Some(matched) if func(&matched) => Some(matched),
            other => {
                // Since we called `self.next()`, we consumed `self.peeked`.
                assert!(self.peeked.is_none());
                self.peeked = Some(other);
                None
            }
        }
    }

    pub fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek_next(&mut self, target: char) -> bool {
        if self.is_at_end() {
            return false; 
        }
        
        match self.peek() {
            Some(&c) => c == target,
            None => false,
        }
    }
    
    pub fn match_next(&mut self, target: char) -> bool {
        match self.next_if(|&next| next == target) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn start(&mut self) {
        self.sub.clear();
    }

    pub fn extract(&self) -> String {
        self.sub.trim_end().to_string()
    }
}

// Peekable implementation. It must remember if a None has been seen in the `.peek()` method.
impl Iterator for SourceBuffer<'_> {
    type Item = char;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.take() {
            Some(ch) => {
                if let Some(ch) = ch {
                    self.sub.push(ch);
                }
                ch
            },
            None => match self.iter.next() {
                Some(ch) => {
                    self.sub.push(ch);
                    Some(ch)
                },
                None => None,
            },
        }
    }

    fn count(mut self) -> usize {
        match self.peeked.take() {
            Some(None) => 0,
            Some(Some(_)) => 1 + self.iter.count(),
            None => self.iter.count(),
        }
    }
    
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.peeked.take() {
            Some(None) => None,
            Some(v @ Some(_)) if n == 0 => v,
            Some(Some(_)) => self.iter.nth(n - 1),
            None => self.iter.nth(n),
        }
    }

    fn last(mut self) -> Option<Self::Item> {
        let peek_opt = match self.peeked.take() {
            Some(None) => return None,
            Some(v) => v,
            None => None,
        };
        self.iter.last().or(peek_opt)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let peek_len = match self.peeked {
            Some(None) => return (0, Some(0)),
            Some(Some(_)) => 1,
            None => 0,
        };
        let (lo, hi) = self.iter.size_hint();
        let lo = lo.saturating_add(peek_len);
        let hi = match hi {
            Some(x) => x.checked_add(peek_len),
            None => None,
        };
        (lo, hi)
    }
}

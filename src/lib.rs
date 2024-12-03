use std::usize;

#[derive(Debug, PartialEq)]
pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, needle: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter: needle,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // Does not move remainder.
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delim = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delim)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + 1))
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("Always returns at least one value")
}

#[test]
fn iter() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello, world", 'o'), "hell");
}

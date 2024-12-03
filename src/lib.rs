#[derive(Debug, PartialEq)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, needle: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter: needle,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // A delimiter may not happen at some point.
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // Why is there a bug here?
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[test]
fn iter() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    letters.eq(vec!["a", "b", "c", "d", "e"].into_iter());
}

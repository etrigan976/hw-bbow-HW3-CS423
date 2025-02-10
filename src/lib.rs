//! Big Bag Of Words
//!
//! The "Big Bag Of Words" is used in text analysis and
//! machine learning.  It reduces a text to a collection of
//! words, each with a count of the number of occurrences.
//!
//! This implementation uses zero-copy strings when
//! reasonably possible to improve performance and reduce
//! memory usage.
//!
//! Words are separated by whitespace, and consist of a
//! span of one or more consecutive letters (any Unicode
//! code point in the "letter" class) with no internal
//! punctuation: leading and trailing punctuation are
//! removed.
//!
//! For example, the text
//!
//! ```text
//! "It ain't over untïl it ain't, over."
//! ```
//!
//! contains the sequence of words `"It"`, `"over"`,
//! `"untïl"`, `"it"`, `"over"`.
//!
//! Words in the bag containing uppercase letters will be
//! represented by their lowercase equivalent.

use std::borrow::Cow;
use std::collections::BTreeMap;

/// Each key in this struct's map is a word in some
/// in-memory text document. The corresponding value is the
/// count of occurrences.
#[derive(Debug, Default, Clone)]
pub struct Bbow<'a>(BTreeMap<Cow<'a, str>, usize>);

fn is_word(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_alphabetic())
}

fn has_uppercase(word: &str) -> bool {
    word.chars().any(char::is_uppercase)
}

impl<'a> Bbow<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this BBOW.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up a BBOW covering
    /// multiple texts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new().extend_from_text("Hello world.");
    /// assert_eq!(2, bbow.len());
    /// assert_eq!(1, bbow.match_count("hello"));
    /// ```
    /// # Function Notes
    /// ## extend_from_text(mut self, target: &'a str) -> Self
    /// takes a string slice target and parses through it, adding each usable
    /// word to its instance. Words are determined as strings of alphabetic
    /// characters, with end-to-end punctuation removed. Words are converted
    /// entirely into their lowercase version. It returns the modified instance
    /// of itself.
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        for word in target.split_whitespace() {
            let word = word.trim_matches(|c: char| !c.is_alphabetic());
            if is_word(word) {
                let word = if has_uppercase(word) {
                    Cow::Owned(word.to_lowercase())
                } else {
                    Cow::Borrowed(word)
                };
                *self.0.entry(word).or_insert(0) += 1;
            }
        }
        self
    }

    /// Report the number of occurrences of the given
    /// `keyword` that are indexed by this BBOW. The keyword
    /// should be lowercase and not contain punctuation, as
    /// per the rules of BBOW: otherwise the keyword will
    /// not match and 0 will be returned.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("b b b-banana b");
    /// assert_eq!(3, bbow.match_count("b"));
    /// ```
    /// # Function Notes
    /// ## match_count(&self, keyword: &str) -> usize
    /// takes a keyword and returns the number of occurrences of the word
    /// in the BBOW. The keyword should be lowercase and not contain
    /// punctuation. If the keyword does not match, returns 0. uses is_word
    /// to check if the keyword is a word and then looks
    /// up the keyword in the internal bbow, returning the count or 0
    pub fn match_count(&self, keyword: &str) -> usize {
        if !is_word(keyword) {
            return 0;
        }
        self.0.get(keyword).cloned().unwrap_or(0)
    }

    pub fn words(&'a self) -> impl Iterator<Item = &'a str> {
        self.0.keys().map(|w| w.as_ref())
    }

    /// Count the overall number of words contained in this BBOW:
    /// multiple occurrences are considered separate.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(3, bbow.count());
    /// ```
    /// # Function Notes
    /// ## count(&self) -> usize
    /// counts the number of contained words in the bbow including 
    /// multi-occurance words
    pub fn count(&self) -> usize {
        self.0.values().sum()
    }

    /// Count the number of unique words contained in this BBOW,
    /// not considering number of occurrences.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(2, bbow.len());
    /// ```
    /// # Function Notes
    /// ## len(&self) -> usize
    /// counts number of unique contained words 
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is this BBOW empty?
    /// # Function Notes
    /// ## is_empty(&self) -> bool
    /// determines whether or not a bbow is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
/// # Testing Area
/// I tried to do pretty thorough tests
/// it was a lot of copy and pasting and 
/// tweaking the same test
/// i only tested extend_from_text and match_count because it didnt seem 
/// necesarry for something like count
#[cfg(test)]
mod tests {
    use super::*;
    /// extend_from_text testing
    #[test]
    fn test_extend_from_text_basic() {
        let bbow = Bbow::new().extend_from_text("Hello world.");
        assert_eq!(2, bbow.len());
        assert_eq!(1, bbow.match_count("hello"));
        assert_eq!(1, bbow.match_count("world"));
    }
    #[test]
    fn test_extend_from_text_with_punctuation() {
        let bbow = Bbow::new().extend_from_text("Hello, world! This is a test.");
        assert_eq!(6, bbow.len());
        assert_eq!(1, bbow.match_count("hello"));
        assert_eq!(1, bbow.match_count("world"));
        assert_eq!(1, bbow.match_count("this"));
        assert_eq!(1, bbow.match_count("is"));
        assert_eq!(1, bbow.match_count("a"));
        assert_eq!(1, bbow.match_count("test"));
    }
    #[test]
    fn test_extend_from_text_with_uppercase() {
        let bbow = Bbow::new().extend_from_text("Hello HELLO HeLLo");
        assert_eq!(1, bbow.len());
        assert_eq!(3, bbow.match_count("hello"));
    }
    #[test]
    fn test_extend_from_text_empty_string() {
        let bbow = Bbow::new().extend_from_text("");
        assert_eq!(0, bbow.len());
        assert_eq!(0, bbow.count());
    }
    #[test]
    fn test_extend_from_text_multiple_spaces() {
        let bbow = Bbow::new().extend_from_text("Hello    world");
        assert_eq!(2, bbow.len());
        assert_eq!(1, bbow.match_count("hello"));
        assert_eq!(1, bbow.match_count("world"));
    }
    #[test]
    fn test_extend_from_text_non_alphabetic() {
        let bbow = Bbow::new().extend_from_text("123 456 !@#");
        assert_eq!(0, bbow.len());
        assert_eq!(0, bbow.count());
    }
    /// match_count testing
    #[test]
    fn test_match_count_basic() {
        let bbow = Bbow::new().extend_from_text("apple apple banana");
        assert_eq!(2, bbow.match_count("apple"));
        assert_eq!(1, bbow.match_count("banana"));
        assert_eq!(0, bbow.match_count("orange"));
    }

    #[test]
    fn test_match_count_with_punctuation() {
        let bbow = Bbow::new().extend_from_text("apple, apple! banana.");
        assert_eq!(2, bbow.match_count("apple"));
        assert_eq!(1, bbow.match_count("banana"));
        assert_eq!(0, bbow.match_count("orange"));
    }

    #[test]
    fn test_match_count_with_uppercase() {
        let bbow = Bbow::new().extend_from_text("Apple apple BANANA");
        assert_eq!(2, bbow.match_count("apple"));
        assert_eq!(1, bbow.match_count("banana"));
        assert_eq!(0, bbow.match_count("orange"));
    }

    #[test]
    fn test_match_count_empty_string() {
        let bbow = Bbow::new().extend_from_text("");
        assert_eq!(0, bbow.match_count("apple"));
    }

    #[test]
    fn test_match_count_non_alphabetic() {
        let bbow = Bbow::new().extend_from_text("123 456 !@#");
        assert_eq!(0, bbow.match_count("123"));
        assert_eq!(0, bbow.match_count("456"));
        assert_eq!(0, bbow.match_count("!@#"));
    }

    #[test]
    fn test_match_count_invalid_keyword() {
        let bbow = Bbow::new().extend_from_text("apple apple banana");
        assert_eq!(0, bbow.match_count("apple!"));
        assert_eq!(0, bbow.match_count("banana1"));
        assert_eq!(0, bbow.match_count(""));
    }
}

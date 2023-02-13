
extern crate log;
extern crate pest;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Word;

#[derive(Debug, Clone, PartialEq)]
struct State {
    pos: usize,
    chars: Vec<char>,
    p: String,
    s: String,
}

impl State {
    fn new() -> State {
        State {
            pos: 0,
            chars: vec![],
            p: String::new(),
            s: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoubleMetaphone {
    pub primary: String,
    pub secondary: String,
}

/// Use Double Metaphone phonetic encoding to determine if two words rhyme.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// // Does rhyme
/// assert!(ttaw::metaphone::rhyme("far", "tar"));
/// assert!(ttaw::metaphone::rhyme("here", "near"));
///
/// // Does not rhyme
/// assert!(!ttaw::metaphone::rhyme("shopping", "cart"));
/// assert!(!ttaw::metaphone::rhyme("run", "uphill"));
/// ```
pub fn rhyme(a: &str, b: &str) -> bool {
    // sanity check, needing to sanity check seems fragile?
    if a.trim().is_empty() || b.trim().is_empty() {
        return false;
    }

    let a_phonetic = encoding(a);
    let b_phonetic = encoding(b);

    log::info!(
        "|{: ^10} | {: ^10} | {: ^10} |",
        a,
        a_phonetic.primary,
        a_phonetic.secondary
    );

    log::info!(
        "|{: ^10} | {: ^10} | {: ^10} |",
        b,
        b_phonetic.primary,
        b_phonetic.secondary
    );

    let mut a_phonetic_end_primary = a_phonetic.primary;
    if let Some(slice) = a_phonetic_end_primary.get(1..) {
        a_phonetic_end_primary = slice.to_string();
    }

    let mut a_phonetic_end_secondary = a_phonetic.secondary;

    if let Some(slice) = a_phonetic_end_secondary.get(1..) {
        a_phonetic_end_secondary = slice.to_string();
    }

    let mut b_phonetic_end_primary = b_phonetic.primary;

    if let Some(slice) = b_phonetic_end_primary.get(1..) {
        b_phonetic_end_primary = slice.to_string();
    }

    let mut b_phonetic_end_secondary = b_phonetic.secondary;

    if let Some(slice) = b_phonetic_end_secondary.get(1..) {
        b_phonetic_end_secondary = slice.to_string();
    }

    a_phonetic_end_primary == b_phonetic_end_primary
        || a_phonetic_end_primary == b_phonetic_end_secondary
        || a_phonetic_end_secondary == b_phonetic_end_primary
        || a_phonetic_end_secondary == b_phonetic_end_secondary
}

/// Use Double Metaphone phonetic encoding to determine if two words alliterate.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw;
// // Does alliterate
/// assert!(ttaw::metaphone::alliteration("bouncing", "bears"));
/// assert!(ttaw::metaphone::alliteration("snappy", "snails"));
///
/// // Does not alliterate
/// assert!(!ttaw::metaphone::alliteration("brown", "fox"));
/// assert!(!ttaw::metaphone::alliteration("lazy", "dog"));
/// ```
pub fn alliteration(a: &str, b: &str) -> bool {
    if Word::parse(Rule::vowel_first, a.get(..1).unwrap_or_default()).is_ok() {
        return false;
    }

    if Word::parse(Rule::vowel_first, b.get(..1).unwrap_or_default()).is_ok() {
        return false;
    }

    let a_phonetic = encoding(a);
    let b_phonetic = encoding(b);

    log::info!(
        "|{: ^10} | {: ^10} | {: ^10} |",
        a,
        a_phonetic.primary,
        a_phonetic.secondary
    );

    log::info!(
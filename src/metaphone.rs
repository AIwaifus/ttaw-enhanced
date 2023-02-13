
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
        "|{: ^10} | {: ^10} | {: ^10} |",
        b,
        b_phonetic.primary,
        b_phonetic.secondary
    );

    let mut a_phonetic_head_primary = a_phonetic.primary;

    if let Some(c) = a_phonetic_head_primary.get(..1) {
        a_phonetic_head_primary = c.to_string();
    }

    let mut a_phonetic_head_secondary = a_phonetic.secondary;

    if let Some(c) = a_phonetic_head_secondary.get(..1) {
        a_phonetic_head_secondary = c.to_string();
    }

    let mut b_phonetic_head_primary = b_phonetic.primary;

    if let Some(c) = b_phonetic_head_primary.get(..1) {
        b_phonetic_head_primary = c.to_string();
    }

    let mut b_phonetic_head_secondary = b_phonetic.secondary;

    if let Some(c) = b_phonetic_head_secondary.get(..1) {
        b_phonetic_head_secondary = c.to_string();
    }

    if a_phonetic_head_primary == b_phonetic_head_primary
        || a_phonetic_head_primary == b_phonetic_head_secondary
        || a_phonetic_head_secondary == b_phonetic_head_primary
        || a_phonetic_head_secondary == b_phonetic_head_secondary
    {
        return true;
    }

    false
}

/// Double Metaphone phonetic encoding.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// assert_eq!(ttaw::metaphone::encoding("Arnow").primary, "ARN");
/// assert_eq!(ttaw::metaphone::encoding("Arnow").secondary, "ARNF");
///
/// assert_eq!(ttaw::metaphone::encoding("detestable").primary, "TTSTPL");
/// assert_eq!(ttaw::metaphone::encoding("detestable").secondary, "TTSTPL");
/// ```
///
pub fn encoding(input: &str) -> DoubleMetaphone {
    let mut state = State::new();
    let word: String = input.to_uppercase() + "     ";

    state.chars = word.chars().collect::<Vec<char>>();

    if Word::parse(Rule::initial_exceptions, word.as_str()).is_ok() {
        state.pos += 1;
    }

    if let Some('X') = state.chars.first() {
        state.p += "S";
        state.s += "S";
        state.pos += 1
    }

    while let Some(c) = state.chars.get(state.pos) {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'Y' | 'À' | 'Ê' | 'É' => {
                vowel_case(&mut state);
            }

            'B' => {
                b_case(&mut state);
            }

            'Ç' => {
                c_cedilla_case(&mut state);
            }

            'C' => {
                c_case(&mut state);
            }

            'D' => {
                d_case(&mut state);
            }

            'F' => {
                f_case(&mut state);
            }

            'G' => {
                g_case(&mut state);
            }

            'H' => {
                h_case(&mut state);
            }

            'J' => {
                j_case(&mut state);
            }

            'K' => {
                k_case(&mut state);
            }

            'L' => {
                l_case(&mut state);
            }

            'M' => {
                m_case(&mut state);
            }

            'N' => {
                n_case(&mut state);
            }

            'Ñ' => {
                top_tilde_n_case(&mut state);
            }

            'P' => {
                p_case(&mut state);
            }

            'Q' => {
                q_case(&mut state);
            }

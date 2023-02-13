
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

            'R' => {
                r_case(&mut state);
            }

            'S' => {
                s_case(&mut state);
            }

            'T' => {
                t_case(&mut state);
            }

            'V' => {
                v_case(&mut state);
            }

            'W' => {
                w_case(&mut state);
            }

            'X' => {
                x_case(&mut state);
            }

            'Z' => {
                z_case(&mut state);
            }

            _ => state.pos += 1,
        }
    }

    DoubleMetaphone {
        primary: state.p,
        secondary: state.s,
    }
}

fn get_char_as_string(chars: &[char], pos: usize) -> String {
    match chars.get(pos) {
        Some(c) => c.to_string(),
        None => String::new(),
    }
}

fn get_substring(chars: &[char], start: usize, end: usize) -> String {
    match chars.get(start..end) {
        Some(s) => s.iter().collect::<String>(),
        None => String::new(),
    }
}

fn germanic(chars: &[char]) -> bool {
    Word::parse(Rule::germanic, chars.iter().collect::<String>().as_str()).is_ok()
}

fn slavo_germanic(chars: &[char]) -> bool {
    Word::parse(
        Rule::slavo_germanic,
        chars.iter().collect::<String>().as_str(),
    )
    .is_ok()
}

fn vowel_case(State { pos, p, s, .. }: &mut State) {
    if *pos == 0 {
        *p += "A";
        *s += "A";
    }

    *pos += 1;
}

fn b_case(State { pos, chars, p, s }: &mut State) {
    *p += "P";
    *s += "P";

    if let Some('B') = chars.get(*pos + 1) {
        *pos += 1;
    }

    *pos += 1;
}

fn c_cedilla_case(State { pos, p, s, .. }: &mut State) {
    *p += "S";
    *s += "S";
    *pos += 1;
}

fn c_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(pos.wrapping_sub(1)) == Some(&'A')
        && chars.get(*pos + 1) == Some(&'H')
        && chars.get(*pos + 2) != Some(&'I')
        && Word::parse(
            Rule::vowels,
            get_char_as_string(chars, pos.wrapping_sub(3)).as_str(),
        )
        .is_err()
        && (chars.get(*pos + 2) != Some(&'E')
            || get_substring(chars, pos.wrapping_sub(2), *pos + 4) == "BACHER"
            || get_substring(chars, pos.wrapping_sub(2), *pos + 4) == "MACHER")
    {
        *p += "K";
        *s += "K";
        *pos += 2;

        return;
    }

    if *pos == 0 && get_substring(chars, 1, 6) == "AESAR" {
        *p += "S";
        *s += "S";
        *pos += 2;

        return;
    }

    if get_substring(chars, *pos + 1, *pos + 4) == "HIA" {
        *p += "K";
        *s += "K";
        *pos += 2;

        return;
    }

    if let Some('H') = chars.get(*pos + 1) {
        if *pos > 0 && chars.get(*pos + 2) == Some(&'A') && chars.get(*pos + 3) == Some(&'E') {
            *p += "K";
            *s += "X";
            *pos += 2;

            return;
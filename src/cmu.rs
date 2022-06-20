extern crate pest;

use error::Error;
use metaphone::{Rule, Word};
use pest::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub struct CmuDict {
    dict: HashMap<String, Vec<Vec<String>>>,
}

impl CmuDict {
    /// Initialize the CmuDict with a path to the existing serialized CMU dictionary
    /// or a directoy containing it. If the dictionary doesn't exisit, it will be
    /// downloaded and serialized at the location specified by the path parameter.
    pub fn new(path: &str) -> Result<CmuDict, Error> {
        match from_json_file(Path::new(path)) {
            Ok(d) => Ok(CmuDict { dict: d }),
            Err(e) => Err(e),
        }
    }

    /// CMUdict phonetic encoding.
    ///
    /// ```rust
    /// extern crate ttaw;
    /// use ttaw::cmu::CmuDict;
    /// let cmudict = CmuDict::new("cmudict.json").unwrap();
    /// assert_eq!(
    ///     cmudict.encoding("permeability"),
    ///     Ok(Some(vec![vec![
    ///         "P".to_string(),
    ///         "ER0".to_string(),
    ///         "M".to_string(),
    ///         "IY2".to_string(),
    ///         "AH0".to_string(),
    ///         "B".to_string(),
    ///         "IH1".to_string(),
    ///         "L".to_string(),
    ///         "IH0".to_string(),
    ///         "T".to_string(),
    ///         "IY0".to_string()
    ///     ]]))
    /// );
    ///
    /// assert_eq!(
    ///     cmudict.encoding("unearthed"),
    ///     Ok(Some(vec![vec![
    ///         "AH0".to_string(),
    ///         "N".to_string(),
    ///         "ER1".to_string(),
    ///         "TH".to_string(),
    ///         "T".to_string()
    ///     ]]))
    /// );
    /// ```
    pub fn encoding(&self, w: &str) -> Result<Option<Vec<Vec<String>>>, Error> {
        Ok(self.dict.get(w).map(|v| v.to_vec()))
    }

    /// Use CMUdict phonetic encoding to determine if two words rhyme.
    ///
    /// ```rust
    /// extern crate ttaw;
    /// use ttaw::cmu::CmuDict;
    /// let cmudict = CmuDict::new("cmudict.json").unwrap();
    /// // Does rhyme
    /// assert!(cmudict.rhyme("hissed", "mist").unwrap());
    /// assert!(cmudict.rhyme("tryst", "wrist").unwrap());
    ///
    /// // Does not rhyme
    /// assert!(!cmudict.rhyme("red", "Edmund").unwrap());
    /// assert!(!cmudict.rhyme("comfy", "chair").unwrap());
    /// ```
    pub fn rhyme(&self, a: &str, b: &str) -> Result<bool, Error> {
        if let (Some(phones_a), Some(phones_b)) = (
            self.dict.get(a.to_string().to_lowercase().trim()),
            self.dict.get(b.to_string().to_lowercase().trim()),
        ) {
            return Ok(eval_rhyme(phones_a, phones_b));
        }

        Ok(false)
    }

    /// Use CMUdict phonetic encoding to determine if two words alliterate.
    ///
    /// ```rust
    /// extern crate ttaw;
    /// use ttaw::cmu::CmuDict;
    /// let cmudict = CmuDict::new("cmudict.json").unwrap();
    // // Does alliterate
    /// assert!(cmudict.alliteration("bouncing", "bears").unwrap());
    /// assert!(cmudict.alliteration("snappy", "snails").unwrap());
    ///
    /// // Does not alliterate
    /// assert!(!cmudi
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
    ///         "N".to_
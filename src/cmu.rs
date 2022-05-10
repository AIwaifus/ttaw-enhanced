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
    /// Initialize the CmuDict with a pa
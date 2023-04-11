[![.github/workflows/ci.yml](https://github.com/AIwaifus/ttaw-enhanced/workflows/.github/workflows/ci.yml/badge.svg?branch=main)](https://github.com/AIwaifus/ttaw-enhanced/actions)
[![codecov](https://codecov.io/gh/AIwaifus/ttaw-enhanced/branch/main/graph/badge.svg?token=7I6VUOOLC2)](https://codecov.io/gh/AIwaifus/ttaw-enhanced)
[![Crates.io Version](https://img.shields.io/crates/v/ttaw-enhanced.svg)](https://crates.io/crates/ttaw-enhanced)
[![Crates.io](https://img.shields.io/crates/d/ttaw-enhanced.svg)](https://crates.io/crates/ttaw-enhanced)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

# ttaw-enhanced
Talking to A Wall - An enhanced natural language processing library offering a modular and flexible approach.

## Functionality
- Determine if two words rhyme using the Double Metaphone phonetic encoding
- Determine if two words rhyme using CMUdict phonetic encoding

- Determine if two words alliterate using the Double Metaphone phonetic encoding
- Determine if two words alliterate using CMUdict phonetic encoding

- Get the CMUdict phonetic encoding of a word
- Get the Double Metaphone phonetic encoding of a word (port of [words/double-metahone](https://github.com/words/double-metaphone) library)

## Rhyme
```rust
extern crate ttaw-enhanced;
use ttaw-enhanced;

// Initialize the CmuDict with a path to the existing serialized CMU dictionary
// or a directory containing it. If the dictionary doesn't exist, it will be
// downloaded and serialized at the location specified by the path parameter.
let cmudict = ttaw-enhanced::cmu::CmuDict::new("cmudict.json").unwrap();

assert_eq!(Ok(true), cmudict.rhyme("far", "tar"));
assert_eq!(Ok(true), ttaw-enhanced::metaphone::rhyme("far", "tar"));

assert_eq!(Ok(false), cmudict.rhyme("shopping", "cart"));
assert_eq!(Ok(false), ttaw-enhanced::metaphone::rhyme("shopping", "cart"));

// Deviations in cmu and metaphone
assert_eq!(true, ttaw-enhanced::metaphone::rhyme("hear", "near"));
assert_eq!(Ok(false), cmudict.rhyme("hear", "near"));
```

## Alliteration
```rust
extern crate ttaw-enhanced;
use ttaw-enhanced;

// Initialize the CmuDict with a path to the existing serialized CMU dictionary
// or a directory containing it. If the dictionary doesn't exist, it will be
// downloaded and serialized at the location specified by the path parameter.
let cmudict = ttaw-enhanced::cmu::CmuDict::new
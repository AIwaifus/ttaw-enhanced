
extern crate ttaw;

use ttaw::metaphone::{alliteration, encoding, rhyme, DoubleMetaphone};

#[test]
fn alliterates_with_spaces() {
    assert!(alliteration("bouncing", "  bears"));
    assert!(alliteration("bouncing", "bears  "));
    assert!(alliteration(" bouncing", "bears"));
    assert!(alliteration("bouncing  ", "bears"));
}

#[test]
fn alliterates_with_caps() {
    assert!(alliteration("Bouncing", "  bears"));
    assert!(alliteration("bouncing", "Bears  "));
    assert!(alliteration(" bouncinG", "bEars"));
    assert!(alliteration("bouncing  ", "beaRs"));
}

#[test]
fn alliterates() {
    assert!(alliteration("bouncing", "bears"));
    assert!(alliteration("bounding", "bears"));
    assert!(alliteration("snappy", "snails"));
}

#[test]
fn quick_brown_fox() {
    assert!(!alliteration("where", "ants"));

    assert!(!alliteration("The", "quick"));
    assert!(!alliteration("brown", "fox"));
    assert!(!alliteration("jumps", "over"));
    assert!(!alliteration("a", "lazy"));
    assert!(!alliteration("lazy", "dog"));
}

#[test]
fn perfect_single() {
    assert!(rhyme("far", "tar"));
    assert!(rhyme("here", "near"));
    assert!(rhyme("a", "say"));
    assert!(rhyme("dissed", "mist"));
}

#[test]
fn general_syllabic() {
    assert!(rhyme("cleaver", "silver"));
    assert!(rhyme("pitter", "patter"));
    assert!(rhyme("bottle", "fiddle"));
}

#[test]
// Not handled yet.
fn perfect_double() {
    assert!(!rhyme("picky", "tricky"));
}

#[test]
// Not handled yet.
fn perfect_dactylic() {
    assert!(!rhyme("cacophonies", "Aristophanes"));
}

#[test]
fn no_rhyme() {
    assert!(!rhyme("tryst", "wrist"));
    assert!(!rhyme("dissed", "trust"));
    assert!(!rhyme("red", "Edmund"));
    assert!(!rhyme("shopping", "cart"));
    assert!(!rhyme("run", "uphill"));
    assert!(!rhyme("comfy", "chair"));

    assert!(!rhyme("empty", "  "));
    assert!(!rhyme("empty", ""));
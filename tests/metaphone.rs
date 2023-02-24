
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
    assert!(!rhyme("empty", "\t"));
    assert!(!rhyme("empty", "\r"));
    assert!(!rhyme("empty", "\n"));
}

#[test]
fn ptah() {
    assert_eq!(
        encoding("ptah"),
        DoubleMetaphone {
            primary: "PT".to_string(),
            secondary: "PT".to_string()
        }
    )
}

#[test]
fn ceasar() {
    assert_eq!(
        encoding("ceasar"),
        DoubleMetaphone {
            primary: "SSR".to_string(),
            secondary: "SSR".to_string()
        }
    )
}

#[test]
fn ach() {
    assert_eq!(
        encoding("ach"),
        DoubleMetaphone {
            primary: "AK".to_string(),
            secondary: "AK".to_string()
        }
    )
}

#[test]
fn chemical() {
    assert_eq!(
        encoding("chemical"),
        DoubleMetaphone {
            primary: "KMKL".to_string(),
            secondary: "KMKL".to_string()
        }
    )
}

#[test]
fn choral() {
    assert_eq!(
        encoding("choral"),
        DoubleMetaphone {
            primary: "KRL".to_string(),
            secondary: "KRL".to_string()
        }
    )
}

#[test]
fn aleksander() {
    assert_eq!(encoding("alexander"), encoding("aleksander"))
}

#[test]
fn hiccups() {
    assert_eq!(encoding("HICCUPS"), encoding("HiCcUpS"));
    assert_eq!(encoding("HiCcUpS"), encoding("hiccups"));
}

#[test]
fn gnarl() {
    assert_eq!(encoding("gnarl").primary.get(..1), Some("N"));
}

#[test]
fn knack() {
    assert_eq!(encoding("knack").primary.get(..1), Some("N"));
}

#[test]
fn pneumatic() {
    assert_eq!(encoding("pneumatic").primary.get(..1), Some("N"));
}

#[test]
fn wrack() {
    assert_eq!(encoding("wrack").primary.get(..1), Some("R"));
}

#[test]
fn psycho() {
    assert_eq!(encoding("psycho").primary.get(..1), Some("S"));
}

#[test]
fn xavier() {
    assert_eq!(encoding("Xavier").primary.get(..1), Some("S"));
}

#[test]
fn vowels() {
    assert_eq!(encoding("a").primary, "A");
    assert_eq!(encoding("e").primary, "A");
    assert_eq!(encoding("i").primary, "A");
    assert_eq!(encoding("o").primary, "A");
    assert_eq!(encoding("u").primary, "A");
    assert_eq!(encoding("y").primary, "A");
}

#[test]
fn drop_initial() {
    assert_eq!(encoding("ba").primary.len(), 1);
    assert_eq!(encoding("be").primary.len(), 1);
    assert_eq!(encoding("bi").primary.len(), 1);
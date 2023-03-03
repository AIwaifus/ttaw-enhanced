
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
    assert_eq!(encoding("bo").primary.len(), 1);
    assert_eq!(encoding("bu").primary.len(), 1);
    assert_eq!(encoding("by").primary.len(), 1);
}

#[test]
fn b_to_p() {
    assert_eq!(encoding("b").primary.get(..1), Some("P"));
    assert_eq!(encoding("bb").primary.get(..1), Some("P"));
}

#[test]
fn c_cedilla_to_s() {
    assert_eq!(encoding("Ç").primary.get(..1), Some("S"));
}

#[test]
fn when_c_to_k() {
    assert_eq!(encoding("ACH").primary.get(1..2), Some("K"));
    assert_ne!(encoding("AACH").primary.get(2..3), Some("K"));
    assert_ne!(encoding("ACHI").primary.get(1..2), Some("K"));
    assert_eq!(encoding("ACHB").primary.get(1..2), Some("K"));
    assert_eq!(encoding("MACHER").secondary.get(1..2), Some("K"));
    assert_eq!(encoding("BACHER").secondary.get(1..2), Some("K"));
}

#[test]
fn caesar() {
    assert_eq!(encoding("CAESAR").primary.get(..1), Some("S"));
}

#[test]
fn chianti() {
    assert_eq!(encoding("chianti").primary.get(..1), Some("K"));
}

#[test]
fn michael() {
    assert_eq!(encoding("michael").primary.get(1..2), Some("K"));

    assert_eq!(encoding("michael").secondary.get(1..2), Some("X"));
}

#[test]
fn chiastic() {
    assert_eq!(encoding("chiastic").primary.get(..1), Some("K"));
}

#[test]
fn chemical_c_to_k() {
    assert_eq!(encoding("chemical").primary.get(..1), Some("K"));
}

#[test]
fn choral_c_to_k() {
    assert_eq!(encoding("choral").primary.get(..1), Some("K"));
}

#[test]
fn chyme_c_to_k() {
    assert_eq!(encoding("chyme").primary.get(..1), Some("K"));
}

#[test]
fn character_c_to_k() {
    assert_eq!(encoding("character").primary.get(..1), Some("K"));
}

#[test]
fn charisma_c_to_k() {
    assert_eq!(encoding("charisma").primary.get(..1), Some("K"));
}

#[test]
fn von_ch_c_to_k() {
    assert_eq!(encoding("von ch").primary.get(2..3), Some("K"));
}

#[test]
fn schooner_c_to_k() {
    assert_eq!(encoding("schooner").primary.get(1..2), Some("K"));
}

#[test]
fn orchestra_c_to_k() {
    assert_eq!(encoding("orchestra").primary.get(2..3), Some("K"));
}

#[test]
fn architect_c_to_k() {
    assert_eq!(encoding("architect").primary.get(2..3), Some("K"));
}

#[test]
fn arch_not_c_to_k() {
    assert_ne!(encoding("arch").primary.get(2..3), Some("K"));
}

#[test]
fn orchid_c_to_k() {
    assert_eq!(encoding("orchid").primary.get(2..3), Some("K"));
}

#[test]
fn chthonic_c_to_k() {
    assert_eq!(encoding("chthonic").primary.get(..1), Some("K"));
}

#[test]
fn fuchsia_c_to_k() {
    assert_eq!(encoding("fuchsia").primary.get(1..2), Some("K"));
}

#[test]
fn chloride_c_to_k() {
    assert_eq!(encoding("chloride").primary.get(..1), Some("K"));
}

#[test]
fn chroma_c_to_k() {
    assert_eq!(encoding("chroma").primary.get(..1), Some("K"));
}
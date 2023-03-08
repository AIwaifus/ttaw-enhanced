
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

#[test]
fn tichner_c_to_k() {
    assert_eq!(encoding("tichner").secondary.get(1..2), Some("K"));
}

#[test]
fn mchugh_c_to_k() {
    assert_eq!(encoding("McHugh").primary.get(1..2), Some("K"));
}

#[test]
fn chore() {
    assert_eq!(encoding("chore").primary.get(..1), Some("X"));
}

#[test]
fn h_after_c() {
    assert_eq!(encoding("achievement").primary.get(1..2), Some("X"));

    assert_eq!(encoding("achievement").secondary.get(1..2), Some("K"));
}

#[test]
fn czerny() {
    assert_eq!(encoding("czerny").primary.get(..1), Some("S"));

    assert_eq!(encoding("czerny").secondary.get(..1), Some("X"));
}

#[test]
fn focaccia() {
    assert_eq!(encoding("focaccia").primary.get(2..3), Some("X"));
}

#[test]
fn accident() {
    assert_eq!(encoding("accident").primary.get(1..2), Some("K"));
    assert_eq!(encoding("accident").primary.get(2..3), Some("S"));
}

#[test]
fn accede() {
    assert_eq!(encoding("accede").primary.get(1..2), Some("K"));
    assert_eq!(encoding("accede").primary.get(2..3), Some("S"));
}

#[test]
fn succeed() {
    assert_eq!(encoding("succeed").primary.get(1..2), Some("K"));
    assert_eq!(encoding("succeed").primary.get(2..3), Some("S"));
}

#[test]
fn bertucci() {
    assert_eq!(encoding("bertucci").primary.get(3..4), Some("X"));
}

#[test]
fn hiccups_c_to_k() {
    assert_eq!(encoding("hiccups").primary.get(1..2), Some("K"));
}

#[test]
fn knack_c_to_k() {
    assert_eq!(encoding("knack").primary.get(1..2), Some("K"));
}

#[test]
fn ancient() {
    assert_eq!(encoding("ancient").primary.get(2..3), Some("S"));

    assert_eq!(encoding("ancient").secondary.get(2..3), Some("X"));
}

#[test]
fn delicious() {
    assert_eq!(encoding("delicious").primary.get(2..3), Some("S"));

    assert_eq!(encoding("delicious").secondary.get(2..3), Some("X"));
}

#[test]
fn acicula() {
    assert_eq!(encoding("acicula").primary.get(1..2), Some("S"));
}

#[test]
fn abduce() {
    assert_eq!(encoding("abduce").primary.get(3..4), Some("S"));
}

#[test]
fn acyl() {
    assert_eq!(encoding("acyl").primary.get(1..2), Some("S"));
}

#[test]
fn maccaffery() {
    assert_eq!(encoding("Mac Caffrey").primary.get(1..2), Some("K"))
}

#[test]
fn macgregor() {
    assert_eq!(encoding("Mac Gregor").primary.get(1..2), Some("K"))
}

#[test]
fn macquillan() {
    assert_eq!(encoding("Mac Quillan").primary.get(1..2), Some("K"))
}

#[test]
fn aback() {
    assert_eq!(encoding("aback").primary.get(2..3), Some("K"))
}

#[test]
fn acquit() {
    assert_eq!(encoding("acquit").primary.get(1..2), Some("K"))
}

#[test]
fn acclimate() {
    assert_eq!(encoding("acclimate").primary.get(1..2), Some("K"))
}

#[test]
fn edge() {
    assert_eq!(encoding("edge").primary.get(1..2), Some("J"))
}

#[test]
fn pidgin() {
    assert_eq!(encoding("pidgin").primary.get(1..2), Some("J"))
}

#[test]
fn edgy() {
    assert_eq!(encoding("edgy").primary.get(1..2), Some("J"))
}

#[test]
fn edgar() {
    assert_eq!(encoding("Edgar").primary.get(1..3), Some("TK"))
}

#[test]
fn width() {
    assert_eq!(encoding("width").primary.get(1..2), Some("T"))
}

#[test]
fn add() {
    assert_eq!(encoding("add").primary.get(1..2), Some("T"))
}

#[test]
fn abduce_slice() {
    assert_eq!(encoding("Abduce").primary.get(2..3), Some("T"))
}

#[test]
fn affect() {
    assert_eq!(encoding("affect").primary.get(1..2), Some("F"))
}

#[test]
fn abaft() {
    assert_eq!(encoding("abaft").primary.get(2..3), Some("F"))
}

#[test]
fn aargh() {
    assert_eq!(encoding("aargh").primary.get(2..3), Some("K"))
}

#[test]
fn ghislane() {
    assert_eq!(encoding("ghislane").primary.get(..1), Some("J"))
}

#[test]
fn ghoul() {
    assert_eq!(encoding("ghoul").primary.get(..1), Some("K"))
}

#[test]
fn hugh() {
    assert_eq!(encoding("hugh").primary, "H")
}

#[test]
fn bough() {
    assert_eq!(encoding("bough").primary, "P")
}

#[test]
fn broughton() {
    assert_eq!(encoding("broughton").primary, "PRTN")
}

#[test]
fn laugh() {
    assert_eq!(encoding("laugh").primary, "LF")
}

#[test]
fn curagh() {
    assert_eq!(encoding("curagh").primary, "KRK")
}

#[test]
fn weight() {
    assert_eq!(encoding("weight").primary, "AT")
}

#[test]
fn agnize() {
    assert_eq!(encoding("agnize").primary.get(..3), Some("AKN"));

    assert_eq!(encoding("agnize").secondary.get(..2), Some("AN"));
}

#[test]
fn tagliaro() {
    assert_eq!(encoding("tagliaro").primary, "TKLR");

    assert_eq!(encoding("tagliaro").secondary, "TLR");
}

#[test]
fn acceptingness() {
    assert!(encoding("acceptingness").primary.ends_with("NNS"));

    assert!(encoding("acceptingness").secondary.ends_with("NKNS"));
}

#[test]
fn cagney() {
    assert_eq!(encoding("cagney").primary, "KKN")
}

#[test]
fn gerben() {
    assert_eq!(encoding("Gerben").primary.get(..1), Some("K"));
    assert_eq!(encoding("Gerben").secondary.get(..1), Some("J"));
}

#[test]
fn auger() {
    assert_eq!(encoding("auger").primary.get(1..2), Some("K"));
    assert_eq!(encoding("auger").secondary.get(1..2), Some("J"));
}

#[test]
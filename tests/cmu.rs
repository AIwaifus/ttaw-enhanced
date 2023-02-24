
extern crate ttaw;

#[cfg(test)]
extern crate tempfile;

use ttaw::cmu::CmuDict;

#[test]
fn cmu_encoding_found() {
    let cmudict = CmuDict::new("cmudict.test").unwrap();
    assert_eq!(
        cmudict.encoding("permeability"),
        Ok(Some(vec![vec![
            "P".to_string(),
            "ER0".to_string(),
            "M".to_string(),
            "IY2".to_string(),
            "AH0".to_string(),
            "B".to_string(),
            "IH1".to_string(),
            "L".to_string(),
            "IH0".to_string(),
            "T".to_string(),
            "IY0".to_string()
        ]]))
    );

    assert_eq!(
        cmudict.encoding("unearthed"),
        Ok(Some(vec![vec![
            "AH0".to_string(),
            "N".to_string(),
            "ER1".to_string(),
            "TH".to_string(),
            "T".to_string()
        ]]))
    );
}

#[test]
fn cmu_encoding_not_found() {
    let cmudict = CmuDict::new("cmudict.test").unwrap();
    assert_eq!(cmudict.encoding("2123123"), Ok(None));
    assert_eq!(cmudict.encoding("%^%##%"), Ok(None));
}

#[test]
fn rhymes_with_spaces() {
    let cmudict = CmuDict::new("cmudict.test").unwrap();
    assert!(cmudict.rhyme("far  ", "tar").unwrap());
    assert!(cmudict.rhyme(" far", "tar").unwrap());
    assert!(cmudict.rhyme("far", " tar").unwrap());
    assert!(cmudict.rhyme("far", "tar  ").unwrap());
}

#[test]
fn rhymes_with_caps() {
    let cmudict = CmuDict::new("cmudict.test").unwrap();
    assert!(cmudict.rhyme("Far", "tar").unwrap());
    assert!(cmudict.rhyme("far", "Tar").unwrap());
    assert!(cmudict.rhyme("fAr", "taR").unwrap());
    assert!(cmudict.rhyme("far", "tAr").unwrap());
}

#[test]
fn perfect_single() {
    let cmudict = CmuDict::new("cmudict.test").unwrap();
    assert!(cmudict.rhyme("far", "tar").unwrap());
    assert!(cmudict.rhyme("a", "say").unwrap());
    assert!(cmudict.rhyme("hissed", "mist").unwrap());
    assert!(cmudict.rhyme("tryst", "wrist").unwrap());
}

#[test]
fn no_rhyme() {
    let cmudict = CmuDict::new("cmudict.test").unwrap();
    assert!(!cmudict.rhyme("dissed", "trust").unwrap());
    assert!(!cmudict.rhyme("red", "Edmund").unwrap());
    assert!(!cmudict.rhyme("shopping", "cart").unwrap());
    assert!(!cmudict.rhyme("run", "uphill").unwrap());
    assert!(!cmudict.rhyme("comfy", "chair").unwrap());

    assert!(!cmudict.rhyme("empty", "  ").unwrap());
    assert!(!cmudict.rhyme("empty", "").unwrap());
    assert!(!cmudict.rhyme("empty", "\t").unwrap());
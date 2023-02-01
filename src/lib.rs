extern crate pest;
///
/// ## CMU
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// let cmudict = ttaw::cmu::CmuDict::new("cmudict.json").unwrap();
/// assert_eq!(
///     cmudict.encoding("permeability"),
///     Ok(Some(vec![vec![
///         "P".to_string(),
///         "ER0".to_string(),
///         "M".to_string(),
///         "IY2".to_string(),
///         "AH0".to_string(),
/// 
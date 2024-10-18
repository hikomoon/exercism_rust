#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    let mut strs = input.graphemes(true).collect::<Vec<&str>>();
    strs.reverse();
    strs.into_iter().collect::<String>()
}

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    let mut strs = input.chars().collect::<Vec<char>>();
    strs.reverse();
    strs.into_iter().collect::<String>()   
}

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let g = UnicodeSegmentation::graphemes(input, true);
    g.rev().collect()
}

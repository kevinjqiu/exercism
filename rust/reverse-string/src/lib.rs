pub fn reverse(input: &str) -> String {
    let mut output = Vec::new();
    for (_, ch) in input.chars().rev().enumerate() {
        output.push(ch);
    }
    output.into_iter().collect()
}

pub fn reverse(input: &str) -> String {
    if input.len() == 0 {
        return String::from("");
    }
    let input_bytes = input.as_bytes();
    let mut output = Vec::new();
    for i in (0..input.len()).rev() {
        output.push(input_bytes[i])
    }
    String::from_utf8(output).unwrap()
}

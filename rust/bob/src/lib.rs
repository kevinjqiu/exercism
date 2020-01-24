fn is_question(message: &str) -> bool {
    return message.ends_with("?")
}

fn is_all_caps(message: &str) -> bool {
    let mut has_cap = false;
    for ch in message.chars() {
        if ch >= 'a' && ch <= 'z' {
            return false
        }
        if ch >= 'A' && ch <= 'Z' {
            has_cap = true;
        }
    }
    return has_cap;
}

pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    if is_all_caps(&msg) && is_question(&msg) {
        return "Calm down, I know what I'm doing!"
    }

    if is_question(&msg) {
        return "Sure."
    }

    if msg.len() == 0 {
        return "Fine. Be that way!"
    }

    if is_all_caps(&msg) {
        return "Whoa, chill out!"
    }

    return "Whatever."
}

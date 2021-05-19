pub fn latexify(text: String) -> String {
    return text
        .replace("<", "{")
        .replace(">", "}")
        // TODO: this is an incredibly stupid hack
        .replace("}{\\lcol}", ">{\\lcol}");
}

pub fn join_string_list(strings: &[&str]) -> Option<String> {
    if strings.len() == 0 {
        return None;
    } else if strings.len() == 1 {
        return Some(strings[0].to_string());
    } else if strings.len() == 2 {
        return Some(format!("{} and {}", strings[0], strings[1]));
    } else {
        return Some(format!(
            "{}, and {}",
            strings[..strings.len() - 1].join(", "),
            strings[strings.len() - 1]
        ));
    }
}

pub fn uppercase_first_letter(text: &str) -> String {
    if let Some(c) = text.get(0..1) {
        return c.to_ascii_uppercase() + if let Some(s) = text.get(1..) { s } else { "" };
    } else {
        return text.to_string();
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses = vec![];
    for i in (end..=start).rev() {
        verses.push(verse(i));
    }
    verses.join("\n")
}

pub fn verse(num: u32) -> String {
    let current_container = generate_current_container(num);
    let pronoun = generate_pronoun(num);
    let count_phrase = generate_count_phrase(num);
    let next_count_phrase = generate_next_count_phrase(num);
    let next_container = generate_next_container(num);
    match num {
        1..=99 => format!(
            "{count_phrase} {current_container} of beer on the wall, {count_phrase} {current_container} of beer.\n\
            Take {pronoun} down and pass it around, {next_count_phrase} {next_container} of beer on the wall.\n"
        ),
        0 => format!("{0} {current_container} of beer on the wall, {count_phrase} {current_container} of beer.\n\
            Go to the store and buy some more, {next_count_phrase} {next_container} of beer on the wall.\n",
            capitalize_first_letter(&count_phrase)),
        _ => todo!(),
    }
}

fn capitalize_first_letter(string_slice: &str) -> String {
    let mut chars = string_slice.chars();
    let first_char = chars.next().unwrap().to_uppercase();
    let rest_of_chars: String = chars.collect();
    format!("{}{}", first_char, rest_of_chars)
}

fn generate_pronoun(num: u32) -> String {
    match num {
        1 => "it".to_string(),
        _ => "one".to_string(),
    }
}

fn generate_current_container(num: u32) -> String {
    match num {
        1 => "bottle".to_string(),
        _ => "bottles".to_string(),
    }
}

fn generate_count_phrase(num: u32) -> String {
    match num {
        0 => "no more".to_string(),
        _ => (num).to_string(),
    }
}

fn generate_next_count_phrase(num: u32) -> String {
    match num {
        1 => "no more".to_string(),
        0 => "99".to_string(),
        _ => format!("{}", num - 1),
    }
}

fn generate_next_container(num: u32) -> String {
    match num {
        2 => "bottle".to_string(),
        _ => "bottles".to_string(),
    }
}

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
        0 => format!(
            "No more {current_container} of beer on the wall, {count_phrase} {current_container} of beer.\n\
            Go to the store and buy some more, {next_count_phrase} {next_container} of beer on the wall.\n"),
        _ => todo!(),
    }
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

pub fn sing(_start: u32, _end: u32) -> String {
    todo!()
}

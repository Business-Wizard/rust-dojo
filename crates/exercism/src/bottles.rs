pub fn verse(num: u32) -> String {
    let current_container = generate_container(num);
    let pronoun = generate_pronoun(num);
    let count_phrase = generate_count_phrase(num);
    let next_container = generate_container(num - 1);
    match num {
        1..=99 => format!(
            "{num} {current_container} of beer on the wall, {num} {current_container} of beer.\n\
            Take {pronoun} down and pass it around, {count_phrase} {next_container} of beer on the wall.\n"
        ),
        _ => todo!(),
    }
}

fn generate_pronoun(num: u32) -> String {
    match num {
        1 => "it".to_string(),
        _ => "one".to_string(),
    }
}

fn generate_container(num: u32) -> String {
    match num {
        1 => "bottle".to_string(),
        _ => "bottles".to_string(),
    }
}

fn generate_count_phrase(num: u32) -> String {
    match num {
        1 => "no more".to_string(),
        _ => (num - 1).to_string(),
    }
}

pub fn sing(_start: u32, _end: u32) -> String {
    todo!()
}

pub fn verse(num: u32) -> String {
    let current_container = container(num);
    let next_num = next_number_word(num);
    let next_container = container(num - 1);
    match num {
        2..=99 => format!(
            "{num} {current_container} of beer on the wall, {num} {current_container} of beer.\n\
            Take one down and pass it around, {next_num} {next_container} of beer on the wall.\n"
        ),
        1 => format!(
            "{num} {current_container} of beer on the wall, {num} {current_container} of beer.\n\
            Take it down and pass it around, {next_num} bottles of beer on the wall.\n"
        ),
        _ => todo!(),
    }
}

fn container(num: u32) -> String {
    match num {
        1 => "bottle".to_string(),
        _ => "bottles".to_string(),
    }
}

fn next_number_word(num: u32) -> String {
    match num {
        1 => "no more".to_string(),
        _ => (num - 1).to_string(),
    }
}

pub fn sing(_start: u32, _end: u32) -> String {
    todo!()
}

pub fn verse(num: u32) -> String {
    let next_num = num - 1;
    let current_container = container(num);
    let next_container = container(next_num);
    match num {
        3..=99 => format!(
            "{num} {current_container} of beer on the wall, {num} {current_container} of beer.\n\
            Take one down and pass it around, {next_num} {next_container} of beer on the wall.\n"
        ),
        2 => format!(
            "{num} {current_container} of beer on the wall, {num} {current_container} of beer.\n\
            Take one down and pass it around, {next_num} {next_container} of beer on the wall.\n"
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

pub fn sing(_start: u32, _end: u32) -> String {
    todo!()
}

pub fn verse(num: u32) -> String {
    let next_num = num - 1;
    match num {
        3..=99 => format!(
            "{num} bottles of beer on the wall, {num} bottles of beer.\n\
            Take one down and pass it around, {next_num} bottles of beer on the wall.\n",
        ),
        _ => todo!(),
    }
}

pub fn sing(_start: u32, _end: u32) -> String {
    todo!()
}

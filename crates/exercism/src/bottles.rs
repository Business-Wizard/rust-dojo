pub fn verse(num: u32) -> String {
    match num {
        3..=99 => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\n\
            Take one down and pass it around, {1} bottles of beer on the wall.\n",
            num,
            num - 1
        ),
        _ => todo!(),
    }
}

pub fn sing(_start: u32, _end: u32) -> String {
    todo!()
}

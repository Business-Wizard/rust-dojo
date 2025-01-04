pub fn verse(num: u32) -> String {
    match num {
        99 => "99 bottles of beer on the wall, 99 bottles of beer.\n\
    Take one down and pass it around, 98 bottles of beer on the wall.\n"
            .to_string(),
        3 => "3 bottles of beer on the wall, 3 bottles of beer.\n\
    Take one down and pass it around, 2 bottles of beer on the wall.\n"
            .to_string(),
        _ => todo!(),
    }
}

pub fn sing(_start: u32, _end: u32) -> String {
    todo!()
}

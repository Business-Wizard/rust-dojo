use exercism::bottles;

#[test]
fn first_generic_verse() {
    let actual = bottles::verse(99);
    let expected = "99 bottles of beer on the wall, 99 bottles of beer.\n\
        Take one down and pass it around, 98 bottles of beer on the wall.\n"
        .to_string();
    assert_eq!(expected, actual);
}

#[test]
fn last_generic_verse() {
    let actual = bottles::verse(3);
    let expected = "3 bottles of beer on the wall, 3 bottles of beer.\n\
        Take one down and pass it around, 2 bottles of beer on the wall.\n"
        .to_string();
    assert_eq!(expected, actual);
}

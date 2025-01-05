mod multiple_verses {
    use exercism::bottles;

    #[test]
    fn first_two_verses() {
        let actual = bottles::sing(99, 98);
        let expected = "99 bottles of beer on the wall, 99 bottles of beer.\n\
            Take one down and pass it around, 98 bottles of beer on the wall.\n\n\
            98 bottles of beer on the wall, 98 bottles of beer.\n\
            Take one down and pass it around, 97 bottles of beer on the wall.\n"
            .to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn last_three_verses() {
        let actual = bottles::sing(3, 0);
        let expected = "3 bottles of beer on the wall, 3 bottles of beer.\n\
            Take one down and pass it around, 2 bottles of beer on the wall.\n\n\
            2 bottles of beer on the wall, 2 bottles of beer.\n\
            Take one down and pass it around, 1 bottle of beer on the wall.\n\n\
            1 bottle of beer on the wall, 1 bottle of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n\n\
            No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn middle_verses() {
        let actual = bottles::sing(50, 48);
        let expected = "50 bottles of beer on the wall, 50 bottles of beer.\n\
            Take one down and pass it around, 49 bottles of beer on the wall.\n\n\
            49 bottles of beer on the wall, 49 bottles of beer.\n\
            Take one down and pass it around, 48 bottles of beer on the wall.\n\n\
            48 bottles of beer on the wall, 48 bottles of beer.\n\
            Take one down and pass it around, 47 bottles of beer on the wall.\n"
            .to_string();
        assert_eq!(expected, actual);
    }
}

mod single_verses {
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

    #[test]
    fn verse_2_bottles_should_handle_noun_endings() {
        let actual = bottles::verse(2);
        let expected = "2 bottles of beer on the wall, 2 bottles of beer.\n\
            Take one down and pass it around, 1 bottle of beer on the wall.\n"
            .to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn verse_1_bottle_should_handle_noun_endings() {
        let actual = bottles::verse(1);
        let expected = "1 bottle of beer on the wall, 1 bottle of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n"
            .to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn verse_0_bottles() {
        let actual = bottles::verse(0);
        let expected = "No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string();
        assert_eq!(expected, actual);
    }
}

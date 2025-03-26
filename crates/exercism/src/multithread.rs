pub fn sum_of_squares_with_single_thread(range: std::ops::Range<u32>) -> u32 {
    range.map(|x| x * x).sum()
}

#[cfg(test)]

mod single_thread_tests {
    use super::*;

    #[test]
    fn test_single() {
        let input = 2..3;
        let actual = sum_of_squares_with_single_thread(input);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}

pub fn sum_of_squares_with_multi_thread(range: std::ops::Range<u32>) -> u32 {
    let range_size = range.end - range.start;
    let thread_count = set_thread_count(range_size);
    if thread_count == 1 {
        return sum_of_squares_with_single_thread(range);
    }

    let threads = setup_thread_pool(range, thread_count);
    threads
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}

fn set_thread_count(data_size: u32) -> u32 {
    (data_size as f64).sqrt().ceil() as u32
}

fn setup_thread_pool(
    range: std::ops::Range<u32>,
    thread_count: u32,
) -> Vec<std::thread::JoinHandle<u32>> {
    let range_size = range.end - range.start;
    let chunk_size = range_size / thread_count;

    let mut thread_pool = vec![];
    for i in 0..thread_count {
        let data_chunk = range
            .clone()
            .skip((i * chunk_size) as usize)
            .take(chunk_size as usize);
        let handle = std::thread::spawn(move || data_chunk.map(|x| x * x).sum());
        thread_pool.push(handle);
    }

    thread_pool
}

#[cfg(test)]

mod multi_thread_tests {
    use super::*;

    #[test]
    fn test_single() {
        let input = 2..3;
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_two() {
        let input = 2..4;
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 4 + 9;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_three() {
        let input = 2..5;
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 4 + 9 + 16;
        assert_eq!(actual, expected);
    }
}

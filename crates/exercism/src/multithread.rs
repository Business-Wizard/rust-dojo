use std::thread;

pub fn sum_of_squares_with_single_thread(data: Vec<u64>) -> u64 {
    data.into_iter().map(|x| x * x).sum()
}

#[cfg(test)]

mod single_thread_tests {
    use super::*;

    #[test]
    fn test_single() {
        let input = vec![1; 1];
        let actual = sum_of_squares_with_single_thread(input);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_two() {
        let input = vec![1; 2];
        let actual = sum_of_squares_with_single_thread(input);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}

pub fn sum_of_squares_with_multi_thread(data: Vec<u64>) -> u64 {
    let data_size = data.len() as u32;
    let thread_count = set_thread_count(data_size);
    dbg!(thread_count);
    if thread_count == 1 {
        return sum_of_squares_with_single_thread(data);
    }

    let threads = setup_thread_pool(data, thread_count);
    threads
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}

fn set_thread_count(data_size: u32) -> u32 {
    let max_threads = 16;
    let log_10 = (data_size as f64).log10().trunc() as u32;
    log_10.max(1).min(max_threads)
}

fn setup_thread_pool(data: Vec<u64>, thread_count: u32) -> Vec<std::thread::JoinHandle<u64>> {
    let range_size = data.len() as u32;
    dbg!(range_size);
    let chunk_size = range_size / thread_count;
    dbg!(chunk_size);

    let mut thread_pool = vec![];
    for i in 0..thread_count {
        let start_idx = (i * chunk_size) as usize;
        let end_idx: usize;
        if i == thread_count - 1 {
            end_idx = data.len();
        } else {
            end_idx = ((i + 1) * chunk_size) as usize;
        }

        let data_chunk = data[start_idx..end_idx].to_vec();
        let handle: thread::JoinHandle<u64> =
            std::thread::spawn(move || data_chunk.into_iter().map(|x| x * x).sum());
        thread_pool.push(handle);
    }

    thread_pool
}

#[cfg(test)]

mod multi_thread_tests {
    use super::*;

    #[test]
    fn test_single() {
        let input = vec![1; 1];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_two() {
        let input = vec![1; 2];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hundred() {
        let input: Vec<u64> = vec![1; 100];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 100;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_two_hundred() {
        let input: Vec<u64> = vec![1; 200];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 200;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_thousand() {
        let input: Vec<u64> = vec![1; 1000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 1000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ten_thousand() {
        let input: Vec<u64> = vec![1; 10_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 10_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hundred_thousand() {
        let input: Vec<u64> = vec![1; 100_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 100_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_million() {
        let input: Vec<u64> = vec![1; 1_000_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 1_000_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ten_million() {
        let input: Vec<u64> = vec![1; 10_000_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 10_000_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hundred_million() {
        let input: Vec<u64> = vec![1; 100_000_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 100_000_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_billion() {
        let input: Vec<u64> = vec![1; 1_000_000_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 1_000_000_000;
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]

mod set_thread_count_tests {
    #[test]
    fn test_one() {
        let input = 1;
        let actual = super::set_thread_count(input);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_two() {
        let input = 2;
        let actual = super::set_thread_count(input);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ten() {
        let input = 10;
        let actual = super::set_thread_count(input);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_twenty() {
        let input = 20;
        let actual = super::set_thread_count(input);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hundred() {
        let input = 100;
        let actual = super::set_thread_count(input);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_thousand() {
        let input = 1000;
        let actual = super::set_thread_count(input);
        let expected = 3;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ten_thousand() {
        let input = 10_000;
        let actual = super::set_thread_count(input);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}

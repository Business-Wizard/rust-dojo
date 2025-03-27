use std::thread;

pub fn sum_of_squares_with_single_thread(data: Vec<u32>) -> u64 {
    data.into_iter().map(|x| (x as u64).pow(2)).sum()
}

#[cfg(test)]

mod single_thread_tests {
    use super::*;

    #[test]
    fn test_single() {
        let input: Vec<u32> = vec![1; 1];
        let actual = sum_of_squares_with_single_thread(input);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_two() {
        let input: Vec<u32> = vec![1; 2];
        let actual = sum_of_squares_with_single_thread(input);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}

pub fn sum_of_squares_with_multi_thread(data: Vec<u32>) -> u128 {
    let data_size = data.len() as u32;
    let thread_count = set_thread_count(data_size);
    dbg!(thread_count);
    if thread_count == 1 {
        return sum_of_squares_with_single_thread(data) as u128;
    }

    let threads = setup_thread_pool(data, thread_count);
    threads
        .into_iter()
        .map(|handle| handle.join().expect("Thread panicked. Likely Overflow."))
        .sum()
}

fn set_thread_count(data_size: u32) -> u32 {
    let max_threads = 16;
    if data_size <= 100 {
        return 1;
    }
    let log_2 = (data_size as f64).log2().trunc() as u32;
    log_2.max(1).min(max_threads)
}

fn setup_thread_pool(data: Vec<u32>, thread_count: u32) -> Vec<std::thread::JoinHandle<u128>> {
    let range_size: u32 = data.len() as u32;
    dbg!(range_size);
    let chunk_size: u32 = range_size / thread_count;
    dbg!(chunk_size);

    let indexes = (0..range_size).step_by(chunk_size as usize);
    let indexes = indexes.chain(std::iter::once(range_size));
    let index_pairs = indexes.clone().zip(indexes.skip(1));

    let mut thread_pool = vec![];
    for (start_idx, end_idx) in index_pairs {
        let data_chunk = data[start_idx as usize..end_idx as usize].to_vec();
        let handle: thread::JoinHandle<u128> =
            std::thread::spawn(move || data_chunk.into_iter().map(|x| (x as u128).pow(2)).sum());
        thread_pool.push(handle);
    }
    thread_pool
}

#[cfg(test)]

mod multi_thread_tests_stub_data {
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
        let input: Vec<u32> = vec![1; 100];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 100;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_two_hundred() {
        let input: Vec<u32> = vec![1; 200];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 200;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_thousand() {
        let input: Vec<u32> = vec![1; 1000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 1000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ten_thousand() {
        let input: Vec<u32> = vec![1; 10_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 10_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hundred_thousand() {
        let input: Vec<u32> = vec![1; 100_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 100_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_million() {
        let input: Vec<u32> = vec![1; 1_000_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 1_000_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ten_million() {
        let input: Vec<u32> = vec![1; 10_000_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 10_000_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hundred_million() {
        let input: Vec<u32> = vec![1; 100_000_000];
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 100_000_000;
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod multi_thread_tests_range_data {
    use super::*;
    use std::time;

    #[test]
    fn test_single() {
        let input: Vec<u32> = (2..3).collect();
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_two() {
        let input: Vec<u32> = (2..4).collect();
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 13;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_million() {
        let input: Vec<u32> = (1..=1_000_00).collect();
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 333_338_333_350_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ten_million() {
        let input: Vec<u32> = (1..=10_000_000).collect();
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 333_333_383_333_335_000_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hundred_million() {
        let input: Vec<u32> = (1..=100_000_000).collect();
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 333_333_338_333_333_350_000_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_billion() {
        let input: Vec<u32> = (1..=1_000_000_000).collect();
        let actual = sum_of_squares_with_multi_thread(input);
        let expected = 333_333_333_833_333_333_500_000_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_billion_runtime() {
        let input = (1..=1_000_000_000).collect();
        let start = time::Instant::now();
        let _ = sum_of_squares_with_multi_thread(input);
        let duration = start.elapsed();
        let expected = time::Duration::from_secs(5);
        println!("Duration: {:?}", duration);
        assert!(duration < expected);
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
    fn test_hundred_should_return_one() {
        let input = 100;
        let actual = super::set_thread_count(input);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_thousand() {
        let input = 1000;
        let actual = super::set_thread_count(input);
        let expected = 9;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ten_thousand() {
        let input = 10_000;
        let actual = super::set_thread_count(input);
        let expected = 13;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_million_should_use_max_threads() {
        let input = 1_000_000;
        let actual = super::set_thread_count(input);
        let expected = 16;
        assert_eq!(actual, expected);
    }
}

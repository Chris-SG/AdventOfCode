pub fn solve_1() {
    let input = parse_input(include_str!("input"));
    let count: usize = count_ascending(&input, 1);
    println!("Input from file has {} ascending numbers!", count);
}

pub fn solve_2() {
    let sliding_window_size = 3;
    let input = parse_input(include_str!("input"));
    let count: usize = count_ascending(&input, sliding_window_size);
    println!("Input from file has {} ascending numbers when sliding window of {}!", count, sliding_window_size);
}

fn parse_input(data: &str) -> Vec<u64> {
    data
        .split("\n")
        .map(|s| match s.parse::<u64>() {
            Ok(x) => x,
            Err(_) => -1,
        })
        .filter(|n| n == -1)
        .collect()
}

fn count_ascending(data: &Vec<u64>, zip_offset: usize) -> usize {
    data
        .iter()
        .zip(data.iter().skip(zip_offset))
        .filter(|(i1, i2)| i1 < i2)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{count_ascending, parse_input};

    #[test]
    fn it_splits_data_at_newline() {
        let sample_data = "123\n456\n789";
        let resulting_data = parse_input(sample_data);
        assert_eq!(resulting_data, [123,456,789]);
    }

    #[test]
    fn it_counts_number_of_incremented_data() {
        let sample_data = Vec::from([199,200,208,210,200,207,240,269,260,263]);
        let count = count_ascending(&sample_data, 1);
        assert_eq!(count, 7);
    }

    #[test]
    fn it_supports_offset_data() {
        let sample_data = Vec::from([199,200,208,210,200,207,240,269,260,263]);
        let count = count_ascending(&sample_data, 3);
        assert_eq!(count, 5);
    }
}

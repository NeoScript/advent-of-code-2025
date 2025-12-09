use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let part_one = solve_part_one().unwrap();
    println!("part one: {}", part_one);
}

fn solve_part_one() -> Result<u32, Box<dyn Error>> {
    let input_path = Path::new("input.txt");
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);

    let result: u32 = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|batteries| get_joltage_v1(&batteries))
        .sum();
    Ok(result)
}

fn get_largest_starting_num_and_index(raw_data: &str) -> (u32, usize) {
    let str_len = raw_data.len();
    let limited_str = &raw_data[..str_len - 1];

    let mut result: u32 = 0;
    let mut result_idx: usize = 0;

    for (index, char) in limited_str.char_indices() {
        let as_digit = char.to_digit(10).unwrap();
        if as_digit > result {
            result = as_digit;
            result_idx = index;
        }
    }

    (result, result_idx)
}

fn get_second_digit(raw_data: &str, starting_at: usize) -> u32 {
    let limited_str = &raw_data[starting_at..];

    let max_val: u32 = limited_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .max()
        .unwrap();

    max_val
}

fn get_joltage_v1(batteries: &str) -> u32 {
    let (first_digit, idx) = get_largest_starting_num_and_index(batteries);
    let second_digit = get_second_digit(batteries, idx + 1);

    (first_digit * 10) + second_digit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_starting_num_and_index() {
        let entry = "12345";
        let (result, index) = get_largest_starting_num_and_index(entry);

        assert_eq!(result, 4);
        assert_eq!(index, 3);
    }
    #[test]
    fn test_find_second_num() {
        let raw_data = "12345";
        let (_, i) = get_largest_starting_num_and_index(raw_data);
        let result = get_second_digit(raw_data, i);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_a() {
        let batteries = "987654321111111";
        let result = get_joltage_v1(batteries);

        assert_eq!(result, 98);
    }

    #[test]
    fn test_b() {
        let batteries = "811111111111119";
        let result = get_joltage_v1(batteries);

        assert_eq!(result, 89);
    }

    #[test]
    fn test_c() {
        let batteries = "234234234234278";
        let result = get_joltage_v1(batteries);
        assert_eq!(result, 78);
    }

    #[test]
    fn test_d() {
        let batteries = "818181911112111";
        let result = get_joltage_v1(batteries);

        assert_eq!(result, 92);
    }
}

use std::{fs, io::Error, ops::RangeInclusive, path::Path};

fn main() {
    println!("Running day 2 calculations");
    let input_one = read_input().expect("should read input file");
    let data_one = parse_input_one(input_one);

    let invalid_ids: Vec<u128> = data_one
        .iter()
        .flat_map(|r| r.clone().find_invalid_ids())
        .collect();

    let sum: u128 = invalid_ids.iter().sum();
    println!("part one: {}", sum);
}

#[derive(Clone)]
struct NumberRange {
    start_str: String,
    end_str: String,
}

impl NumberRange {
    pub fn from_string(input: String) -> Self {
        input.into()
    }
    pub fn as_range(&self) -> RangeInclusive<u128> {
        let start_parse_error = format!("failed parsing: {}", self.start_str);
        let start_num: u128 = self
            .start_str
            .parse::<u64>()
            .expect(&start_parse_error)
            .into();

        let end_parse_error = format!("failed parsing: {}", self.end_str);
        let end_num: u128 = self.end_str.parse::<u64>().expect(&end_parse_error).into();

        start_num..=end_num
    }
    pub fn find_invalids_part_two(&self) -> Vec<u128> {
        let results = vec![];

        results
    }

    pub fn find_invalid_ids(&self) -> Vec<u128> {
        let mut invalids = vec![];
        let range = self.as_range();

        for i in range {
            let str_repr = format!("{}", i);
            let str_len = str_repr.len();

            if str_len % 2 == 0 {
                let midpoint_index = str_len / 2;
                let first_half = &str_repr[0..midpoint_index];
                let second_half = &str_repr[midpoint_index..];

                if first_half.eq(second_half) {
                    invalids.push(i);
                }
            }
        }

        invalids
    }
}

impl From<String> for NumberRange {
    fn from(value: String) -> Self {
        let dash_index = value.find("-").expect("should find dash");
        let (start_str, end_str) = (value[0..dash_index].trim(), value[dash_index + 1..].trim());
        NumberRange {
            start_str: start_str.into(),
            end_str: end_str.into(),
        }
    }
}

fn parse_input_one(file_contents: String) -> Vec<NumberRange> {
    let splits = file_contents.split(",");

    let results: Vec<NumberRange> = splits.map(|entry| entry.to_string().into()).collect();
    results
}

fn read_input() -> Result<String, Error> {
    let path = Path::new("input1.txt");
    fs::read_to_string(path)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_to_numer_range() {
        let input_string = String::from("1-10");
        let result: NumberRange = input_string.into();

        assert_eq!(result.start_str, "1");
        assert_eq!(result.end_str, "10");
    }

    #[test]
    fn test_parse_string_with_extra_spacing() {
        let input_string = String::from(" 1-20 ");
        let result: NumberRange = input_string.into();

        assert_eq!(result.start_str, "1");
        assert_eq!(result.end_str, "20");
    }

    #[test]
    fn test_range_creation() {
        let input_obj = NumberRange {
            start_str: "1".into(),
            end_str: "10".into(),
        };
        let result = input_obj.as_range();

        assert_eq!(result.start(), &1);
        assert_eq!(result.end(), &10);
    }

    #[test]
    fn test_invalid_ids_one() {
        let input_obj: NumberRange = String::from("99-115").into();
        let results = input_obj.find_invalid_ids();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], 99);
    }

    #[test]
    fn test_invalid_ids_two() {
        let input_obj: NumberRange = String::from("99-115").into();
        let results = input_obj.find_invalids_part_two();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], 11);
        assert_eq!(results[1], 22);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Dial(u32);

impl Default for Dial {
    fn default() -> Self {
        Self(50)
    }
}

impl Dial {
    pub fn tick(&mut self, direction: &Direction) {
        match (self.0, direction) {
            (99, Direction::Right) => self.0 = 0,
            (0, Direction::Left) => self.0 = 99,
            (_, Direction::Left) => {
                self.0 -= 1;
            }
            (_, Direction::Right) => self.0 += 1,
        }
    }
    pub fn turn(&mut self, rotation: &RotationInstruction) {
        for _step in 0..rotation.quantity {
            self.tick(&rotation.direction);
        }
    }

    pub fn turn_and_count_zeros(&mut self, rotation: &RotationInstruction) -> u64 {
        let mut counter = 0;
        for _step in 0..rotation.quantity {
            self.tick(&rotation.direction);

            if self.at_zero() {
                counter += 1;
            }
        }

        counter
    }

    pub fn at_zero(self) -> bool {
        self.0 == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left = -1,
    Right = 1,
}

#[derive(Debug)]
pub struct RotationInstruction {
    direction: Direction,
    quantity: u64,
}

impl TryFrom<&str> for RotationInstruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // println!("trying to convert: {}", value);
        let (dir_indicator, quantity_str) =
            value.split_at_checked(1).ok_or("Could not split row")?;

        let direction = match dir_indicator {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => {
                panic!("invalid direction detected: {}", value)
            }
        };

        let quantity: u64 = quantity_str
            .parse::<u64>()
            .expect("should be able to parse value");

        Ok(RotationInstruction {
            direction,
            quantity,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_parsing() {
        let instruction = "R1000";
        let result: RotationInstruction = instruction.try_into().expect("should parse");

        assert_eq!(result.direction, Direction::Right);
        assert_eq!(result.quantity, 1000);
    }

    #[test]
    fn test_count_zeros_per_click() {
        let mut dial = Dial::default();
        let test_rotation = RotationInstruction {
            direction: Direction::Right,
            quantity: 1000,
        };
        let result = dial.turn_and_count_zeros(&test_rotation);

        assert_eq!(
            result, 10,
            "applying R1000 to Dial starting at 50 should result in 10 for the zero counts"
        );

        assert_eq!(dial.0, 50);
    }

    #[test]
    fn test_count_zeros_with_two_times_r500() {
        let mut dial = Dial::default();
        let first_rotation = "R500"
            .try_into()
            .expect("should create right 500 instruction");

        let second_rotation = "R500"
            .try_into()
            .expect("should create right 500 instruction");

        let mut counter = 0;
        counter += dial.turn_and_count_zeros(&first_rotation);
        counter += dial.turn_and_count_zeros(&second_rotation);

        assert_eq!(counter, 10);
        assert_eq!(dial.0, 50);
    }

    #[test]
    fn test_simple_turn_and_count() {
        let mut dial = Dial::default();
        let rotation = "R50".try_into().expect("should make rotation instruction");
        let mut counter = 0;
        counter += dial.turn_and_count_zeros(&rotation);

        assert_eq!(counter, 1);
        assert_eq!(dial.0, 0);
    }
}

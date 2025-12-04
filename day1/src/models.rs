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

    pub fn at_zero(self) -> bool {
        self.0 == 0
    }
}

#[derive(Clone, Copy, Debug)]
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
        println!("trying to convert: {}", value);
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

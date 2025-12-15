use std::str::FromStr;

/* ---------- */

const INPUT: &str = include_str!("../../../inputs/2015/day06.txt");

/* ---------- */

pub(crate) fn resolve() {
    println!("[2015 DAY5]");

    let mut grid = Grid::new();

    INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().expect("Failed to parse a line"))
        .for_each(|instr| grid.update(instr));

    println!("Part1 => {}", grid.lights_on());
    println!("Part2 => {}", grid.intensity());
}

/* ---------- */

#[derive(Debug)]
struct Grid {
    grid: [[Light; 1000]; 1000],
}

impl Grid {
    #[inline(always)]
    fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    fn update(&mut self, instr: Instruction) {
        let x_dist = instr.to.x - instr.from.x + 1;
        let y_dist = instr.to.y - instr.from.y + 1;

        self.grid
            .iter_mut()
            .skip(instr.from.y)
            .take(y_dist)
            .for_each(|light_line| {
                light_line
                    .iter_mut()
                    .skip(instr.from.x)
                    .take(x_dist)
                    .for_each(|light| light.update(instr.instr_type));
            });
    }

    #[inline(always)]
    fn lights_on(&self) -> u64 {
        self.grid
            .iter()
            .map(|light_line| light_line.iter().filter(|light| light.is_on()).count() as u64)
            .sum()
    }

    #[inline(always)]
    fn intensity(&self) -> u64 {
        self.grid
            .iter()
            .map(|light_line| {
                light_line
                    .iter()
                    .map(|light| light.intensity())
                    .sum::<u64>()
            })
            .sum()
    }
}

impl Default for Grid {
    #[inline(always)]
    fn default() -> Self {
        Self {
            grid: [[Light::default(); 1000]; 1000],
        }
    }
}

/* ---------- */

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Light {
    on: bool,
    intensity: u8,
}

impl Light {
    #[inline(always)]
    fn update(&mut self, instr_type: InstructionType) {
        match instr_type {
            InstructionType::Off => {
                self.on = false;
                self.intensity = self.intensity.saturating_sub(1)
            }
            InstructionType::On => {
                self.on = true;
                self.intensity += 1
            }
            InstructionType::Toggle => {
                self.on = !self.on;
                self.intensity += 2
            }
        }
    }

    #[inline(always)]
    fn is_on(&self) -> bool {
        self.on
    }

    fn intensity(&self) -> u64 {
        self.intensity as u64
    }
}

/* ---------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl FromStr for Position {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or("Invalid format: missing position separator")?;

        let x = x.parse().map_err(|_| "Failed to parse x position")?;
        let y = y.parse().map_err(|_| "Failed to parse y position")?;

        Ok(Self { x, y })
    }
}

/* ---------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    instr_type: InstructionType,
    from: Position,
    to: Position,
}

impl FromStr for Instruction {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let mut instr_type = parts.next().ok_or("instruction type not found")?;

        instr_type = if instr_type == "turn" {
            parts.next().ok_or("turn intruction type not found")?
        } else {
            instr_type
        };
        let instr_type = instr_type.parse()?;
        let from = parts.next().ok_or("form position nt found")?.parse()?;

        parts.next();

        let to = parts.next().ok_or("form position nt found")?.parse()?;

        Ok(Self {
            instr_type,
            from,
            to,
        })
    }
}

/* ---------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstructionType {
    Off,
    On,
    Toggle,
}

impl FromStr for InstructionType {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "toggle" => Ok(Self::Toggle),
            _ => Err("Invalid instruction type"),
        }
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_position() {
        assert_eq!("0,0".parse(), Ok(Position { x: 0, y: 0 }));
        assert_eq!("499,499".parse(), Ok(Position { x: 499, y: 499 }));
        assert_eq!("999,999".parse(), Ok(Position { x: 999, y: 999 }));
    }

    #[test]
    fn parse_instruction() {
        let truth = Instruction {
            instr_type: InstructionType::On,
            from: Position { x: 0, y: 0 },
            to: Position { x: 999, y: 999 },
        };
        assert_eq!("turn on 0,0 through 999,999".parse(), Ok(truth));

        let truth = Instruction {
            instr_type: InstructionType::Toggle,
            from: Position { x: 0, y: 0 },
            to: Position { x: 999, y: 0 },
        };
        assert_eq!("toggle 0,0 through 999,0".parse(), Ok(truth));

        let truth = Instruction {
            instr_type: InstructionType::Off,
            from: Position { x: 499, y: 499 },
            to: Position { x: 500, y: 500 },
        };
        assert_eq!("turn off 499,499 through 500,500".parse(), Ok(truth));
    }
}

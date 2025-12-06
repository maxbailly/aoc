use std::str::FromStr;

/* ---------- */

const INPUT: &str = include_str!("../../../inputs/2015/day02.txt");

/* ---------- */

pub(crate) fn resolve() {
    println!("[2015 DAY1]");

    let presents = INPUT
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse a present line");

    println!("Part1 => {}", part1(&presents));
    println!("Part2 => {}", part2(&presents));
}

/* ---------- */

fn part1(presents: &[Present]) -> u64 {
    presents
        .iter()
        .map(|present| present.surface_area() + present.min_area())
        .sum()
}

/* ---------- */

fn part2(presents: &[Present]) -> u64 {
    presents
        .iter()
        .map(|present| present.min_perimeter() + present.volume())
        .sum()
}

/* ---------- */

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
struct Present {
    length: u64,
    width: u64,
    height: u64,
}

impl Present {
    fn areas(&self) -> [u64; 3] {
        [
            self.length * self.width,
            self.length * self.height,
            self.width * self.height,
        ]
    }

    fn surface_area(&self) -> u64 {
        let areas = self.areas();
        areas.iter().map(|area| area * 2).sum()
    }

    fn min_area(&self) -> u64 {
        self.areas()
            .into_iter()
            .min()
            .expect("areas should not be empty")
    }

    fn volume(&self) -> u64 {
        self.length * self.width * self.height
    }

    fn min_perimeter(&self) -> u64 {
        let perimeters = [
            (self.length + self.width) * 2,
            (self.length + self.height) * 2,
            (self.width + self.height) * 2,
        ];

        perimeters
            .into_iter()
            .min()
            .expect("perimeters should not be empty")
    }
}

impl FromStr for Present {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('x');

        let length = parts
            .next()
            .ok_or("missing lenght")?
            .parse()
            .map_err(|_| "failed to parse length")?;
        let width = parts
            .next()
            .ok_or("missing width")?
            .parse()
            .map_err(|_| "failed to parse width")?;
        let height = parts
            .next()
            .ok_or("missing height")?
            .parse()
            .map_err(|_| "failed to parse height")?;

        Ok(Self {
            length,
            width,
            height,
        })
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::Present;

    #[test]
    fn present_parsing() {
        let truth = Present {
            length: 1,
            width: 2,
            height: 3,
        };

        assert_eq!("1x2x3".parse(), Ok(truth))
    }

    #[test]
    fn part1() {
        let present = "2x3x4".parse().expect("failed to parse a present");
        assert_eq!(super::part1(&[present]), 58);
        let present = "1x1x10".parse().expect("failed to parse a present");
        assert_eq!(super::part1(&[present]), 43);
    }

    #[test]
    fn part2() {
        let present = "2x3x4".parse().expect("failed to parse a present");
        assert_eq!(super::part2(&[present]), 34);
        let present = "1x1x10".parse().expect("failed to parse a present");
        assert_eq!(super::part2(&[present]), 14);
    }
}

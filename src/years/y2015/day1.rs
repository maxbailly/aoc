const INPUT: &[u8] = include_bytes!("../../../inputs/2015/day1.txt");

/* ---------- */

pub(crate) fn resolve() {
    println!("[2015 DAY1]");
    println!("Part1 => {}", part1(INPUT));
    println!("Part2 => {}", part2(INPUT));
}

/* ---------- */

fn part1(input: &[u8]) -> i64 {
    FloorIterator::from(input)
        .last()
        .expect("input shouldn't be empty")
}

/* ---------- */

fn part2(input: &[u8]) -> u64 {
    1 + FloorIterator::from(input)
        .take_while(|floor| *floor != -1)
        .count() as u64
}

/* ---------- */

struct FloorIterator<'a> {
    floor: i64,
    instructions: &'a [u8],
}

impl<'a> From<&'a [u8]> for FloorIterator<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self {
            floor: 0,
            instructions: value,
        }
    }
}

impl Iterator for FloorIterator<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.instructions.first().map(|instr| {
            self.floor += match instr {
                b')' => -1,
                b'(' => 1,
                b'\n' => 0,
                _ => unreachable!("invalid character: {instr}"),
            };

            self.floor
        });

        if !self.instructions.is_empty() {
            self.instructions = &self.instructions[1..];
        }

        ret
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(b"(())"), 0);
        assert_eq!(super::part1(b"()()"), 0);
        assert_eq!(super::part1(b"((("), 3);
        assert_eq!(super::part1(b"(()(()("), 3);
        assert_eq!(super::part1(b"))((((("), 3);
        assert_eq!(super::part1(b"())"), -1);
        assert_eq!(super::part1(b"))("), -1);
        assert_eq!(super::part1(b")))"), -3);
        assert_eq!(super::part1(b")())())"), -3);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(b")"), 1);
        assert_eq!(super::part2(b"()())"), 5);
    }
}

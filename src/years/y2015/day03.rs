use std::collections::HashSet;

/* ---------- */

const DIRECTIONS: &[u8] = include_bytes!("../../../inputs/2015/day03.txt");

/* ---------- */

pub(crate) fn resolve() {
    println!("[2015 DAY3]");

    let directions = DIRECTIONS
        .iter()
        .filter(|dir| matches!(dir, b'^' | b'v' | b'>' | b'<'))
        .map(|dir| (*dir).into())
        .collect::<Vec<_>>();

    println!("Part1 => {}", part1(&directions));
    println!("Part2 => {}", part2(&directions));
}

/* ---------- */

fn part1(directions: &[Direction]) -> u64 {
    let mut current_pos = Position::default();
    let mut homes_visited = HashSet::with_capacity(DIRECTIONS.len());

    homes_visited.insert(current_pos);

    directions.iter().for_each(|dir| {
        current_pos.move_to(*dir);
        homes_visited.insert(current_pos);
    });

    homes_visited.len() as u64
}

/* ---------- */

fn part2(directions: &[Direction]) -> u64 {
    let mut current_pos = Position::default();
    let mut homes_visited = HashSet::with_capacity(DIRECTIONS.len());

    homes_visited.insert(current_pos);

    let santa_dirs = directions.iter().step_by(2);
    santa_dirs.for_each(|dir| {
        current_pos.move_to(*dir);
        homes_visited.insert(current_pos);
    });

    current_pos = Position::default();
    let robo_dirs = directions.iter().skip(1).step_by(2);
    robo_dirs.for_each(|dir| {
        current_pos.move_to(*dir);
        homes_visited.insert(current_pos);
    });

    homes_visited.len() as u64
}

/* ---------- */

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Self::North,
            b'v' => Self::South,
            b'>' => Self::East,
            b'<' => Self::West,
            _ => unreachable!("invalid character: {value}"),
        }
    }
}

/* ---------- */

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn move_to(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
        }
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let dir = b"^v"
            .iter()
            .filter(|dir| matches!(dir, b'^' | b'v' | b'>' | b'<'))
            .map(|dir| (*dir).into())
            .collect::<Vec<_>>();
        assert_eq!(super::part1(&dir), 2);

        let dir = b"^>v<"
            .iter()
            .filter(|dir| matches!(dir, b'^' | b'v' | b'>' | b'<'))
            .map(|dir| (*dir).into())
            .collect::<Vec<_>>();
        assert_eq!(super::part1(&dir), 4);

        let dir = b"^v^v^v^v^v"
            .iter()
            .filter(|dir| matches!(dir, b'^' | b'v' | b'>' | b'<'))
            .map(|dir| (*dir).into())
            .collect::<Vec<_>>();
        assert_eq!(super::part1(&dir), 2);
    }

    #[test]
    fn part2() {
        let dir = b"^v"
            .iter()
            .filter(|dir| matches!(dir, b'^' | b'v' | b'>' | b'<'))
            .map(|dir| (*dir).into())
            .collect::<Vec<_>>();
        assert_eq!(super::part2(&dir), 3);

        let dir = b"^>v<"
            .iter()
            .filter(|dir| matches!(dir, b'^' | b'v' | b'>' | b'<'))
            .map(|dir| (*dir).into())
            .collect::<Vec<_>>();
        assert_eq!(super::part2(&dir), 3);

        let dir = b"^v^v^v^v^v"
            .iter()
            .filter(|dir| matches!(dir, b'^' | b'v' | b'>' | b'<'))
            .map(|dir| (*dir).into())
            .collect::<Vec<_>>();
        assert_eq!(super::part2(&dir), 11);
    }
}

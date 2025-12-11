const INPUT: &[u8] = include_bytes!("../../../inputs/2015/day05.txt");

/* ---------- */

pub(crate) fn resolve() {
    println!("[2015 DAY4]");
    println!("Part1 => {}", part1());
    println!("Part2 => {}", part2());
}

/* ---------- */

fn part1() -> u64 {
    INPUT
        .split(|c| *c == b'\n')
        .filter(|line| part1_filter(line))
        .count() as u64
}

fn part1_filter(line: &[u8]) -> bool {
    #[inline(always)]
    fn is_vowel(c: u8) -> bool {
        matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
    }

    if line.is_empty() {
        return false;
    }

    let mut nb_vowels = if is_vowel(line[line.len() - 1]) { 1 } else { 0 };
    let mut nb_double_letters = 0u8;

    for window in line.windows(2) {
        if matches!(&window, &b"ab" | &b"cd" | &b"pq" | &b"xy") {
            return false;
        }

        if is_vowel(window[0]) {
            nb_vowels += 1
        }

        if window[0] == window[1] {
            nb_double_letters += 1
        }
    }

    nb_vowels >= 3 && nb_double_letters > 0
}

/* ---------- */

fn part2() -> u64 {
    INPUT
        .split(|c| *c == b'\n')
        .filter(|line| part2_filter(line))
        .count() as u64
}

fn part2_filter(line: &[u8]) -> bool {
    if line.is_empty() {
        return false;
    }

    let mut has_duplicates = false;
    let mut has_in_between = false;

    for (pos, window) in line.windows(3).enumerate() {
        if window[0] == window[2] {
            has_in_between = true;
        }

        if !has_duplicates {
            let haystack = &line[pos + 2..];
            has_duplicates = haystack.windows(2).any(|win| win == &window[..2]);
        }
    }

    has_duplicates && has_in_between
}

/* ---------- */

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert!(super::part1_filter(b"ugknbfddgicrmopn"));
        assert!(super::part1_filter(b"aaa"));
        assert!(!super::part1_filter(b"jchzalrnumimnmhp"));
        assert!(!super::part1_filter(b"haegwjzuvuyypxyu"));
        assert!(!super::part1_filter(b"dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2() {
        assert!(super::part2_filter(b"qjhvhtzxzqqjkmpb"));
        assert!(super::part2_filter(b"xxyxx"));
        assert!(super::part2_filter(b"aabcdefegaa"));
        assert!(super::part2_filter(b"xyxy"));
        assert!(!super::part2_filter(b"uurcxstgmygtbstg"));
        assert!(!super::part2_filter(b"ieodomkazucvgmuy"));
        assert!(!super::part2_filter(b"aaa"));
    }
}

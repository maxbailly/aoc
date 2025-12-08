use md5::{Digest, Md5};

/* ---------- */

const INPUT: &[u8] = b"bgvyzdsv";

/* ---------- */

pub(crate) fn resolve() {
    println!("[2015 DAY4]");
    println!("Part1 => {}", part1());
    println!("Part2 => {}", part2());
}

/* ---------- */

fn part1() -> u64 {
    fn win_cond(hash: &[u8]) -> bool {
        hash[0] == 0 && hash[1] == 0 && hash[2] >> 4 == 0
    }

    find_hash(win_cond)
}

/* ---------- */

fn part2() -> u64 {
    fn win_cond(hash: &[u8]) -> bool {
        hash[0] == 0 && hash[1] == 0 && hash[2] == 0
    }

    find_hash(win_cond)
}

/* ---------- */

fn find_hash(predicate: impl Fn(&[u8]) -> bool) -> u64 {
    let mut n = 0;

    loop {
        let mut data = INPUT.to_vec();
        data.extend(n.to_string().as_bytes());

        let mut md5 = Md5::new();
        md5.update(data);

        let hash = md5.finalize();
        let hash = &hash.as_slice()[..3];

        if predicate(hash) {
            return n;
        }

        n += 1;
    }
}

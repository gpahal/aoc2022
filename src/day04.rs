use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day04/part1.txt");
    let lines = data.trim().split("\n");
    let full_contains_list = lines
        .map(|line| {
            let mut iter = line.split(",");
            let range_1 = Range::parse(iter.next()?)?;
            let range_2 = Range::parse(iter.next()?)?;
            Some(range_1.fully_contains(&range_2) || range_2.fully_contains(&range_1))
        })
        .map(|x| if let Some(x) = x { x } else { false });
    let total_score = full_contains_list.filter(|x| *x).count();
    println!("Total score: {total_score}");
}

pub fn part2() {
    let data = must_read_file("data/day04/part2.txt");
    let lines = data.trim().split("\n");
    let partially_contains_list = lines
        .map(|line| {
            let mut iter = line.split(",");
            let range_1 = Range::parse(iter.next()?)?;
            let range_2 = Range::parse(iter.next()?)?;
            Some(range_1.partially_contains(&range_2) || range_2.partially_contains(&range_1))
        })
        .map(|x| if let Some(x) = x { x } else { false });
    let total_score = partially_contains_list.filter(|x| *x).count();
    println!("Total score: {total_score}");
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn parse(s: &str) -> Option<Self> {
        let mut iter = s.trim().split('-');
        let start = iter.next()?.parse::<u64>().ok()?;
        let end = iter.next()?.parse::<u64>().ok()?;
        Some(Self::new(start, end))
    }

    fn partially_contains(&self, value: &Self) -> bool {
        (self.start <= value.start && self.end >= value.start)
            || (self.start <= value.end && self.end >= value.end)
    }

    fn fully_contains(&self, value: &Self) -> bool {
        self.start <= value.start && self.end >= value.end
    }
}

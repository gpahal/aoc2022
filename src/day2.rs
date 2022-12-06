use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day2/part1.txt");
    let lines = data.trim().split("\n");
    let scores = lines.map(|line| {
        let parts = line.split(" ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return 0;
        }

        let elf_selection = parts[0];
        let my_selection = parts[1];
        match (elf_selection, my_selection) {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            ("C", "Y") => 2 + 0,
            ("C", "Z") => 3 + 3,
            _ => 0,
        }
    });
    let total_score = scores.sum::<u64>();
    println!("Total score: {total_score}");
}

pub fn part2() {
    let data = must_read_file("data/day2/part2.txt");
    let lines = data.trim().split("\n");
    let scores = lines.map(|line| {
        let parts = line.split(" ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return 0;
        }

        let elf_selection = parts[0];
        let my_selection = parts[1];
        match (elf_selection, my_selection) {
            ("A", "X") => 0 + 3,
            ("A", "Y") => 3 + 1,
            ("A", "Z") => 6 + 2,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 0 + 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 6 + 1,
            _ => 0,
        }
    });
    let total_score = scores.sum::<u64>();
    println!("Total score: {total_score}");
}

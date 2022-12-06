use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day1/part1.txt");
    let line_groups = data.trim().split("\n\n");
    let max_total_calories = line_groups.map(get_total_calories).max().unwrap_or(0);
    println!("Max total calories: {max_total_calories}");
}

pub fn part2() {
    let data = must_read_file("data/day1/part2.txt");
    let line_groups = data.trim().split("\n\n");
    let mut total_calories_list = line_groups.map(get_total_calories).collect::<Vec<_>>();
    total_calories_list.sort_by(|a, b| b.cmp(a));
    let total_calories_top_3 = total_calories_list[0..3].iter().sum::<u64>();
    println!("Sum of top 2 total calories: {total_calories_top_3}");
}

fn get_total_calories(line_group: &str) -> u64 {
    let lines = line_group.split("\n").map(|line| line.trim());
    lines.map(|line| line.parse::<u64>().unwrap()).sum::<u64>()
}

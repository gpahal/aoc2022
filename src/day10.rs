use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day10/part1.txt");
    let x_values = parse_x_values(&data);

    let score = get_total_signal_value(&x_values, &[18, 58, 98, 138, 178, 218]);
    println!("Score: {score}");
}

pub fn part2() {
    let data = must_read_file("data/day10/part2.txt");
    let x_values = parse_x_values(&data);

    let mut crt = [['.'; 40]; 6];
    for i in 0..6 {
        for j in 0..40 {
            let cycle = i * 40 + j + 1;
            let x_before_cycle = if cycle > 1 { x_values[cycle - 2] } else { 1 };
            if (x_before_cycle - j as i64).abs() < 2 {
                crt[i][j] = '#';
            }
        }
    }

    for i in 0..6 {
        for j in 0..40 {
            print!("{}", crt[i][j]);
        }
        println!();
    }
}

fn parse_x_values(data: &str) -> Vec<i64> {
    let mut x_values = Vec::new();
    let mut prev_x = 1i64;
    let lines = data.trim().split("\n");
    lines.for_each(|line| {
        let line = line.trim();
        x_values.push(prev_x);
        let new_x = prev_x
            + if line.starts_with("addx ") {
                if let Ok(diff) = line[5..].parse::<i64>() {
                    x_values.push(prev_x + diff);
                    diff
                } else {
                    0
                }
            } else {
                0
            };
        prev_x = new_x;
    });
    x_values
}

fn get_total_signal_value(x_values: &Vec<i64>, i_list: &[usize]) -> i64 {
    let mut total_signal_value = 0i64;
    for curr_i in i_list {
        total_signal_value += x_values[*curr_i] * (*curr_i + 2) as i64;
    }
    total_signal_value
}

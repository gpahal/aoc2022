use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day08/part1.txt");
    let lines = data.trim().split("\n");
    let grid = lines
        .map(|l| l.trim())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if grid.len() == 0 || grid[0].len() == 0 {
        println!("Empty grid");
        return;
    }

    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut visible_grid = Vec::new();
    for i in 0..row_count {
        let mut curr_row = Vec::new();
        for j in 0..col_count {
            curr_row.push(i == 0 || j == 0 || i == grid.len() - 1 || j == grid[i].len() - 1);
        }
        visible_grid.push(curr_row);
    }

    for i in 1..(row_count - 1) {
        let mut max = grid[i][0];
        for j in 1..(col_count - 1) {
            let curr = grid[i][j];
            if curr > max {
                max = curr;
                visible_grid[i][j] = true;
            }
        }
    }

    for i in 1..(row_count - 1) {
        let mut max = grid[i][col_count - 1];
        for j in (1..(col_count - 1)).rev() {
            let curr = grid[i][j];
            if curr > max {
                max = curr;
                visible_grid[i][j] = true;
            }
        }
    }

    for j in 1..(col_count - 1) {
        let mut max = grid[0][j];
        for i in 1..(row_count - 1) {
            let curr = grid[i][j];
            if curr > max {
                max = curr;
                visible_grid[i][j] = true;
            }
        }
    }

    for j in 1..(col_count - 1) {
        let mut max = grid[row_count - 1][j];
        for i in (1..(row_count - 1)).rev() {
            let curr = grid[i][j];
            if curr > max {
                max = curr;
                visible_grid[i][j] = true;
            }
        }
    }

    let visible_count = visible_grid
        .iter()
        .map(|r| r.iter().filter(|v| **v).count())
        .sum::<usize>();
    println!("Visible grid count: {visible_count}");
}

pub fn part2() {
    let data = must_read_file("data/day08/part2.txt");
    let lines = data.trim().split("\n");
    let grid = lines
        .map(|l| l.trim())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if grid.len() == 0 || grid[0].len() == 0 {
        println!("Empty grid");
        return;
    }

    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut grid_scores = Vec::new();
    for i in 0..row_count {
        let mut curr_row = Vec::new();
        for j in 0..col_count {
            if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[i].len() - 1 {
                curr_row.push(0);
            } else {
                curr_row.push(1);
            }
        }
        grid_scores.push(curr_row);
    }

    for i in 1..(row_count - 1) {
        let row_scores = get_iter_scores(grid[i][0], &grid[i][1..col_count - 1], false);
        let row_scores_rev =
            get_iter_scores(grid[i][col_count - 1], &grid[i][1..col_count - 1], true);
        for j in 1..(col_count - 1) {
            grid_scores[i][j] *= row_scores[j - 1] * row_scores_rev[j - 1];
        }
    }

    for j in 1..(col_count - 1) {
        let mut curr_arr = Vec::new();
        for i in 1..(row_count - 1) {
            curr_arr.push(grid[i][j]);
        }
        let col_scores = get_iter_scores(grid[0][j], &curr_arr, false);
        let col_scores_reverse = get_iter_scores(grid[row_count - 1][j], &curr_arr, true);
        for i in 1..(row_count - 1) {
            grid_scores[i][j] *= col_scores[i - 1] * col_scores_reverse[i - 1];
        }
    }

    println!(
        "Highest scores: {}",
        grid_scores
            .iter()
            .map(|r| r.iter().max().unwrap())
            .max()
            .unwrap()
    );
}

fn get_iter_scores(start: u32, arr: &[u32], reverse: bool) -> Vec<usize> {
    let len = arr.len();
    let mut prev = start;
    let mut scores = Vec::new();
    for i in 0..arr.len() {
        let curr = if reverse { arr[len - 1 - i] } else { arr[i] };
        if curr > prev {
            let mut found = false;
            for j in (0..i).rev() {
                let curr_prev = if reverse { arr[len - 1 - j] } else { arr[j] };
                if curr_prev >= curr {
                    scores.push(i - j);
                    found = true;
                    break;
                }
            }
            if !found {
                scores.push(i + 1);
            }
        } else {
            scores.push(1);
        }
        prev = curr
    }

    if reverse {
        scores.reverse();
    }
    scores
}

use std::{collections::VecDeque, str::Lines};

use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day11/part1.txt");
    let mut monkeys = Monkeys::parse(&data);
    monkeys.do_rounds(20, true);
    println!("Monkey business: {}", monkeys.get_monkey_business());
}

pub fn part2() {
    let data = must_read_file("data/day11/part2.txt");
    let mut monkeys = Monkeys::parse(&data);
    monkeys.do_rounds(10000, false);
    println!("Monkey business: {}", monkeys.get_monkey_business());
}

#[derive(Debug, Clone)]
struct Test {
    divisibe_by: u64,
    throw_to_on_true: usize,
    throw_to_on_false: usize,
}

#[derive(Debug, Clone)]
enum OpType {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum Value {
    Arg,
    Literal(u64),
}

impl Value {
    fn parse(str: &str) -> Option<Value> {
        let str = str.trim();
        if str == "old" {
            Some(Value::Arg)
        } else if let Ok(value) = str.parse::<u64>() {
            Some(Value::Literal(value))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Op {
    op_type: OpType,
    left: Value,
    right: Value,
}

impl Op {
    fn evaluate(&self, arg: u64) -> u64 {
        match self.op_type {
            OpType::Add => match self.left {
                Value::Arg => {
                    arg + match self.right {
                        Value::Arg => arg,
                        Value::Literal(value) => value,
                    }
                }
                Value::Literal(value) => {
                    value
                        + match self.right {
                            Value::Arg => arg,
                            Value::Literal(value) => value,
                        }
                }
            },
            OpType::Multiply => match self.left {
                Value::Arg => {
                    arg * match self.right {
                        Value::Arg => arg,
                        Value::Literal(value) => value,
                    }
                }
                Value::Literal(value) => {
                    value
                        * match self.right {
                            Value::Arg => arg,
                            Value::Literal(value) => value,
                        }
                }
            },
        }
    }
}

struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test: Test,

    inspected_count: u64,
}

struct Monkeys {
    monkeys: Vec<Monkey>,
    lcm_divisible_by: u64,
}

impl Monkey {
    fn parse(lines: &mut Lines) -> Option<Monkey> {
        let mut items = VecDeque::new();
        let mut op = Op {
            op_type: OpType::Add,
            left: Value::Arg,
            right: Value::Literal(0),
        };
        let mut test = Test {
            divisibe_by: 0,
            throw_to_on_true: 0,
            throw_to_on_false: 0,
        };

        let mut found_first_line = false;
        for line in lines {
            let line = line.trim();
            if line.len() == 0 {
                if found_first_line {
                    break;
                } else {
                    continue;
                }
            } else if line.starts_with("Monkey ") {
                found_first_line = true;
                continue;
            } else if line.starts_with("Starting items:") {
                line[15..].split(",").for_each(|item| {
                    if let Ok(item) = item.trim().parse() {
                        items.push_back(item);
                    }
                });
            } else if line.starts_with("Operation:") {
                let line = line[10..].trim();
                if line.starts_with("new = ") {
                    let line = line[6..].trim();
                    let parts = line.split(" ").collect::<Vec<_>>();
                    if parts.len() == 3 {
                        if parts[1] == "+" {
                            op.op_type = OpType::Add;
                        } else if parts[1] == "*" {
                            op.op_type = OpType::Multiply;
                        }

                        op.left = Value::parse(parts[0])?;
                        op.right = Value::parse(parts[2])?;
                    }
                }
            } else if line.starts_with("Test: divisible by") {
                if let Ok(divisible_by) = line[18..].trim().parse() {
                    test.divisibe_by = divisible_by;
                }
            } else if line.starts_with("If true: throw to monkey") {
                if let Ok(throw_to_on_true) = line[24..].trim().parse::<usize>() {
                    test.throw_to_on_true = throw_to_on_true;
                }
            } else if line.starts_with("If false: throw to monkey") {
                if let Ok(throw_to_on_false) = line[25..].trim().parse::<usize>() {
                    test.throw_to_on_false = throw_to_on_false;
                }
            }
        }
        if found_first_line {
            Some(Monkey {
                items,
                op,
                test,
                inspected_count: 0,
            })
        } else {
            None
        }
    }

    fn print(&self) {
        println!("Monkey {{");
        println!("    Items: {:?}", self.items);
        println!("    Operations: {:?}", self.op);
        println!("    Test: divisible by {}", self.test.divisibe_by);
        println!(
            "        If true: throw to monkey {}",
            self.test.throw_to_on_true
        );
        println!(
            "        If false: throw to monkey {}",
            self.test.throw_to_on_false
        );
        println!("    Inspected count: {}", self.inspected_count);
        println!("}}");
    }
}

impl Monkeys {
    fn parse(data: &str) -> Monkeys {
        let mut lines = data.trim().lines();
        let mut lcm_divisible_by = 1u64;
        let mut monkeys = Vec::new();
        loop {
            if let Some(monkey) = Monkey::parse(&mut lines) {
                lcm_divisible_by = lcm(lcm_divisible_by, monkey.test.divisibe_by);
                monkeys.push(monkey);
            } else {
                break;
            }
        }
        Monkeys {
            monkeys,
            lcm_divisible_by,
        }
    }

    fn do_rounds(&mut self, rounds: u32, divide_by_3: bool) {
        for _ in 0..rounds {
            self.do_round(divide_by_3);
        }
    }

    fn do_round(&mut self, divide_by_3: bool) {
        for index in 0..self.monkeys.len() {
            self.inspect_all(index, divide_by_3);
        }
    }

    fn inspect_all(&mut self, index: usize, divide_by_3: bool) {
        while self.inspect_single(index, divide_by_3).is_some() {}
    }

    fn inspect_single(&mut self, index: usize, divide_by_3: bool) -> Option<()> {
        let monkey = &mut self.monkeys[index];
        let item = monkey.items.pop_front()?;
        monkey.inspected_count += 1;
        let item = monkey.op.evaluate(item);
        let item = if divide_by_3 {
            item / 3
        } else {
            item % self.lcm_divisible_by
        };

        let test = self.monkeys[index].test.clone();
        if item % test.divisibe_by == 0 {
            self.monkeys[test.throw_to_on_true].items.push_back(item);
        } else {
            self.monkeys[test.throw_to_on_false].items.push_back(item);
        }
        Some(())
    }

    fn get_monkey_business(&self) -> u64 {
        let mut inspected_counts = self
            .monkeys
            .iter()
            .map(|m| m.inspected_count)
            .collect::<Vec<_>>();
        inspected_counts.sort_by(|a, b| b.cmp(a));
        inspected_counts[0..2].iter().product::<u64>()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (i, monkey) in self.monkeys.iter().enumerate() {
            print!("{i}: ");
            monkey.print();
            println!();
        }
    }
}

fn gcd(n1: u64, n2: u64) -> u64 {
    let mut x = n1;
    let mut y = n2;
    if n1 < n2 {
        x = n2;
        y = n1;
    }

    let mut rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }
    y
}

fn lcm(n1: u64, n2: u64) -> u64 {
    (n1 / gcd(n1, n2)) * n2
}

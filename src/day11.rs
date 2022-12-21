use std::iter;

struct Monkey {
    items: Vec<u64>,
    oper: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    test_val: u64,
    target_true: usize,
    target_false: usize,
}

fn read_monkey(input: &str) -> Monkey {
    let lines: Vec<&str> = input.lines().collect();

    let items = lines[1]
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect();

    let oper_data: Vec<&str> = lines[2].split_whitespace().skip(4).collect();
    let oper: Box<dyn Fn(u64) -> u64> = if oper_data[0] == "*" && oper_data[1] == "old" {
        Box::new(|old| old * old)
    } else {
        let val: u64 = oper_data[1].parse().unwrap();
        if oper_data[0] == "+" {
            Box::new(move |old| old + val)
        } else {
            Box::new(move |old| old * val)
        }
    };

    let test_val: u64 = lines[3]
        .strip_prefix("  Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();
    let test = Box::new(move |x| x % test_val == 0);

    let target_true = lines[4]
        .strip_prefix("    If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    let target_false = lines[5]
        .strip_prefix("    If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    Monkey {
        items,
        oper,
        test,
        test_val,
        target_true,
        target_false,
    }
}

fn read_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(read_monkey).collect()
}

pub fn run_a(input: &str) {
    let mut monkeys = read_monkeys(input);
    let mut counts = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut true_items = vec![];
            let mut false_items = vec![];
            let monkey = &mut monkeys[i];
            for &item in &monkey.items {
                let inspect = (monkey.oper)(item) / 3;
                if (monkey.test)(inspect) {
                    true_items.push(inspect);
                } else {
                    false_items.push(inspect);
                }
                counts[i] += 1;
            }
            monkey.items.clear();
            let target_true = monkey.target_true;
            let target_false = monkey.target_false;
            monkeys[target_true].items.extend_from_slice(&true_items);
            monkeys[target_false].items.extend_from_slice(&false_items);
        }
    }

    counts.sort();
    println!("Monkey business: {}", counts[counts.len() - 2] * counts[counts.len() - 1]);
}

pub fn run_b(input: &str) {
    let mut monkeys = read_monkeys(input);
    let mut counts = vec![0u64; monkeys.len()];

    let mut wrap = 1;
    for monkey in &monkeys {
        wrap *= monkey.test_val;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut true_items = vec![];
            let mut false_items = vec![];
            let monkey = &mut monkeys[i];
            for &item in &monkey.items {
                let inspect = (monkey.oper)(item) % wrap;
                if (monkey.test)(inspect) {
                    true_items.push(inspect);
                } else {
                    false_items.push(inspect);
                }
                counts[i] += 1;
            }
            monkey.items.clear();
            let target_true = monkey.target_true;
            let target_false = monkey.target_false;
            monkeys[target_true].items.extend_from_slice(&true_items);
            monkeys[target_false].items.extend_from_slice(&false_items);
        }
    }

    counts.sort();
    println!("Monkey business: {}", counts[counts.len() - 2] * counts[counts.len() - 1]);
}

fn process() -> i32 {
    let instructions = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (op, amount_str): (&str, &str) = line.split_once(" ").unwrap();
            let amount = amount_str.parse::<i32>().unwrap();
            match op {
                "forward" => (amount, 0),
                "up" => (0, -amount),
                "down" => (0, amount),
                _ => (0, 0),
            }
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    instructions.0 * instructions.1
}

fn main() {
    println!("{}", process());
}

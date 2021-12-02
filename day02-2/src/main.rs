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
        .fold((0, 0, 0), |acc, item| {
            let horizontal = acc.0 + item.0;
            let aim = acc.1 + item.1;
            let depth = acc.2 + aim * item.0;
            (horizontal, aim, depth)
        });

    instructions.0 * instructions.2
}

fn main() {
    println!("{}", process());
}

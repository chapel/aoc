fn slice_window() -> i32 {
    let depth_numbers = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .as_slice()
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<i32>>();

    let mut windows_depth_numbers = depth_numbers.as_slice().windows(2);

    let mut count_increased = 0;
    #[allow(clippy::while_let_on_iterator)]
    while let Some([last_depth, depth]) = windows_depth_numbers.next() {
        if depth > last_depth {
            count_increased += 1;
        }
    }

    count_increased
}

fn main() {
    println!("Slice window count increased: {}", slice_window());
}

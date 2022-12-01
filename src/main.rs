use std::fs;
fn main() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Something went wrong reading the file");
    let answer = part1(&contents);
    println!("The answer is {}", answer)
}

fn part1(input: &str) -> u32 {
    let line_groups = input.split("\n\n");
    let max_calories = line_groups
        .map(|group| {
            group
                .split("\n")
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    return max_calories;
}

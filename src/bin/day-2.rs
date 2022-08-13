use aoc_2021::day_2;
fn main() {
    let moves:Vec<day_2::Vector> = include_str!("inputs/day-2.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    println!("Final Position: {}", day_2::final_destination(&moves));
}

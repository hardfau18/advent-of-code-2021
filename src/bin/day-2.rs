use aoc_2021::day_2;
fn main() {
    let moves: Vec<day_2::Course> = include_str!("inputs/day-2.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut sm = day_2::SubMarine::default();
    moves.iter().for_each(|movement| sm.advance_without_aim(movement));
    println!("Part-1: Final destination - {}", sm.final_destination());
    let mut sm = day_2::SubMarine::default();
    moves.iter().for_each(|movement| sm.advance(movement));
    println!("Part-1: Final destination - {}", sm.final_destination());
}

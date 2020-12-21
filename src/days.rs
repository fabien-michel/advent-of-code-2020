mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17_01;
mod day17_02;

pub const DAYS: [[fn(); 2]; 16] = [
    [day01::day01_01, day01::day01_02],
    [day02::day02_01, day02::day02_02],
    [day03::day03_01, day03::day03_02],
    [day04::day04_01, day04::day04_02],
    [day05::day05_01, day05::day05_02],
    [day06::day06_01, day06::day06_02],
    [day07::day07_01, day07::day07_02],
    [day08::day08_01, day08::day08_02],
    [day09::day09_01, day09::day09_02],
    [day10::day10_01, day10::day10_02],
    [day11::day11_01, day11::day11_02],
    [day12::day12_01, day12::day12_02],
    [day13::day13_01, day13::day13_02],
    [day14::day14_01, day14::day14_02],
    [day15::day15_01, day15::day15_02],
    [day16::day16_01, day16::day16_02],
    [day17_01::day17_01, day17_02::day17_02],
];

pub fn get_day_runs(day_number: usize, puzzle_number: usize) -> Vec<fn()> {
    if puzzle_number == 0 {
        return Vec::from(DAYS[day_number - 1]);
    }
    return vec![DAYS[day_number - 1][puzzle_number - 1]];
}

pub fn get_all_day_runs() -> Vec<fn()> {
    let mut all_day_runs: Vec<fn()> = vec![];

    for day_runs in DAYS.iter() {
        for day_run in day_runs {
            all_day_runs.push(*day_run);
        }
    }
    return all_day_runs;
}

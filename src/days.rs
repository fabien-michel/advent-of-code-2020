mod day01;
mod day02;
mod day03;

pub const DAYS: [[fn(); 2]; 3] = [
    [day01::day01_01, day01::day01_02],
    [day02::day02_01, day02::day02_02],
    [day03::day03_01, day03::day03_02],
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

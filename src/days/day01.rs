// use crate::utils::read_lines;
// mod utils;
use crate::utils::print_day_banner;
use crate::utils::read_lines;
use itertools::iproduct;

pub fn day01_01() {
    print_day_banner(1, 1);
    let mut expenses = load_expenses();
    expenses.sort();
    for (exp_1, exp_2) in iproduct!(expenses.iter(), expenses.iter()) {
        let (exp_sum, exp_prod) = sum_prod_expenses(&[exp_1, exp_2]);
        if exp_sum == 2020 {
            println!(
                "({:?} ; {:?}) : {:?} :  {:?}",
                exp_1, exp_2, exp_sum, exp_prod
            );
            break;
        }
    }
}

pub fn day01_02() {
    print_day_banner(1, 2);
    let mut expenses = load_expenses();
    expenses.sort();
    for (exp_1, exp_2, exp_3) in iproduct!(expenses.iter(), expenses.iter(), expenses.iter()) {
        let (exp_sum, exp_prod) = sum_prod_expenses(&[exp_1, exp_2, exp_3]);
        if exp_sum == 2020 {
            println!(
                "({:?} ; {:?} ; {:?}) : {:?}: {:?}",
                exp_1, exp_2, exp_3, exp_sum, exp_prod
            );
            break;
        }
    }
}

fn sum_prod_expenses(expenses: &[&i64]) -> (i64, i64) {
    let sum: i64 = expenses.iter().copied().sum();
    let prod: i64 = expenses.iter().copied().product();
    return (sum, prod);
}

fn load_expenses() -> Vec<i64> {
    return read_lines("./inputs/01")
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
}

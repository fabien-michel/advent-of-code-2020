use crate::utils::print_day_banner;
use crate::utils::read_lines;
use regex::Captures;
use regex::Regex;

pub fn day18_01() {
    print_day_banner(18, 1);
    let expressions = load_expressions();
    let result: isize = expressions.into_iter().map(resolve_expression_01).sum();
    println!("Total sum: {:?}", result);
}

pub fn day18_02() {
    print_day_banner(18, 2);
    let expressions = load_expressions();
    let result: isize = expressions.into_iter().map(resolve_expression_02).sum();
    println!("Total sum: {:?}", result);
}

fn resolve_expression_01(expression: String) -> isize {
    lazy_static! {
        static ref RE_COMPUTE: Regex =
            Regex::new(r"(?P<v1>\d+) (?P<op>[+|*]) (?P<v2>\d+)").unwrap();
    }

    let mut expression = resolve_parentheses(expression, resolve_expression_01);
    expression = resolve_operations(expression, &RE_COMPUTE, |v1, v2, op| match op {
        "+" => v1 + v2,
        "*" => v1 * v2,
        _ => panic!(),
    });

    let value = expression.parse().unwrap();

    return value;
}

fn resolve_expression_02(expression: String) -> isize {
    lazy_static! {
        static ref RE_COMPUTATIONS: [(Regex, fn(isize, isize, &str) -> isize); 2] = [
            (
                Regex::new(r"(?P<v1>\d+) (?P<op>\+) (?P<v2>\d+)").unwrap(),
                |v1, v2, _op| v1 + v2
            ),
            (
                Regex::new(r"(?P<v1>\d+) (?P<op>\*) (?P<v2>\d+)").unwrap(),
                |v1, v2, _op| v1 * v2
            )
        ];
    }

    let mut expression = resolve_parentheses(expression, resolve_expression_02);

    for (re, operator) in RE_COMPUTATIONS.iter() {
        expression = resolve_operations(expression, re, *operator);
    }

    let value = expression.parse().unwrap();

    return value;
}

fn resolve_parentheses(expression: String, exp_resolver: fn(String) -> isize) -> String {
    lazy_static! {
        static ref RE_PARENTHESES: Regex = Regex::new(r"\(([^\(]*?)\)").unwrap();
    }
    let mut expression = expression;
    while RE_PARENTHESES.is_match(expression.as_str()) {
        expression = RE_PARENTHESES
            .replace_all(expression.as_str(), |caps: &Captures| {
                exp_resolver(caps[1].to_string()).to_string()
            })
            .into_owned()
            .clone();
    }
    return expression;
}

fn resolve_operations(
    expression: String,
    re: &Regex,
    operator: fn(isize, isize, &str) -> isize,
) -> String {
    let mut expression = expression;
    while re.is_match(expression.as_str()) {
        expression = re
            .replace(expression.as_str(), |caps: &Captures| {
                let v1: isize = caps["v1"].parse().unwrap();
                let v2: isize = caps["v2"].parse().unwrap();
                let total = operator(v1, v2, &caps["op"]);
                return total.to_string();
            })
            .into_owned()
            .clone();
    }
    return expression;
}

fn load_expressions() -> Vec<String> {
    return read_lines("./inputs/18")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
}

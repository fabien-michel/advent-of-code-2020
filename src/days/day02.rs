use crate::utils::print_day_banner;
use crate::utils::read_lines;
use regex::Regex;

pub fn day02_01() {
    print_day_banner(2, 1);
    let password_infos = load_password_infos();

    let mut valid_passwords_count: u16 = 0;
    // println!("{:?}", password_infos)
    for password_info in password_infos {
        let mut char_count: u8 = 0;
        for password_char in password_info.password.chars() {
            if password_char == password_info.letter {
                char_count += 1;
            }
        }
        if char_count >= password_info.min && char_count <= password_info.max {
            valid_passwords_count += 1;
        }
    }

    println!("Valid passwords: {}", valid_passwords_count)
}

pub fn day02_02() {
    print_day_banner(2, 2);
    let password_infos = load_password_infos();
    let mut valid_passwords_count: u32 = 0;

    for password_info in password_infos {
        let password_chars: Vec<char> = password_info.password.chars().collect();
        // if password_info.max as usize > password_chars.len() ||  password_info.min as usize > password_chars.len() {
        //     continue;
        // }
        let min_char = password_chars[password_info.min as usize - 1];
        let max_char = password_chars[password_info.max as usize - 1];
        let min_char_equal = min_char == password_info.letter;
        let max_char_equal = max_char == password_info.letter;
        if (min_char_equal && !max_char_equal) || (max_char_equal && !min_char_equal) {
            valid_passwords_count += 1;
        }
    }
    println!("Valid passwords: {}", valid_passwords_count)
}

fn load_password_infos() -> Vec<PasswordInfo> {
    return read_lines("./inputs/02")
        .unwrap()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();
}

#[derive(Debug)]
struct PasswordInfo {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

fn parse_line(line: String) -> PasswordInfo {
    // println!("{}", line);
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let caps = re.captures(line.as_str()).unwrap();
    // println!("{:?}", caps);
    return PasswordInfo {
        min: caps[1].parse::<u8>().unwrap(),
        max: caps[2].parse::<u8>().unwrap(),
        letter: caps[3].chars().next().unwrap(),
        password: caps[4].to_string(),
    };

    // let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
}

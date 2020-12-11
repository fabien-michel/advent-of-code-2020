use crate::utils::print_day_banner;
use crate::utils::read_file;
use regex::Regex;

#[derive(Debug)]
struct PassportInfo {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

pub fn day04_01() {
    print_day_banner(4, 1);
    let passport_infos = load_passports();
    let valid_passports: Vec<&PassportInfo> = passport_infos
        .iter()
        .filter(are_required_fields_present)
        .collect();
    let valid_passports_count = valid_passports.len();

    println!("Valid passports: {:?}", valid_passports_count)
}

fn are_required_fields_present(passport_info: &&PassportInfo) -> bool {
    let is_valid = !(passport_info.byr.is_none()
        || passport_info.iyr.is_none()
        || passport_info.eyr.is_none()
        || passport_info.hgt.is_none()
        || passport_info.hcl.is_none()
        || passport_info.ecl.is_none()
        || passport_info.pid.is_none());
    return is_valid;
}

pub fn day04_02() {
    print_day_banner(4, 2);
    let passport_infos = load_passports();
    let valid_passports: Vec<&PassportInfo> = passport_infos
        .iter()
        .filter(are_required_fields_present)
        .filter(is_valid_passport)
        .collect();
    let valid_passports_count = valid_passports.len();

    println!("Valid passports: {:?}", valid_passports_count)
}

fn is_valid_passport(passport_info: &&PassportInfo) -> bool {
    if !(1920..=2002).contains(&passport_info.byr.unwrap()) {
        return false;
    }
    if !(2010..=2020).contains(&passport_info.iyr.unwrap()) {
        return false;
    }
    if !(2020..=2030).contains(&passport_info.eyr.unwrap()) {
        return false;
    }
    let hgt = passport_info.hgt.as_ref().unwrap();
    let hgt_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let hgt_caps = hgt_re.captures(hgt.as_str());
    if hgt_caps.is_none() {
        return false;
    }
    let hgt_caps = hgt_caps.unwrap();
    let hgt_value = hgt_caps[1].parse::<u8>().unwrap();
    let hgt_unit = &hgt_caps[2];
    match hgt_unit {
        "cm" => {
            if !(150..=193).contains(&hgt_value) {
                return false;
            }
        }
        "in" => {
            if !(59..=76).contains(&hgt_value) {
                return false;
            }
        }
        _ => return false,
    };
    let hcl = passport_info.hcl.as_ref().unwrap();
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    if !hcl_re.is_match(hcl.as_str()) {
        return false;
    }
    let ecl = passport_info.ecl.as_ref().unwrap();
    let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    if !ecl_re.is_match(ecl.as_str()) {
        return false;
    }
    let pid = passport_info.pid.as_ref().unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    if !pid_re.is_match(pid.as_str()) {
        return false;
    }

    return true;
}

fn load_passports() -> Vec<PassportInfo> {
    let file = read_file("./inputs/04");
    let tmp = file.as_str().replace("\n", " ");
    let tmp = tmp.replace("  ", "\n");
    let passports_strings: Vec<&str> = tmp.split('\n').collect();

    return passports_strings
        .iter()
        .map(parse_passport)
        .collect::<Vec<PassportInfo>>();
}

fn parse_passport(passport_str: &&str) -> PassportInfo {
    let mut passport_info = PassportInfo {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None,
    };
    let passport_details = passport_str.split(" ");
    for passport_detail in passport_details {
        let passport_detail_data: Vec<&str> = passport_detail.split(':').collect();
        if passport_detail_data.len() < 2 {
            continue;
        }
        let passport_detail_name: &str = passport_detail_data[0];
        let passport_detail_value: &str = passport_detail_data[1];
        match passport_detail_name {
            "byr" => passport_info.byr = Some(passport_detail_value.parse::<u16>().unwrap()),
            "iyr" => passport_info.iyr = Some(passport_detail_value.parse::<u16>().unwrap()),
            "eyr" => passport_info.eyr = Some(passport_detail_value.parse::<u16>().unwrap()),
            "hgt" => passport_info.hgt = Some(passport_detail_value.to_string()),
            "hcl" => passport_info.hcl = Some(passport_detail_value.to_string()),
            "ecl" => passport_info.ecl = Some(passport_detail_value.to_string()),
            "pid" => passport_info.pid = Some(passport_detail_value.to_string()),
            "cid" => passport_info.cid = Some(passport_detail_value.to_string()),
            _ => {}
        }
    }
    return passport_info;
}

use crate::utils::print_day_banner;
use crate::utils::read_lines;

pub fn day13_01() {
    print_day_banner(13, 1);
    let (est_time, bus_ids) = load_infos();
    let bus_ids: Vec<isize> = bus_ids.iter().filter_map(|bus_id| *bus_id).collect();
    let rounds_needed: Vec<(isize, isize)> = bus_ids
        .iter()
        .map(|bus_id| (*bus_id, est_time / *bus_id + 1))
        .collect();
    let arrival_times: Vec<(isize, isize)> = rounds_needed
        .iter()
        .map(|(bus_id, rounds_needed)| (*bus_id, rounds_needed * bus_id))
        .collect();

    let mut min_arrival_time = arrival_times[0];
    for arrival_time in &arrival_times {
        if arrival_time.1 < min_arrival_time.1 {
            min_arrival_time = *arrival_time
        }
    }

    let waiting_time = min_arrival_time.1 - est_time;

    println!("{:?}", min_arrival_time.0 * waiting_time);
}

pub fn day13_02() {
    print_day_banner(13, 2);

    let (_, bus_ids) = load_infos();
    let r_ni: Vec<(isize, isize)> = bus_ids
        .iter()
        .enumerate()
        .filter_map(|(index, bus_id)| {
            if bus_id.is_some() {
                Some((bus_id.unwrap() - index as isize, bus_id.unwrap()))
            } else {
                None
            }
        })
        .collect();
    
    let n: isize = r_ni.iter().fold(1, |acc, i| acc * i.1);
    let r_ni_nni: Vec<(isize, isize, isize)> =
        r_ni.iter().map(|(r, ni)| (*r, *ni, n / *ni)).collect();
    let r_ni_nni_invmod: Vec<(isize, isize, isize, isize)> = r_ni_nni
        .iter()
        .map(|(r, ni, nni)| (*r, *ni, *nni, mod_inv(*nni, *ni)))
        .collect();
    let prods: Vec<isize> = r_ni_nni_invmod
        .iter()
        .map(|(r, _, nni, invmod)| r * nni * invmod)
        .collect();
    let sum: isize = prods.iter().sum();
    let mod_sum: isize = sum % n;

    println!("Time: {:?}", mod_sum);
}

fn mod_inv(a: isize, module: isize) -> isize {
    let mut mn = (module, a);
    let mut xy = (0, 1);
    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

fn load_infos() -> (isize, Vec<Option<isize>>) {
    let lines: Vec<String> = read_lines("./inputs/13")
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    let est_time = lines[0].parse::<isize>().unwrap();
    let bus_ids = lines[1]
        .split(',')
        .map(|bus_id| match bus_id.parse::<isize>() {
            Ok(bus_id) => Some(bus_id),
            Err(_) => None,
        })
        .collect();
    return (est_time, bus_ids);
}

use itertools::Itertools;
use std::env;

mod days;

// use crate::days::day1;
// use crate::days::day2;
// use crate::days::day3;
// use crate::days::day4;
// use crate::days::day5;
// use crate::days::day6;
use crate::days::day7;

fn main() {
    let args = env::args().collect_vec();

    let fname = args.get(1).expect("No filename provided.");
    // let res = day1::max_calories(fname, 3);
    // let res = day2::rps_2(fname);
    // let res = day3::elf_groups(fname);
    // let res = day4::camp_cleanup_overlap(fname);
    // let res = day5::crate_mover_9001(fname);
    // let res = day6::read_comm_message(fname);
    let res = day7::sum_file_system(fname);

    if let Ok(out) = res {
        println!("{:#?}", out)
    } else {
        panic!("{}", res.unwrap_err())
    }
}

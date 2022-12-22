use std::env;
use itertools::Itertools;

mod days;

// use crate::days::day1;
use crate::days::day2;

fn main() {
    let args = env::args().collect_vec();

    let fname = args.get(1).expect("No filename provided.");
    // let res = day1::max_calories(fname, 3);
    let res = day2::rps(fname);
    if let Ok(out) = res {
        println!("{out}")
    } else {
        panic!("{}", res.unwrap_err())
    }
}

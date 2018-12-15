#![no_std]

extern crate watchdog_ralloc as ralloc;

fn count_eq(vec: &Vec<i64>, num: i64) -> i64 {
    let mut counter = 0;
    for i in vec {
        if *i == num {
            counter += 1;
        }
    }
    return counter;
}

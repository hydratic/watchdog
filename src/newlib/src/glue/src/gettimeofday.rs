#![no_std]

extern crate watchdog_raw as raw;
extern crate watchdog_ralloc as ralloc;

pub struct _TimeEval {

}

pub struct _Timezone {
    zone: &str,
    setback: i8,
}

impl __TimeEval for _TimeEval {
    type TimeEval = _TimeEval;
}

impl __Timezone for _Timezone {
    type Timezone = _Timezone;
}

pub fn gettimeofday(p: TimeEval, z: Timezone) -> i16 {
    let time = read_rtc("time_only");
    return time;
}

use cstr::cstr;
use likwid;
use std::error::Error;
use std::result::Result;

#[test]
fn run_marker() -> Result<(), Box<dyn Error>> {
    let _likwid = likwid::Likwid::init();

    likwid::register_region(cstr!("test"))?;
    likwid::marker_start_region(cstr!("test"))?;

    let mut counter = 0;
    for i in 0..10000 {
        counter += i;
    }

    likwid::marker_stop_region(cstr!("test"))?;

    println!("{}", counter);

    Ok(())
}

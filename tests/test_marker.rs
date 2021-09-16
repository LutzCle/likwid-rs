use likwid;
use std::error::Error;
use std::result::Result;

#[test]
fn run_marker() -> Result<(), Box<dyn Error>> {
    likwid::init();
    likwid::register_region("test")?;
    likwid::marker_start_region("test")?;

    let mut counter = 0;
    for i in 0..10000 {
        counter += i;
    }

    likwid::marker_stop_region("test")?;
    likwid::close();

    println!("{}", counter);

    Ok(())
}

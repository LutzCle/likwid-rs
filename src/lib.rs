pub mod error;
mod likwid;

use crate::error::Result;
use std::ffi::CString;
use std::io::Error as IoError;
use std::os::raw::c_int;

pub fn init() {
    unsafe {
        likwid::likwid_markerInit();
    }
}

pub fn thread_init() {
    unsafe {
        likwid::likwid_markerThreadInit();
    }
}

pub fn next_group() {
    unsafe {
        likwid::likwid_markerNextGroup();
    }
}

pub fn close() {
    unsafe {
        likwid::likwid_markerClose();
    }
}

pub fn register_region(region_tag: &str) -> Result<()> {
    let c_region_tag = CString::new(region_tag)?;

    let ret = unsafe { likwid::likwid_markerRegisterRegion(c_region_tag.as_ptr()) };

    if ret != 0 {
        Err(IoError::from_raw_os_error(-ret))?;
    }

    Ok(())
}

pub fn marker_start_region(region_tag: &str) -> Result<()> {
    let c_region_tag = CString::new(region_tag)?;

    let ret = unsafe { likwid::likwid_markerStartRegion(c_region_tag.as_ptr()) };

    if ret != 0 {
        Err(IoError::from_raw_os_error(-ret))?;
    }

    Ok(())
}

pub fn marker_stop_region(region_tag: &str) -> Result<()> {
    let c_region_tag = CString::new(region_tag)?;

    let ret = unsafe { likwid::likwid_markerStopRegion(c_region_tag.as_ptr()) };

    if ret != 0 {
        Err(IoError::from_raw_os_error(-ret))?;
    }

    Ok(())
}

pub fn marker_reset_region(region_tag: &str) -> Result<()> {
    let c_region_tag = CString::new(region_tag)?;

    let ret = unsafe { likwid::likwid_markerResetRegion(c_region_tag.as_ptr()) };

    if ret != 0 {
        Err(IoError::from_raw_os_error(-ret))?;
    }

    Ok(())
}

pub fn marker_get_region(region_tag: &str, events: &mut Vec<f64>) -> Result<(f64, i32)> {
    let c_region_tag = CString::new(region_tag)?;
    let mut time = 0.0;
    let mut count = 0;
    let mut nr_events = events.len() as c_int;

    unsafe {
        likwid::likwid_markerGetRegion(
            c_region_tag.as_ptr(),
            &mut nr_events,
            events.as_mut_ptr(),
            &mut time,
            &mut count,
        );
    };

    assert!(
        nr_events >= 0,
        "Event length returned by Likwid is smaller than 0"
    );
    events.resize(nr_events as usize, 0.0);

    Ok((time, count))
}

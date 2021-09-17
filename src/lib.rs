pub mod error;

#[cfg(feature = "likwid_perfmon")]
mod likwid;

use crate::error::Result;

pub fn init() {
    #[cfg(feature = "likwid_perfmon")]
    unsafe {
        likwid::likwid_markerInit();
    }
}

pub fn thread_init() {
    #[cfg(feature = "likwid_perfmon")]
    unsafe {
        likwid::likwid_markerThreadInit();
    }
}

pub fn next_group() {
    #[cfg(feature = "likwid_perfmon")]
    unsafe {
        likwid::likwid_markerNextGroup();
    }
}

pub fn close() {
    #[cfg(feature = "likwid_perfmon")]
    unsafe {
        likwid::likwid_markerClose();
    }
}

pub fn register_region(_region_tag: &std::ffi::CStr) -> Result<()> {
    #[cfg(feature = "likwid_perfmon")]
    {
        let ret = unsafe { likwid::likwid_markerRegisterRegion(_region_tag.as_ptr()) };

        if ret != 0 {
            Err(std::io::Error::from_raw_os_error(-ret))?;
        }
    }

    Ok(())
}

pub fn marker_start_region(_region_tag: &std::ffi::CStr) -> Result<()> {
    #[cfg(feature = "likwid_perfmon")]
    {
        let ret = unsafe { likwid::likwid_markerStartRegion(_region_tag.as_ptr()) };

        if ret != 0 {
            Err(std::io::Error::from_raw_os_error(-ret))?;
        }
    }

    Ok(())
}

pub fn marker_stop_region(_region_tag: &std::ffi::CStr) -> Result<()> {
    #[cfg(feature = "likwid_perfmon")]
    {
        let ret = unsafe { likwid::likwid_markerStopRegion(_region_tag.as_ptr()) };

        if ret != 0 {
            Err(std::io::Error::from_raw_os_error(-ret))?;
        }
    }

    Ok(())
}

pub fn marker_reset_region(_region_tag: &std::ffi::CStr) -> Result<()> {
    #[cfg(feature = "likwid_perfmon")]
    {
        let ret = unsafe { likwid::likwid_markerResetRegion(_region_tag.as_ptr()) };

        if ret != 0 {
            Err(std::io::Error::from_raw_os_error(-ret))?;
        }
    }

    Ok(())
}

pub fn marker_get_region(
    _region_tag: &std::ffi::CStr,
    _events: &mut Vec<f64>,
) -> Result<(f64, i32)> {
    #[cfg(feature = "likwid_perfmon")]
    {
        let mut time = 0.0;
        let mut count = 0;
        let mut nr_events = _events.len() as std::os::raw::c_int;

        unsafe {
            likwid::likwid_markerGetRegion(
                _region_tag.as_ptr(),
                &mut nr_events,
                _events.as_mut_ptr(),
                &mut time,
                &mut count,
            );
        };

        assert!(
            nr_events >= 0,
            "Event length returned by Likwid is smaller than 0"
        );
        _events.resize(nr_events as usize, 0.0);

        Ok((time, count))
    }

    #[cfg(not(feature = "likwid_perfmon"))]
    Ok((0.0, 0))
}

#![no_std]
#![feature(c_variadic)]

extern crate alloc;

pub mod kernel_api;
#[cfg(feature = "included_libc")]
mod libc_impl;
pub mod types;

pub use types::*;

pub fn init(rsdp: PhysAddr, log_level: LogLevel, no_acpi_mode: bool) -> Result<(), Status> {
    let rt_params = uacpi_sys::uacpi_params {
        log_level: log_level.0,
        flags: 0,
    };

    let mut params = uacpi_sys::uacpi_init_params {
        rt_params,
        rsdp: rsdp.0,
        no_acpi_mode,
    };

    let status: Status = unsafe { uacpi_sys::uacpi_initialize(&mut params).into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status),
    }
}

pub fn namespace_load() -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_namespace_load().into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status),
    }
}

pub fn namespace_initialize() -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_namespace_initialize().into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status),
    }
}

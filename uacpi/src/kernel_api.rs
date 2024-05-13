use crate::types::{
    FirmwareRequest, Handle, IOAddr, LogLevel, PCIAddress, PhysAddr, Status, WorkType,
};
use alloc::{
    alloc::{alloc, dealloc},
    boxed::Box,
    string::String,
    sync::Arc,
};
use core::{
    alloc::Layout,
    ffi::{c_char, c_void},
};
use log::{debug, error, info, trace, warn};
use printf_compat::{format, output};

pub trait KernelApi {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        dealloc(ptr, layout)
    }

    #[cfg(feature = "logging")]
    fn log(&self, log_level: LogLevel, string: &str) {
        if log_level == LogLevel::TRACE {
            trace!("{string}");
        } else if log_level == LogLevel::DEBUG {
            debug!("{string}");
        } else if log_level == LogLevel::INFO {
            info!("{string}");
        } else if log_level == LogLevel::WARN {
            warn!("{string}");
        } else if log_level == LogLevel::ERROR {
            error!("{string}");
        }
    }

    #[cfg(not(feature = "logging"))]
    fn log(&self, log_level: LogLevel, string: &str);

    fn stall(&self, usec: u8);
    fn sleep(&self, msec: u8);
    fn get_ticks(&self) -> u64;

    unsafe fn map(&self, phys: PhysAddr, len: usize) -> *mut c_void;
    unsafe fn unmap(&self, addr: *mut c_void, len: usize);
    unsafe fn raw_memory_read(&self, phys: PhysAddr, byte_width: u8) -> Result<u64, Status>;
    unsafe fn raw_memory_write(
        &self,
        phys: PhysAddr,
        byte_width: u8,
        val: u64,
    ) -> Result<(), Status>;

    fn create_event(&self) -> Handle;
    fn destroy_event(&self, event: Handle);
    fn wait_for_event(&self, event: Handle, timeout: u16) -> bool;
    fn signal_event(&self, event: Handle);
    fn reset_event(&self, event: Handle);

    fn create_mutex(&self) -> Handle;
    fn destroy_mutex(&self, mutex: Handle);
    fn acquire_mutex(&self, mutex: Handle, timeout: u16) -> bool;
    fn release_mutex(&self, mutex: Handle);

    unsafe fn pci_read(
        &self,
        address: PCIAddress,
        offset: usize,
        byte_width: u8,
    ) -> Result<u64, Status>;
    unsafe fn pci_write(
        &self,
        address: PCIAddress,
        offset: usize,
        byte_width: u8,
        val: u64,
    ) -> Result<(), Status>;

    unsafe fn io_map(&self, base: IOAddr, len: usize) -> Result<Handle, Status>;
    unsafe fn io_unmap(&self, handle: Handle);
    unsafe fn io_read(&self, handle: Handle, offset: usize, byte_width: u8) -> Result<u64, Status>;
    unsafe fn io_write(
        &self,
        handle: Handle,
        offset: usize,
        byte_width: u8,
        val: u64,
    ) -> Result<(), Status>;
    unsafe fn raw_io_read(&self, addr: IOAddr, byte_width: u8) -> Result<u64, Status>;
    unsafe fn raw_io_write(&self, addr: IOAddr, byte_width: u8, val: u64) -> Result<(), Status>;

    fn firmware_request(&self, req: FirmwareRequest) -> Result<(), Status>;

    fn schedule_work(&self, work_type: WorkType, handler: Box<dyn Fn()>) -> Result<(), Status>;

    fn install_interrupt_handler(&self, irq: u32, handler: Box<dyn Fn()>)
        -> Result<Handle, Status>;
    fn uninstall_interrupt_handler(&self, handle: Handle) -> Result<(), Status>;
}

static mut KERNEL_API: Option<Arc<dyn KernelApi>> = None;

pub fn set_kernel_api(api: Arc<dyn KernelApi>) {
    unsafe { KERNEL_API = Some(api) }
}

fn get_kernel_api() -> Arc<dyn KernelApi> {
    unsafe { KERNEL_API.as_ref().expect("No kernel api set").clone() }
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_get_ticks() -> u64 {
    get_kernel_api().get_ticks()
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_stall(usec: u8) {
    get_kernel_api().stall(usec)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_sleep(msec: u8) {
    get_kernel_api().sleep(msec)
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_log(
    log_level: uacpi_sys::uacpi_log_level,
    fmt: *const c_char,
    mut va: ...
) {
    let mut s = String::new();
    format(fmt, va.as_va_list(), output::fmt_write(&mut s));

    get_kernel_api().log(LogLevel(log_level), &s[0..s.len() - 1])
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_map(
    phys: uacpi_sys::uacpi_phys_addr,
    len: usize,
) -> *mut c_void {
    get_kernel_api().map(PhysAddr(phys), len)
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_unmap(addr: *mut c_void, len: usize) {
    get_kernel_api().unmap(addr, len)
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_raw_memory_read(
    phys: uacpi_sys::uacpi_phys_addr,
    byte_width: u8,
    val: *mut u64,
) -> Status {
    match get_kernel_api().raw_memory_read(PhysAddr(phys), byte_width) {
        Ok(ret) => {
            *val = ret;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_raw_memory_write(
    phys: uacpi_sys::uacpi_phys_addr,
    byte_width: u8,
    val: u64,
) -> Status {
    match get_kernel_api().raw_memory_write(PhysAddr(phys), byte_width, val) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_alloc(size: usize) -> *mut c_void {
    get_kernel_api()
        .alloc(Layout::from_size_align(size, 8).unwrap())
        .cast()
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_calloc(count: usize, size: usize) -> *mut c_void {
    get_kernel_api()
        .alloc(Layout::from_size_align(count * size, 8).unwrap())
        .cast()
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_free(ptr: *mut c_void, size: usize) {
    get_kernel_api().dealloc(ptr.cast(), Layout::from_size_align(size, 8).unwrap())
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_create_event() -> Handle {
    get_kernel_api().create_event()
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_free_event(event: Handle) {
    get_kernel_api().destroy_event(event)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_wait_for_event(event: Handle, timeout: u16) -> bool {
    get_kernel_api().wait_for_event(event, timeout)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_signal_event(event: Handle) {
    get_kernel_api().signal_event(event)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_reset_event(event: Handle) {
    get_kernel_api().reset_event(event)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_create_mutex() -> Handle {
    get_kernel_api().create_mutex()
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_free_mutex(mutex: Handle) {
    get_kernel_api().destroy_mutex(mutex)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_acquire_mutex(mutex: Handle, timeout: u16) -> bool {
    get_kernel_api().acquire_mutex(mutex, timeout)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_release_mutex(mutex: Handle) {
    get_kernel_api().release_mutex(mutex)
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_pci_read(
    address: *const uacpi_sys::uacpi_pci_address,
    offset: usize,
    byte_width: u8,
    val: *mut u64,
) -> Status {
    match get_kernel_api().pci_read(PCIAddress(*address), offset, byte_width) {
        Ok(ret) => {
            *val = ret;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_pci_write(
    address: *const uacpi_sys::uacpi_pci_address,
    offset: usize,
    byte_width: u8,
    val: u64,
) -> Status {
    match get_kernel_api().pci_write(PCIAddress(*address), offset, byte_width, val) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_io_map(
    base: uacpi_sys::uacpi_io_addr,
    len: usize,
    out_handle: *mut uacpi_sys::uacpi_handle,
) -> Status {
    match get_kernel_api().io_map(IOAddr(base), len) {
        Ok(ret) => {
            *out_handle = ret.0;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_io_unmap(handle: uacpi_sys::uacpi_handle) {
    get_kernel_api().io_unmap(Handle(handle))
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_io_read(
    handle: uacpi_sys::uacpi_handle,
    offset: usize,
    byte_width: u8,
    val: *mut u64,
) -> Status {
    match get_kernel_api().io_read(Handle(handle), offset, byte_width) {
        Ok(ret) => {
            *val = ret;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_io_write(
    handle: uacpi_sys::uacpi_handle,
    offset: usize,
    byte_width: u8,
    val: u64,
) -> Status {
    match get_kernel_api().io_write(Handle(handle), offset, byte_width, val) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_raw_io_read(
    addr: uacpi_sys::uacpi_io_addr,
    byte_width: u8,
    val: *mut u64,
) -> Status {
    match get_kernel_api().raw_io_read(IOAddr(addr), byte_width) {
        Ok(ret) => {
            *val = ret;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_raw_io_write(
    addr: uacpi_sys::uacpi_io_addr,
    byte_width: u8,
    val: u64,
) -> Status {
    match get_kernel_api().raw_io_write(IOAddr(addr), byte_width, val) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_handle_firmware_request(
    req: *const uacpi_sys::uacpi_firmware_request,
) -> Status {
    match get_kernel_api().firmware_request(req.read().into()) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_schedule_work(
    work_type: WorkType,
    handler: extern "C" fn(Handle),
    ctx: Handle,
) -> Status {
    match get_kernel_api().schedule_work(work_type, Box::new(move || handler(ctx))) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_install_interrupt_handler(
    irq: u32,
    handler: extern "C" fn(Handle),
    ctx: Handle,
    out_irq_handle: *mut Handle,
) -> Status {
    match get_kernel_api().install_interrupt_handler(irq, Box::new(move || handler(ctx))) {
        Ok(val) => {
            *out_irq_handle = val;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_uninstall_interrupt_handler(
    _handler: extern "C" fn(Handle),
    irq_handle: Handle,
) -> Status {
    match get_kernel_api().uninstall_interrupt_handler(irq_handle) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

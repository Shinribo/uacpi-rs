#![no_std]

type PhysAddr = u64;

#[repr(C)]
#[derive(Debug)]
pub enum Status {
    Ok = 0,
    MappingFailed = 1,
    OutOfMemory = 2,
    BadChecksum = 3,
    InvalidSignature = 4,
    InvalidTableLength = 5,
    NotFound = 6,
    InvalidArgument = 7,
    Unimplemented = 8,
    AlreadyExists = 9,
    InternalError = 10,
    TypeMismatch = 11,
    InitLevelMismatch = 12,
    NamespaceNodeDangling = 13,
    NoHandler = 14,
    NoResourceEndTag = 15,
    CompiledOut = 16,
    HardwareTimeout = 17,

    // All errors that have bytecode-related origin should go here
    AmlUndefinedReference = 0xEFF0000,
    AmlInvalidNamesring = 0xEFF0001,
    AmlObjectAlreadyExists = 0xEFF0002,
    AmlInvalidOpcode = 0xEFF0003,
    AmlIncompatibleObjectType = 0xEFF0004,
    AmlBadEncoding = 0xEFF0005,
    AmlOutOfBoundsIndex = 0xEFF0006,
    AmlSyncLevelTooHigh = 0xEFF0007,
    AmlInvalidResource = 0xEFF0008,
    AmlLoopTimeout = 0xEFF0009,
}

#[repr(C)]
#[derive(Debug)]
pub enum LogLevel {
    Debug = 4,
    Trace = 3,
    Info = 2,
    Warn = 1,
    Error = 0,
}

#[repr(C)]
pub struct Params {
    log_level: LogLevel,
    flags: u64,
}

#[repr(C)]
pub struct InitParams {
    rsdp: PhysAddr,
    rt_params: Params,
    no_acpi_mode: bool,
}

extern "C" {
    pub fn uacpi_initialize(params: *mut InitParams) -> Status;
}

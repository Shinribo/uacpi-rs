use core::fmt::Debug;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogLevel(pub(crate) uacpi_sys::uacpi_log_level);

impl LogLevel {
    pub const DEBUG: LogLevel = LogLevel(uacpi_sys::uacpi_log_level_UACPI_LOG_DEBUG);
    pub const TRACE: LogLevel = LogLevel(uacpi_sys::uacpi_log_level_UACPI_LOG_TRACE);
    pub const INFO: LogLevel = LogLevel(uacpi_sys::uacpi_log_level_UACPI_LOG_INFO);
    pub const WARN: LogLevel = LogLevel(uacpi_sys::uacpi_log_level_UACPI_LOG_WARN);
    pub const ERROR: LogLevel = LogLevel(uacpi_sys::uacpi_log_level_UACPI_LOG_ERROR);
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Handle(pub(crate) uacpi_sys::uacpi_handle);

impl Handle {
    pub fn new(handle: u64) -> Handle {
        Handle(handle as _)
    }

    pub fn as_u64(self) -> u64 {
        self.0 as _
    }
}

impl Debug for Handle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.as_u64())
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PhysAddr(pub(crate) uacpi_sys::uacpi_phys_addr);

impl PhysAddr {
    pub fn new(phys_addr: u64) -> PhysAddr {
        PhysAddr(phys_addr as _)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct IOAddr(pub(crate) uacpi_sys::uacpi_io_addr);

impl IOAddr {
    pub fn new(phys_addr: u64) -> IOAddr {
        IOAddr(phys_addr as _)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PCIAddress(pub(crate) uacpi_sys::uacpi_pci_address);

impl PCIAddress {
    pub fn segment(&self) -> u16 {
        self.0.segment
    }

    pub fn bus(&self) -> u8 {
        self.0.bus
    }

    pub fn device(&self) -> u8 {
        self.0.device
    }

    pub fn function(&self) -> u8 {
        self.0.function
    }
}

impl Debug for PCIAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:x}:{:x}:{:x}:{:x}",
            self.segment(),
            self.bus(),
            self.device(),
            self.function()
        )
    }
}

#[must_use]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Ok = uacpi_sys::uacpi_status_UACPI_STATUS_OK,
    MappingFailed = uacpi_sys::uacpi_status_UACPI_STATUS_MAPPING_FAILED,
    OutOfMemory = uacpi_sys::uacpi_status_UACPI_STATUS_OUT_OF_MEMORY,
    BadChecksum = uacpi_sys::uacpi_status_UACPI_STATUS_BAD_CHECKSUM,
    InvalidSignature = uacpi_sys::uacpi_status_UACPI_STATUS_INVALID_SIGNATURE,
    InvalidTableLenght = uacpi_sys::uacpi_status_UACPI_STATUS_INVALID_TABLE_LENGTH,
    NotFound = uacpi_sys::uacpi_status_UACPI_STATUS_NOT_FOUND,
    InvalidArgument = uacpi_sys::uacpi_status_UACPI_STATUS_INVALID_ARGUMENT,
    Unimplemented = uacpi_sys::uacpi_status_UACPI_STATUS_UNIMPLEMENTED,
    AlreadyExists = uacpi_sys::uacpi_status_UACPI_STATUS_ALREADY_EXISTS,
    InternalError = uacpi_sys::uacpi_status_UACPI_STATUS_INTERNAL_ERROR,
    TypeMismatch = uacpi_sys::uacpi_status_UACPI_STATUS_TYPE_MISMATCH,
    InitLevelMismatch = uacpi_sys::uacpi_status_UACPI_STATUS_INIT_LEVEL_MISMATCH,
    NamespaceNodeDangling = uacpi_sys::uacpi_status_UACPI_STATUS_NAMESPACE_NODE_DANGLING,
    NoHandler = uacpi_sys::uacpi_status_UACPI_STATUS_NO_HANDLER,
    NoResourceEndTag = uacpi_sys::uacpi_status_UACPI_STATUS_NO_RESOURCE_END_TAG,
    CompiledOut = uacpi_sys::uacpi_status_UACPI_STATUS_COMPILED_OUT,
    HardwareTimeout = uacpi_sys::uacpi_status_UACPI_STATUS_HARDWARE_TIMEOUT,
    AmlUndefinedReference = uacpi_sys::uacpi_status_UACPI_STATUS_AML_UNDEFINED_REFERENCE,
    AmlInvalidNamestring = uacpi_sys::uacpi_status_UACPI_STATUS_AML_INVALID_NAMESTRING,
    AmlObjectAlreadyExists = uacpi_sys::uacpi_status_UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS,
    AmlInvalidOpcode = uacpi_sys::uacpi_status_UACPI_STATUS_AML_INVALID_OPCODE,
    AmlIncompatibleObjectType = uacpi_sys::uacpi_status_UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE,
    AmlBadEncoding = uacpi_sys::uacpi_status_UACPI_STATUS_AML_BAD_ENCODING,
    AmlOutOfBoundsIndex = uacpi_sys::uacpi_status_UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX,
    AmlSyncLevelTooHigh = uacpi_sys::uacpi_status_UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH,
    AmlInvalidResource = uacpi_sys::uacpi_status_UACPI_STATUS_AML_INVALID_RESOURCE,
    AmlLoopTimeout = uacpi_sys::uacpi_status_UACPI_STATUS_AML_LOOP_TIMEOUT,
}

impl From<uacpi_sys::uacpi_status> for Status {
    fn from(status: uacpi_sys::uacpi_status) -> Self {
        match status {
            uacpi_sys::uacpi_status_UACPI_STATUS_OK => Status::Ok,
            uacpi_sys::uacpi_status_UACPI_STATUS_MAPPING_FAILED => Status::MappingFailed,
            uacpi_sys::uacpi_status_UACPI_STATUS_OUT_OF_MEMORY => Status::OutOfMemory,
            uacpi_sys::uacpi_status_UACPI_STATUS_BAD_CHECKSUM => Status::BadChecksum,
            uacpi_sys::uacpi_status_UACPI_STATUS_INVALID_SIGNATURE => Status::InvalidSignature,
            uacpi_sys::uacpi_status_UACPI_STATUS_INVALID_TABLE_LENGTH => Status::InvalidTableLenght,
            uacpi_sys::uacpi_status_UACPI_STATUS_NOT_FOUND => Status::NotFound,
            uacpi_sys::uacpi_status_UACPI_STATUS_INVALID_ARGUMENT => Status::InvalidArgument,
            uacpi_sys::uacpi_status_UACPI_STATUS_UNIMPLEMENTED => Status::Unimplemented,
            uacpi_sys::uacpi_status_UACPI_STATUS_ALREADY_EXISTS => Status::AlreadyExists,
            uacpi_sys::uacpi_status_UACPI_STATUS_INTERNAL_ERROR => Status::InternalError,
            uacpi_sys::uacpi_status_UACPI_STATUS_TYPE_MISMATCH => Status::TypeMismatch,
            uacpi_sys::uacpi_status_UACPI_STATUS_INIT_LEVEL_MISMATCH => Status::InitLevelMismatch,
            uacpi_sys::uacpi_status_UACPI_STATUS_NAMESPACE_NODE_DANGLING => {
                Status::NamespaceNodeDangling
            }
            uacpi_sys::uacpi_status_UACPI_STATUS_NO_HANDLER => Status::NoHandler,
            uacpi_sys::uacpi_status_UACPI_STATUS_NO_RESOURCE_END_TAG => Status::NoResourceEndTag,
            uacpi_sys::uacpi_status_UACPI_STATUS_COMPILED_OUT => Status::CompiledOut,
            uacpi_sys::uacpi_status_UACPI_STATUS_HARDWARE_TIMEOUT => Status::HardwareTimeout,
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_UNDEFINED_REFERENCE => {
                Status::AmlUndefinedReference
            }
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_INVALID_NAMESTRING => {
                Status::AmlInvalidNamestring
            }
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS => {
                Status::AmlObjectAlreadyExists
            }
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_INVALID_OPCODE => Status::AmlInvalidOpcode,
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE => {
                Status::AmlIncompatibleObjectType
            }
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_BAD_ENCODING => Status::AmlBadEncoding,
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX => {
                Status::AmlOutOfBoundsIndex
            }
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH => {
                Status::AmlSyncLevelTooHigh
            }
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_INVALID_RESOURCE => Status::AmlInvalidResource,
            uacpi_sys::uacpi_status_UACPI_STATUS_AML_LOOP_TIMEOUT => Status::AmlLoopTimeout,
            _ => panic!("Unknown uacpi_status value: {:#x}", status),
        }
    }
}

#[derive(Debug)]
pub enum FirmwareRequest {
    Breakpoint { context: Handle },
    Fatal { typ: u8, code: u32, arg: u64 },
}

impl From<uacpi_sys::uacpi_firmware_request> for FirmwareRequest {
    fn from(value: uacpi_sys::uacpi_firmware_request) -> Self {
        match value.type_ as u32 {
            uacpi_sys::uacpi_firmware_request_type_UACPI_FIRMWARE_REQUEST_TYPE_BREAKPOINT => {
                FirmwareRequest::Breakpoint {
                    context: Handle(unsafe { value.__bindgen_anon_1.breakpoint.ctx }),
                }
            }
            uacpi_sys::uacpi_firmware_request_type_UACPI_FIRMWARE_REQUEST_TYPE_FATAL => {
                FirmwareRequest::Fatal {
                    typ: unsafe { value.__bindgen_anon_1.fatal.type_ },
                    code: unsafe { value.__bindgen_anon_1.fatal.code },
                    arg: unsafe { value.__bindgen_anon_1.fatal.arg },
                }
            }
            _ => panic!("Unknown uacpi_firmware_request_type: {:#x}", value.type_),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum WorkType {
    GPEExecution = uacpi_sys::uacpi_work_type_UACPI_WORK_GPE_EXECUTION,
    Notification = uacpi_sys::uacpi_work_type_UACPI_WORK_NOTIFICATION,
}

impl From<uacpi_sys::uacpi_work_type> for WorkType {
    fn from(value: uacpi_sys::uacpi_work_type) -> Self {
        match value {
            uacpi_sys::uacpi_work_type_UACPI_WORK_GPE_EXECUTION => Self::GPEExecution,
            uacpi_sys::uacpi_work_type_UACPI_WORK_NOTIFICATION => Self::Notification,
            _ => panic!("Unknown uacpi_work_type: {:#x}", value),
        }
    }
}

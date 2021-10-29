use winapi::shared::minwindef::*;
use winapi::shared::ntdef::{HRESULT, LONG, PCWSTR};

// #[link(name = "mf")]
// #[link(name = "mfplay")]
// #[link(name = "dxva2")]
#[link(name = "mfplat")]
// #[link(name = "mfreadwrite")]
// #[link(name = "mfuuid")]
extern "system" {
    pub fn MFStartup(Version: ULONG, dwFlags: DWORD) -> HRESULT;

    pub fn MFLockSharedWorkQueue(
        wszClass: PCWSTR,
        BasePriority: LONG,
        pdwTaskID: *mut DWORD,
        pID: *mut DWORD,
    ) -> HRESULT;
}
pub const MF_SDK_VERSION: ULONG = 0x0002;
pub const MF_API_VERSION: ULONG = 0x0070;
pub const MF_VERSION: ULONG = MF_SDK_VERSION << 16 | MF_API_VERSION;
pub const MFSTARTUP_LITE: DWORD = 0x1;

#[repr(C)]
pub struct AUDIOCLIENT_ACTIVATION_PARAMS {
    pub ActivationType: AUDIOCLIENT_ACTIVATION_TYPE,
    pub ProcessLoopbackParams: AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS,
}

#[repr(C)]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: DWORD,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}

#[repr(C)]
pub enum PROCESS_LOOPBACK_MODE {
    PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE,
    PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE,
}

#[repr(C)]
pub enum AUDIOCLIENT_ACTIVATION_TYPE {
    AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT,
    AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK,
}

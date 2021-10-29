use std::{mem, ptr};
use std::{thread, time::Duration};
use winapi::shared::wtypes::VT_BLOB;
use winapi::um::propidl::*;

use crate::windefs::*;

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
pub struct Loopback {}
impl Loopback {
    pub fn new() -> Self {
        Self {}
    }
    pub fn initialize(&self) {
        unsafe {
            dbg!(MFStartup(MF_VERSION, MFSTARTUP_LITE));

            let task_id = ptr::null_mut();
            let queue_id = ptr::null_mut();
            dbg!(MFLockSharedWorkQueue(
                wide_null("Capture").as_ptr(),
                0,
                task_id,
                queue_id,
            ));
        }
    }
    pub fn activate(&self, pid: u32) {
        let params = AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
            TargetProcessId: pid,
            ProcessLoopbackMode:
                PROCESS_LOOPBACK_MODE::PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE,
        };

        let audio_params = AUDIOCLIENT_ACTIVATION_PARAMS {
            ActivationType:
                AUDIOCLIENT_ACTIVATION_TYPE::AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK,
            ProcessLoopbackParams: params,
        };

        unsafe {
            let activate_params: PROPVARIANT = mem::zeroed();
            activate_params.vt = VT_BLOB;
            activate_params.blob
        }
        //             PROPVARIANT activateParams = {};
        //             activateParams.vt = VT_BLOB;
        //             activateParams.blob.cbSize = sizeof(audioclientActivationParams);
        //             activateParams.blob.pBlobData = (BYTE*)&audioclientActivationParams;

        //             wil::com_ptr_nothrow<IActivateAudioInterfaceAsyncOperation> asyncOp;
        //             RETURN_IF_FAILED(ActivateAudioInterfaceAsync(VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK, __uuidof(IAudioClient), &activateParams, this, &asyncOp));

        //             // Wait for activation completion
        //             m_hActivateCompleted.wait();
    }
    pub fn run(&self, pid: u32, file: &String) {
        self.initialize();
        self.activate(pid);

        thread::sleep(Duration::from_secs(10));
        self.stop();
    }
    pub fn stop(&self) {}
}

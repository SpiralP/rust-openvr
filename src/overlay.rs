use std::{error::Error, ffi::CString, fmt::Display};

use openvr_sys::{EVROverlayError, EVROverlayError_VROverlayError_None, VROverlayHandle_t};

use crate::Overlay;

#[derive(Debug)]
pub struct VROverlayError(pub EVROverlayError);
impl Display for VROverlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EVROverlayError {}", self.0)
    }
}
impl Error for VROverlayError {}
impl From<EVROverlayError> for VROverlayError {
    fn from(err: EVROverlayError) -> Self {
        VROverlayError(err)
    }
}

impl Overlay {
    /** Finds an existing overlay with the specified key. */
    pub fn find(&self, overlay_key: &str) -> Result<VROverlayHandle_t, VROverlayError> {
        let mut overlay_handle: VROverlayHandle_t = unsafe { std::mem::zeroed() };
        let overlay_key = CString::new(overlay_key).unwrap();

        let result =
            unsafe { self.0.FindOverlay.unwrap()(overlay_key.as_ptr() as _, &mut overlay_handle) };
        if result == EVROverlayError_VROverlayError_None {
            Ok(overlay_handle)
        } else {
            Err(result.into())
        }
    }

    /** Creates a new named overlay. All overlays start hidden and with default settings. */
    pub fn create(
        &self,
        overlay_key: &str,
        overlay_name: &str,
    ) -> Result<VROverlayHandle_t, VROverlayError> {
        let mut overlay_handle: VROverlayHandle_t = unsafe { std::mem::zeroed() };
        let overlay_key = CString::new(overlay_key).unwrap();
        let overlay_name = CString::new(overlay_name).unwrap();

        let result = unsafe {
            self.0.CreateOverlay.unwrap()(
                overlay_key.as_ptr() as _,
                overlay_name.as_ptr() as _,
                &mut overlay_handle,
            )
        };
        if result == EVROverlayError_VROverlayError_None {
            Ok(overlay_handle)
        } else {
            Err(result.into())
        }
    }

    /** Destroys the specified overlay. When an application calls VR_Shutdown all overlays created by that app are
     * automatically destroyed. */
    pub fn destroy(&self, overlay_handle: VROverlayHandle_t) -> Result<(), VROverlayError> {
        let result = unsafe { self.0.DestroyOverlay.unwrap()(overlay_handle) };
        if result == EVROverlayError_VROverlayError_None {
            Ok(())
        } else {
            Err(result.into())
        }
    }
}

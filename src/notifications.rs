use std::{error::Error, ffi::CString, fmt::Display};

use openvr_sys::{
    EVRNotificationError, EVRNotificationError_VRNotificationError_OK, EVRNotificationStyle,
    EVRNotificationType, NotificationBitmap_t, VRNotificationId, VROverlayHandle_t,
};

use crate::Notifications;

#[derive(Debug)]
pub struct VRNotificationError(pub EVRNotificationError);
impl Display for VRNotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EVRNotificationError {}", self.0)
    }
}
impl Error for VRNotificationError {}
impl From<EVRNotificationError> for VRNotificationError {
    fn from(err: EVRNotificationError) -> Self {
        VRNotificationError(err)
    }
}

impl Notifications {
    /** Create a notification and enqueue it to be shown to the user.
     * An overlay handle is required to create a notification, as otherwise it would be impossible for a user to act on it.
     * To create a two-line notification, use a line break ('\n') to split the text into two lines.
     * The pImage argument may be NULL, in which case the specified overlay's icon will be used instead. */
    pub fn create(
        &self,
        overlay_handle: VROverlayHandle_t,
        user_value: u64,
        notification_type: EVRNotificationType,
        text: &str,
        style: EVRNotificationStyle,
        image: Option<*mut NotificationBitmap_t>,
    ) -> Result<VRNotificationId, VRNotificationError> {
        let mut id = 0;
        let text = CString::new(text).unwrap();
        let image: *mut NotificationBitmap_t =
            image.unwrap_or(std::ptr::null_mut::<NotificationBitmap_t>());
        let result = unsafe {
            self.0.CreateNotification.unwrap()(
                overlay_handle,
                user_value,
                notification_type,
                text.as_ptr() as _,
                style,
                image,
                &mut id,
            )
        };
        if result == EVRNotificationError_VRNotificationError_OK {
            Ok(id)
        } else {
            Err(result.into())
        }
    }

    /** Destroy a notification, hiding it first if it currently shown to the user. */
    pub fn remove(&self, id: VRNotificationId) -> Result<(), VRNotificationError> {
        let result = unsafe { self.0.RemoveNotification.unwrap()(id) };
        if result == EVRNotificationError_VRNotificationError_OK {
            Ok(())
        } else {
            Err(result.into())
        }
    }
}

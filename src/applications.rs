use std::{
    ffi::{CStr, CString},
    path::Path,
};

use openvr_sys::{
    k_unMaxApplicationKeyLength, EVRApplicationError, EVRApplicationError_VRApplicationError_None,
};

use crate::Applications;

impl Applications {
    /** Adds an application manifest to the list to load when building the list of installed applications.
     * Temporary manifests are not automatically loaded */
    pub fn add_application_manifest(
        &self,
        app_manifest_full_path: &Path,
        temporary: bool,
    ) -> Result<(), EVRApplicationError> {
        let app_manifest_full_path =
            CString::new(app_manifest_full_path.to_string_lossy().as_bytes()).unwrap();
        let result = unsafe {
            self.0.AddApplicationManifest.unwrap()(app_manifest_full_path.as_ptr() as _, temporary)
        };

        if result == EVRApplicationError_VRApplicationError_None {
            Ok(())
        } else {
            Err(result)
        }
    }

    /** Removes an application manifest from the list to load when building the list of installed applications. */
    pub fn remove_application_manifest(
        &self,
        app_manifest_full_path: &Path,
    ) -> Result<(), EVRApplicationError> {
        let app_manifest_full_path =
            CString::new(app_manifest_full_path.to_string_lossy().as_bytes()).unwrap();

        let result = unsafe {
            self.0.RemoveApplicationManifest.unwrap()(app_manifest_full_path.as_ptr() as _)
        };

        if result == EVRApplicationError_VRApplicationError_None {
            Ok(())
        } else {
            Err(result)
        }
    }

    /** Returns true if an application is installed */
    pub fn is_application_installed(&self, app_key: &str) -> bool {
        let app_key = CString::new(app_key).unwrap();

        unsafe { self.0.IsApplicationInstalled.unwrap()(app_key.as_ptr() as _) }
    }

    /** Returns the number of applications available in the list */
    pub fn get_application_count(&self) -> u32 {
        unsafe { self.0.GetApplicationCount.unwrap()() }
    }

    /** Returns the key of the specified application. The index is at least 0 and is less than the return
     * value of GetApplicationCount(). The buffer should be at least k_unMaxApplicationKeyLength in order to
     * fit the key. */
    pub fn get_application_key_by_index(
        &self,
        app_index: u32,
    ) -> Result<String, EVRApplicationError> {
        let mut buffer = [0; k_unMaxApplicationKeyLength as _];
        unsafe {
            let result = self.0.GetApplicationKeyByIndex.unwrap()(
                app_index,
                buffer.as_mut_ptr(),
                buffer.len() as _,
            );
            if result == EVRApplicationError_VRApplicationError_None {
                Ok(CStr::from_ptr(buffer.as_ptr())
                    .to_str()
                    .unwrap()
                    .to_string())
            } else {
                Err(result)
            }
        }
    }

    /** Returns the key of the application for the specified Process Id. The buffer should be at least
     * k_unMaxApplicationKeyLength in order to fit the key. */
    pub fn get_application_key_by_process_id(
        &self,
        process_id: u32,
    ) -> Result<String, EVRApplicationError> {
        let mut buffer = [0; k_unMaxApplicationKeyLength as _];
        unsafe {
            let result = self.0.GetApplicationKeyByProcessId.unwrap()(
                process_id,
                buffer.as_mut_ptr(),
                buffer.len() as _,
            );
            if result == EVRApplicationError_VRApplicationError_None {
                Ok(CStr::from_ptr(buffer.as_ptr())
                    .to_str()
                    .unwrap()
                    .to_string())
            } else {
                Err(result)
            }
        }
    }

    /** Launches the application. The existing scene application will exit and then the new application will start.
     * This call is not valid for dashboard overlay applications. */
    pub fn launch_application(&self, app_key: &str) -> Result<(), EVRApplicationError> {
        let app_key = CString::new(app_key).unwrap();
        let result = unsafe { self.0.LaunchApplication.unwrap()(app_key.as_ptr() as _) };

        if result == EVRApplicationError_VRApplicationError_None {
            Ok(())
        } else {
            Err(result)
        }
    }

    /** Sets the application auto-launch flag. This is only valid for applications which return true for VRApplicationProperty_IsDashboardOverlay_Bool. */
    pub fn set_application_auto_launch(
        &self,
        app_key: &str,
        auto_launch: bool,
    ) -> Result<(), EVRApplicationError> {
        let app_key = CString::new(app_key).unwrap();
        let result =
            unsafe { self.0.SetApplicationAutoLaunch.unwrap()(app_key.as_ptr() as _, auto_launch) };
        if result == EVRApplicationError_VRApplicationError_None {
            Ok(())
        } else {
            Err(result)
        }
    }

    /** Gets the application auto-launch flag. This is only valid for applications which return true for VRApplicationProperty_IsDashboardOverlay_Bool. */
    pub fn get_application_auto_launch(&self, app_key: &str) -> bool {
        let app_key = CString::new(app_key).unwrap();
        unsafe { self.0.GetApplicationAutoLaunch.unwrap()(app_key.as_ptr() as _) }
    }
}

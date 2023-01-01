extern crate openvr_sys;
#[macro_use]
extern crate lazy_static;

use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{error, fmt, mem, ptr};

use openvr_sys as sys;

mod tracking;

pub mod chaperone;
pub mod compositor;
pub mod property;
pub mod render_models;
pub mod system;
pub mod applications;

pub use tracking::*;

pub use sys::VkDevice_T;
pub use sys::VkInstance_T;
pub use sys::VkPhysicalDevice_T;
pub use sys::VkQueue_T;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize OpenVR
///
/// # Panics
///
/// When the library has already been initialized
///
/// # Safety
///
/// The `Context` MUST be dropped or shut down with `Context::shutdown` before shutting down the graphics API. No OpenVR
/// calls may be made on object derived from a `Context` after the `Context` has been dropped or explicitly shut down.
pub unsafe fn init(ty: ApplicationType) -> Result<Context, InitError> {
    if INITIALIZED.swap(true, Ordering::Acquire) {
        panic!("OpenVR has already been initialized!");
    }

    let mut error = sys::EVRInitError_VRInitError_None;
    sys::VR_InitInternal(&mut error, ty as sys::EVRApplicationType);
    if error != sys::EVRInitError_VRInitError_None {
        return Err(InitError(error));
    }
    if !sys::VR_IsInterfaceVersionValid(sys::IVRSystem_Version.as_ptr() as *const i8) {
        sys::VR_ShutdownInternal();
        return Err(InitError(
            sys::EVRInitError_VRInitError_Init_InterfaceNotFound,
        ));
    }
    Ok(Context { live: AtomicBool::new(true) })
}

pub struct Applications(pub &'static sys::VR_IVRApplications_FnTable);
pub struct Chaperone(pub &'static sys::VR_IVRChaperone_FnTable);
pub struct ChaperoneSetup(pub &'static sys::VR_IVRChaperoneSetup_FnTable);
pub struct Compositor(pub &'static sys::VR_IVRCompositor_FnTable);
pub struct DriverManager(pub &'static sys::VR_IVRDriverManager_FnTable);
pub struct ExtendedDisplay(pub &'static sys::VR_IVRExtendedDisplay_FnTable);
pub struct Input(pub &'static sys::VR_IVRInput_FnTable);
pub struct IOBuffer(pub &'static sys::VR_IVRIOBuffer_FnTable);
pub struct Notifications(pub &'static sys::VR_IVRNotifications_FnTable);
pub struct Overlay(pub &'static sys::VR_IVROverlay_FnTable);
pub struct RenderModels(pub &'static sys::VR_IVRRenderModels_FnTable);
pub struct Resources(pub &'static sys::VR_IVRResources_FnTable);
pub struct Screenshots(pub &'static sys::VR_IVRScreenshots_FnTable);
pub struct Settings(pub &'static sys::VR_IVRSettings_FnTable);
pub struct SpatialAnchors(pub &'static sys::VR_IVRSpatialAnchors_FnTable);
pub struct System(pub &'static sys::VR_IVRSystem_FnTable);
pub struct TrackedCamera(pub &'static sys::VR_IVRTrackedCamera_FnTable);

/// Entry points into OpenVR.
///
/// At most one of this object may exist at a time.
///
/// See safety notes in `init`.
pub struct Context { live: AtomicBool }

fn load<T>(suffix: &[u8]) -> Result<*const T, InitError> {
    let mut magic = Vec::from(b"FnTable:".as_ref());
    magic.extend(suffix);
    let mut error = sys::EVRInitError_VRInitError_None;
    let result = unsafe { sys::VR_GetGenericInterface(magic.as_ptr() as *const i8, &mut error) };
    if error != sys::EVRInitError_VRInitError_None {
        return Err(InitError(
            sys::EVRInitError_VRInitError_Init_InterfaceNotFound,
        ));
    }
    Ok(result as *const T)
}

impl Context {
    pub fn applications(&self) -> Result<Applications, InitError> {
        load(sys::IVRApplications_Version).map(|x| unsafe {Applications(&*x)})
    }
    pub fn chaperone(&self) -> Result<Chaperone, InitError> {
        load(sys::IVRChaperone_Version).map(|x| unsafe {Chaperone(&*x)})
    }
    pub fn chaperone_setup(&self) -> Result<ChaperoneSetup, InitError> {
        load(sys::IVRChaperoneSetup_Version).map(|x| unsafe {ChaperoneSetup(&*x)})
    }
    pub fn compositor(&self) -> Result<Compositor, InitError> {
        load(sys::IVRCompositor_Version).map(|x| unsafe {Compositor(&*x)})
    }
    pub fn driver_manager(&self) -> Result<DriverManager, InitError> {
        load(sys::IVRDriverManager_Version).map(|x| unsafe {DriverManager(&*x)})
    }
    pub fn extended_display(&self) -> Result<ExtendedDisplay, InitError> {
        load(sys::IVRExtendedDisplay_Version).map(|x| unsafe {ExtendedDisplay(&*x)})
    }
    pub fn input(&self) -> Result<Input, InitError> {
        load(sys::IVRInput_Version).map(|x| unsafe {Input(&*x)})
    }
    pub fn io_buffer(&self) -> Result<IOBuffer, InitError> {
        load(sys::IVRIOBuffer_Version).map(|x| unsafe {IOBuffer(&*x)})
    }
    pub fn notifications(&self) -> Result<Notifications, InitError> {
        load(sys::IVRNotifications_Version).map(|x| unsafe {Notifications(&*x)})
    }
    pub fn overlay(&self) -> Result<Overlay, InitError> {
        load(sys::IVROverlay_Version).map(|x| unsafe {Overlay(&*x)})
    }
    pub fn render_models(&self) -> Result<RenderModels, InitError> {
        load(sys::IVRRenderModels_Version).map(|x| unsafe {RenderModels(&*x)})
    }
    pub fn resources(&self) -> Result<Resources, InitError> {
        load(sys::IVRResources_Version).map(|x| unsafe {Resources(&*x)})
    }
    pub fn screenshots(&self) -> Result<Screenshots, InitError> {
        load(sys::IVRScreenshots_Version).map(|x| unsafe {Screenshots(&*x)})
    }
    pub fn settings(&self) -> Result<Settings, InitError> {
        load(sys::IVRSettings_Version).map(|x| unsafe {Settings(&*x)})
    }
    pub fn spatial_anchors(&self) -> Result<SpatialAnchors, InitError> {
        load(sys::IVRSpatialAnchors_Version).map(|x| unsafe {SpatialAnchors(&*x)})
    }
    pub fn system(&self) -> Result<System, InitError> {
        load(sys::IVRSystem_Version).map(|x| unsafe {System(&*x)})
    }
    pub fn tracked_camera(&self) -> Result<TrackedCamera, InitError> {
        load(sys::IVRTrackedCamera_Version).map(|x| unsafe {TrackedCamera(&*x)})
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { self.shutdown() }
    }
}

impl Context {
    /// Shut down OpenVR. Repeated calls are safe.
    ///
    /// Called implicitly by `Context::drop`.
    ///
    /// # Safety
    ///
    /// This *must* be called *before* shutting down the graphics API, or OpenVR may invoke undefined behavior by
    /// attempting to free graphics resources.
    ///
    /// No calls to other OpenVR methods may be made after this has been called unless a new `Context` is first
   
    /// constructed.
    pub unsafe fn shutdown(&self) {
        if self.live.swap(false, Ordering::Acquire) {
            sys::VR_ShutdownInternal();
            INITIALIZED.store(false, Ordering::Release);
        }
    }


    /// # Safety
    ///
    /// it just works
    pub unsafe fn get_runtime_path(&self) -> Option<PathBuf> {
        if !self.live.load(Ordering::Relaxed) {
            return None;
        }

        let mut runtime_path = [0; 1024];
        let mut unrequired_size = 0;
        if sys::VR_GetRuntimePath(
            runtime_path.as_mut_ptr(),
            runtime_path.len() as _,
            &mut unrequired_size,
        ) && (unrequired_size as usize) < runtime_path.len()
        {
            CStr::from_ptr(runtime_path.as_ptr())
                .to_str()
                .ok()
                .map(|s| s.into())
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ApplicationType {
    /// Some other kind of application that isn't covered by the other entries
    Other = sys::EVRApplicationType_VRApplication_Other as isize,
    /// Application will submit 3D frames
    Scene = sys::EVRApplicationType_VRApplication_Scene as isize,
    /// Application only interacts with overlays
    Overlay = sys::EVRApplicationType_VRApplication_Overlay as isize,
    /// Application should not start SteamVR if it's not already running, and should not keep it running if everything
    /// else quits.
    Background = sys::EVRApplicationType_VRApplication_Background as isize,
    /// Init should not try to load any drivers. The application needs access to utility interfaces (like IVRSettings
    /// and IVRApplications) but not hardware.
    Utility = sys::EVRApplicationType_VRApplication_Utility as isize,
    /// Reserved for vrmonitor
    VRMonitor = sys::EVRApplicationType_VRApplication_VRMonitor as isize,
    /// Reserved for Steam
    SteamWatchdog = sys::EVRApplicationType_VRApplication_SteamWatchdog as isize,
    /// Start up SteamVR
    Bootstrapper = sys::EVRApplicationType_VRApplication_Bootstrapper as isize,
}

#[derive(Copy, Clone)]
pub struct InitError(sys::EVRInitError);

impl fmt::Debug for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = unsafe { CStr::from_ptr(sys::VR_GetVRInitErrorAsSymbol(self.0)) };
        f.pad(
            msg.to_str()
                .expect("OpenVR init error symbol was not valid UTF-8"),
        )
    }
}

impl error::Error for InitError {
    fn description(&self) -> &str {
        let msg = unsafe { CStr::from_ptr(sys::VR_GetVRInitErrorAsEnglishDescription(self.0)) };
        msg.to_str()
            .expect("OpenVR init error description was not valid UTF-8")
    }
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(error::Error::description(self))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Eye {
    Left = sys::EVREye_Eye_Left as isize,
    Right = sys::EVREye_Eye_Right as isize,
}

/// Helper to call OpenVR functions that return strings
unsafe fn get_string<F: FnMut(*mut std::os::raw::c_char, u32) -> u32>(mut f: F) -> Option<CString> {
    let n = f(ptr::null_mut(), 0);
    if n == 0 {
        return None;
    }
    let mut storage = Vec::new();
    storage.reserve_exact(n as usize);
    storage.resize(n as usize, mem::uninitialized());
    let n_ = f(storage.as_mut_ptr() as *mut _, n);
    assert!(n == n_);
    storage.truncate((n - 1) as usize); // Strip trailing null
    Some(CString::from_vec_unchecked(storage))
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ControllerAxis {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ControllerState {
    pub packet_num: u32,
    pub button_pressed: u64,
    pub button_touched: u64,
    pub axis: [ControllerAxis; 5],
}

pub mod button_id {
    use super::sys;
    pub const SYSTEM: sys::EVRButtonId = sys::EVRButtonId_k_EButton_System;
    pub const APPLICATION_MENU: sys::EVRButtonId = sys::EVRButtonId_k_EButton_ApplicationMenu;
    pub const GRIP: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Grip;
    pub const DPAD_LEFT: sys::EVRButtonId = sys::EVRButtonId_k_EButton_DPad_Left;
    pub const DPAD_UP: sys::EVRButtonId = sys::EVRButtonId_k_EButton_DPad_Up;
    pub const DPAD_RIGHT: sys::EVRButtonId = sys::EVRButtonId_k_EButton_DPad_Right;
    pub const DPAD_DOWN: sys::EVRButtonId = sys::EVRButtonId_k_EButton_DPad_Down;
    pub const A: sys::EVRButtonId = sys::EVRButtonId_k_EButton_A;
    pub const PROXIMITY_SENSOR: sys::EVRButtonId = sys::EVRButtonId_k_EButton_ProximitySensor;
    pub const AXIS0: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis0;
    pub const AXIS1: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis1;
    pub const AXIS2: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis2;
    pub const AXIS3: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis3;
    pub const AXIS4: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis4;
    pub const STEAM_VR_TOUCHPAD: sys::EVRButtonId = sys::EVRButtonId_k_EButton_SteamVR_Touchpad;
    pub const STEAM_VR_TRIGGER: sys::EVRButtonId = sys::EVRButtonId_k_EButton_SteamVR_Trigger;
    pub const DASHBOARD_BACK: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Dashboard_Back;
    pub const MAX: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Max;
}


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

#[link(name = "openvr_api")]
extern {}

extern "C" {
	pub fn VR_Init(err: *mut HmdError) -> *const ();
	pub fn VR_Shutdown();
	pub fn VR_IsHmdPresent() -> bool;
	pub fn VR_GetStringForHmdError(err: HmdError) -> *const u8;
}

pub type TrackedDeviceIndex_t = u32;
pub type VREvent_Data_t = [u8; 16];
pub type VRControllerState_t = VRControllerState001_t;
pub type VROverlayHandle_t = u64;
pub type VRNotificationId = u32;
#[repr(C)]
#[derive(Debug)]
pub enum Hmd_Eye {
	Left = 0,
	Right = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum GraphicsAPIConvention {
	DirectX = 0,
	OpenGL = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum HmdTrackingResult {
	Uninitialized = 1,
	Calibrating_InProgress = 100,
	Calibrating_OutOfRange = 101,
	Running_OK = 200,
	Running_OutOfRange = 201,
}

#[repr(C)]
#[derive(Debug)]
pub enum TrackedDeviceClass {
	Invalid = 0,
	HMD = 1,
	Controller = 2,
	TrackingReference = 4,
	Other = 1000,
}

#[repr(C)]
#[derive(Debug)]
pub enum TrackingUniverseOrigin {
	TrackingUniverseSeated = 0,
	TrackingUniverseStanding = 1,
	TrackingUniverseRawAndUncalibrated = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum TrackedDeviceProperty {
	TrackingSystemName_String = 1000,
	ModelNumber_String = 1001,
	SerialNumber_String = 1002,
	RenderModelName_String = 1003,
	WillDriftInYaw_Bool = 1004,
	ManufacturerName_String = 1005,
	TrackingFirmwareVersion_String = 1006,
	HardwareRevision_String = 1007,
	AllWirelessDongleDescriptions_String = 1008,
	ConnectedWirelessDongle_String = 1009,
	ReportsTimeSinceVSync_Bool = 2000,
	SecondsFromVsyncToPhotons_Float = 2001,
	DisplayFrequency_Float = 2002,
	UserIpdMeters_Float = 2003,
	CurrentUniverseId_Uint64 = 2004,
	PreviousUniverseId_Uint64 = 2005,
	DisplayFirmwareVersion_String = 2006,
	IsOnDesktop_Bool = 2007,
	AttachedDeviceId_String = 3000,
	SupportedButtons_Uint64 = 3001,
	Axis0Type_Int32 = 3002,
	Axis1Type_Int32 = 3003,
	Axis2Type_Int32 = 3004,
	Axis3Type_Int32 = 3005,
	Axis4Type_Int32 = 3006,
	FieldOfViewLeftDegrees_Float = 4000,
	FieldOfViewRightDegrees_Float = 4001,
	FieldOfViewTopDegrees_Float = 4002,
	FieldOfViewBottomDegrees_Float = 4003,
	TrackingRangeMinimumMeters_Float = 4004,
	TrackingRangeMaximumMeters_Float = 4005,
}

#[repr(C)]
#[derive(Debug)]
pub enum TrackedPropertyError {
	Success = 0,
	WrongDataType = 1,
	WrongDeviceClass = 2,
	BufferTooSmall = 3,
	UnknownProperty = 4,
	InvalidDevice = 5,
	CouldNotContactServer = 6,
	ValueNotProvidedByDevice = 7,
	StringExceedsMaximumLength = 8,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVREventType {
	None = 0,
	TrackedDeviceActivated = 100,
	TrackedDeviceDeactivated = 101,
	TrackedDeviceUpdated = 102,
	ButtonPress = 200,
	ButtonUnpress = 201,
	ButtonTouch = 202,
	ButtonUntouch = 203,
	MouseMove = 300,
	MouseButtonDown = 301,
	MouseButtonUp = 302,
	InputFocusCaptured = 400,
	InputFocusReleased = 401,
	SceneFocusLost = 402,
	SceneFocusGained = 403,
	OverlayShown = 500,
	OverlayHidden = 501,
	SystemOverlayActivated = 502,
	SystemOverlayDeactivated = 503,
	SystemOverlayThumbSelected = 504,
	Notification_Dismissed = 600,
	Notification_BeginInteraction = 601,
	Notification_Scroll = 602,
	Notification_ClickOn = 603,
	Notification_ClickOff = 604,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRButtonId {
	EButton_System = 0,
	EButton_ApplicationMenu = 1,
	EButton_Grip = 2,
	EButton_Axis0 = 32,
	EButton_Axis1 = 33,
	EButton_Axis2 = 34,
	EButton_Axis3 = 35,
	EButton_Axis4 = 36,
	EButton_Max = 64,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRMouseButton {
	Left = 1,
	Right = 2,
	Middle = 4,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRControllerAxisType {
	eControllerAxis_None = 0,
	eControllerAxis_TrackPad = 1,
	eControllerAxis_Joystick = 2,
	eControllerAxis_Trigger = 3,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRControllerEventOutputType {
	OSEvents = 0,
	VREvents = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum HmdError {
	None = 0,
	Unknown = 1,
	Init_InstallationNotFound = 100,
	Init_InstallationCorrupt = 101,
	Init_VRClientDLLNotFound = 102,
	Init_FileNotFound = 103,
	Init_FactoryNotFound = 104,
	Init_InterfaceNotFound = 105,
	Init_InvalidInterface = 106,
	Init_UserConfigDirectoryInvalid = 107,
	Init_HmdNotFound = 108,
	Init_NotInitialized = 109,
	Init_PathRegistryNotFound = 110,
	Init_NoConfigPath = 111,
	Init_NoLogPath = 112,
	Init_PathRegistryNotWritable = 113,
	Driver_Failed = 200,
	Driver_Unknown = 201,
	Driver_HmdUnknown = 202,
	Driver_NotLoaded = 203,
	Driver_RuntimeOutOfDate = 204,
	Driver_HmdInUse = 205,
	IPC_ServerInitFailed = 300,
	IPC_ConnectFailed = 301,
	IPC_SharedStateInitFailed = 302,
	IPC_CompositorInitFailed = 303,
	IPC_MutexInitFailed = 304,
	VendorSpecific_UnableToConnectToOculusRuntime = 1000,
	Steam_SteamInstallationNotFound = 2000,
}

#[repr(C)]
#[derive(Debug)]
pub enum CameraImageResult {
	OK = 0,
	Uninitalized = 1,
	NotReady = 2,
	SameFrame = 3,
}

#[repr(C)]
#[derive(Debug)]
pub enum ChaperoneCalibrationState {
	OK = 1,
	Warning = 100,
	Warning_BaseStationMayHaveMoved = 101,
	Warning_BaseStationRemoved = 102,
	Warning_SeatedBoundsInvalid = 103,
	Error = 200,
	Error_BaseStationUninitalized = 201,
	Error_BaseStationConflict = 202,
	Error_SoftBoundsInvalid = 203,
	Error_HardBoundsInvalid = 204,
}

#[repr(C)]
#[derive(Debug)]
pub enum Compositor_DeviceType {
	DeviceType_None = 0,
	DeviceType_D3D9 = 1,
	DeviceType_D3D9Ex = 2,
	DeviceType_D3D10 = 3,
	DeviceType_D3D11 = 4,
	DeviceType_OpenGL = 5,
}

#[repr(C)]
#[derive(Debug)]
pub enum VRCompositorError {
	None = 0,
	IncompatibleVersion = 100,
	DoNotHaveFocus = 101,
}

#[repr(C)]
#[derive(Debug)]
pub enum VROverlayError {
	None = 0,
	UnknownOverlay = 10,
	InvalidHandle = 11,
	PermissionDenied = 12,
	OverlayLimitExceeded = 13,
	WrongVisibilityType = 14,
	KeyTooLong = 15,
	NameTooLong = 16,
	KeyInUse = 17,
	WrongTransformType = 18,
	InvalidTrackedDevice = 19,
}

#[repr(C)]
#[derive(Debug)]
pub enum VROverlayInputMethod {
	None = 0,
	Mouse = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum VROverlayVisibility {
	Manual = 0,
	SystemOverlay = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum VROverlayTransformType {
	Absolute = 0,
	TrackedDeviceRelative = 1,
	SystemOverlay = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum VROverlayFlags {
	None = 0,
	Curved = 1,
	RGSS4X = 2,
}

#[repr(C)]
pub struct HmdMatrix34_t {
	m: [[f32; 4]; 3],
}
#[repr(C)]
pub struct HmdMatrix44_t {
	m: [[f32; 4]; 4],
}
#[repr(C)]
pub struct HmdVector3_t {
	v: [f32; 3],
}
#[repr(C)]
pub struct HmdVector3d_t {
	v: [f64; 3],
}
#[repr(C)]
pub struct HmdVector2_t {
	v: [f32; 2],
}
#[repr(C)]
pub struct HmdQuaternion_t {
	w: f64,
	x: f64,
	y: f64,
	z: f64,
}
#[repr(C)]
pub struct HmdQuad_t {
	vCorners: HmdVector3_t,
}
#[repr(C)]
pub struct DistortionCoordinates_t {
	rfRed: [f32; 2],
	rfGreen: [f32; 2],
	rfBlue: [f32; 2],
}
#[repr(C)]
pub struct TrackedDevicePose_t {
	mDeviceToAbsoluteTracking: HmdMatrix34_t,
	vVelocity: HmdVector3_t,
	vAngularVelocity: HmdVector3_t,
	eTrackingResult: HmdTrackingResult,
	bPoseIsValid: bool,
	bDeviceIsConnected: bool,
}
#[repr(C)]
pub struct RenderModel_Vertex_t {
	vPosition: HmdVector3_t,
	vNormal: HmdVector3_t,
	rfTextureCoord: [f32; 2],
}
#[repr(C)]
pub struct RenderModel_TextureMap_t {
	unWidth: u16,
	unHeight: u16,
	rubTextureMapData: *mut u8,
}
#[repr(C)]
pub struct RenderModel_t {
	ulInternalHandle: u64,
	rVertexData: RenderModel_Vertex_t,
	unVertexCount: u32,
	rIndexData: *mut u16,
	unTriangleCount: u32,
	diffuseTexture: RenderModel_TextureMap_t,
}
#[repr(C)]
pub struct VRTextureBounds_t {
	uMin: f32,
	vMin: f32,
	uMax: f32,
	vMax: f32,
}
#[repr(C)]
pub struct VREvent_Controller_t {
	button: EVRButtonId,
}
#[repr(C)]
pub struct VREvent_Mouse_t {
	x: f32,
	y: f32,
	button: EVRMouseButton,
}
#[repr(C)]
pub struct VREvent_Notification_t {
	x: f32,
	y: f32,
	notificationId: u32,
}
#[repr(C)]
pub struct VREvent_Process_t {
	pid: u32,
	oldPid: u32,
}
#[repr(C)]
pub struct VREvent_Overlay_t {
	overlayHandle: u64,
}
#[repr(C)]
pub struct VREvent_Reserved_t {
	reserved0: u64,
	reserved1: u64,
}
#[repr(C)]
pub struct VREvent_t {
	eventType: EVREventType,
	trackedDeviceIndex: TrackedDeviceIndex_t,
	data: VREvent_Data_t,
	eventAgeSeconds: f32,
}
#[repr(C)]
pub struct HiddenAreaMesh_t {
	pVertexData: HmdVector2_t,
	unTriangleCount: u32,
}
#[repr(C)]
pub struct VRControllerAxis_t {
	x: f32,
	y: f32,
}
#[repr(C)]
pub struct VRControllerState001_t {
	unPacketNum: u32,
	ulButtonPressed: u64,
	ulButtonTouched: u64,
	rAxis: VRControllerAxis_t,
}
#[repr(C)]
pub struct Compositor_OverlaySettings {
	size: u32,
	curved: bool,
	antialias: bool,
	scale: f32,
	distance: f32,
	alpha: f32,
	uOffset: f32,
	vOffset: f32,
	uScale: f32,
	vScale: f32,
	gridDivs: f32,
	gridWidth: f32,
	gridScale: f32,
	transform: HmdMatrix44_t,
}
#[repr(C)]
pub struct CameraInfo_t {
	width: u32,
	height: u32,
	depth: u32,
	fx: f32,
	cx: f32,
	fy: f32,
	cy: f32,
}
#[repr(C)]
pub struct CameraImage_t {
	frameID: i32,
	pose: HmdMatrix34_t,
	pBuffer: *const u8,
	unBufferLen: u32,
	result: CameraImageResult,
}
#[repr(C)]
pub struct ChaperoneSoftBoundsInfo_t {
	quadCorners: HmdQuad_t,
}
#[repr(C)]
pub struct ChaperoneSeatedBoundsInfo_t {
	vSeatedHeadPosition: HmdVector3_t,
	vDeskEdgePositions: HmdVector3_t,
}
#[repr(C)]
pub struct Compositor_FrameTiming {
	size: u32,
	frameStart: f64,
	frameVSync: f32,
	droppedFrames: u32,
	frameIndex: u32,
	pose: TrackedDevicePose_t,
}
#[repr(C)]
pub struct VROverlayIntersectionParams_t {
	vSource: HmdVector3_t,
	vDirection: HmdVector3_t,
	eOrigin: TrackingUniverseOrigin,
}
#[repr(C)]
pub struct VROverlayIntersectionResults_t {
	vPoint: HmdVector3_t,
	vNormal: HmdVector3_t,
	vUVs: HmdVector2_t,
	fDistance: f32,
}
#[repr(C)]
pub struct NotificationBitmap {
	bytes: *const u8,
	width: i32,
	height: i32,
	depth: i32,
}
extern "C" {
	pub fn VR_IVRSystem_GetWindowBounds(ptr: *const (), pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32) ;
	pub fn VR_IVRSystem_GetRecommendedRenderTargetSize(ptr: *const (), pnWidth: *mut u32, pnHeight: *mut u32) ;
	pub fn VR_IVRSystem_GetEyeOutputViewport(ptr: *const (), eEye: Hmd_Eye, pnX: *mut u32, pnY: *mut u32, pnWidth: *mut u32, pnHeight: *mut u32) ;
	pub fn VR_IVRSystem_GetProjectionMatrix(ptr: *const (), eEye: Hmd_Eye, fNearZ: f32, fFarZ: f32, eProjType: GraphicsAPIConvention) -> HmdMatrix44_t;
	pub fn VR_IVRSystem_GetProjectionRaw(ptr: *const (), eEye: Hmd_Eye, pfLeft: *mut f32, pfRight: *mut f32, pfTop: *mut f32, pfBottom: *mut f32) ;
	pub fn VR_IVRSystem_ComputeDistortion(ptr: *const (), eEye: Hmd_Eye, fU: f32, fV: f32) -> DistortionCoordinates_t;
	pub fn VR_IVRSystem_GetEyeToHeadTransform(ptr: *const (), eEye: Hmd_Eye) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetTimeSinceLastVsync(ptr: *const (), pfSecondsSinceLastVsync: *mut f32, pulFrameCounter: *mut u64) -> bool;
	pub fn VR_IVRSystem_GetD3D9AdapterIndex(ptr: *const (), ) -> i32;
	pub fn VR_IVRSystem_GetDXGIOutputInfo(ptr: *const (), pnAdapterIndex: *mut i32, pnAdapterOutputIndex: *mut i32) ;
	pub fn VR_IVRSystem_AttachToWindow(ptr: *const (), hWnd: *mut ()) -> bool;
	pub fn VR_IVRSystem_GetDeviceToAbsoluteTrackingPose(ptr: *const (), eOrigin: TrackingUniverseOrigin, fPredictedSecondsToPhotonsFromNow: f32, pTrackedDevicePoseArray: TrackedDevicePose_t, unTrackedDevicePoseArrayCount: u32) ;
	pub fn VR_IVRSystem_ResetSeatedZeroPose(ptr: *const (), ) ;
	pub fn VR_IVRSystem_GetSeatedZeroPoseToStandingAbsoluteTrackingPose(ptr: *const (), ) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetTrackedDeviceClass(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t) -> TrackedDeviceClass;
	pub fn VR_IVRSystem_IsTrackedDeviceConnected(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRSystem_GetBoolTrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> bool;
	pub fn VR_IVRSystem_GetFloatTrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> f32;
	pub fn VR_IVRSystem_GetInt32TrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> i32;
	pub fn VR_IVRSystem_GetUint64TrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> u64;
	pub fn VR_IVRSystem_GetMatrix34TrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetStringTrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pchValue: *const u8, unBufferSize: u32, pError: *mut TrackedPropertyError) -> u32;
	pub fn VR_IVRSystem_GetPropErrorNameFromEnum(ptr: *const (), error: TrackedPropertyError) -> *const u8;
	pub fn VR_IVRSystem_PollNextEvent(ptr: *const (), pEvent: VREvent_t) -> bool;
	pub fn VR_IVRSystem_PollNextEventWithPose(ptr: *const (), eOrigin: TrackingUniverseOrigin, pEvent: *mut VREvent_t, pTrackedDevicePose: *mut TrackedDevicePose_t) -> bool;
	pub fn VR_IVRSystem_GetEventTypeNameFromEnum(ptr: *const (), eType: EVREventType) -> *const u8;
	pub fn VR_IVRSystem_GetHiddenAreaMesh(ptr: *const (), eEye: Hmd_Eye) -> HiddenAreaMesh_t;
	pub fn VR_IVRSystem_GetControllerState(ptr: *const (), unControllerDeviceIndex: TrackedDeviceIndex_t, pControllerState: *mut VRControllerState_t) -> bool;
	pub fn VR_IVRSystem_GetControllerStateWithPose(ptr: *const (), eOrigin: TrackingUniverseOrigin, unControllerDeviceIndex: TrackedDeviceIndex_t, pControllerState: *mut VRControllerState_t, pTrackedDevicePose: TrackedDevicePose_t) -> bool;
	pub fn VR_IVRSystem_TriggerHapticPulse(ptr: *const (), unControllerDeviceIndex: TrackedDeviceIndex_t, unAxisId: u32, usDurationMicroSec: u16) ;
	pub fn VR_IVRSystem_GetButtonIdNameFromEnum(ptr: *const (), eButtonId: EVRButtonId) -> *const u8;
	pub fn VR_IVRSystem_GetControllerAxisTypeNameFromEnum(ptr: *const (), eAxisType: EVRControllerAxisType) -> *const u8;
	pub fn VR_IVRSystem_CaptureInputFocus(ptr: *const (), ) -> bool;
	pub fn VR_IVRSystem_ReleaseInputFocus(ptr: *const (), ) ;
	pub fn VR_IVRSystem_IsInputFocusCapturedByAnotherProcess(ptr: *const (), ) -> bool;
	pub fn VR_IVRSystem_DriverDebugRequest(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, pchRequest: *const u8, pchResponseBuffer: *const u8, unResponseBufferSize: u32) -> u32;
	pub fn VR_IVRCameraAccess_GetCameraCount(ptr: *const (), ) -> u32;
	pub fn VR_IVRCameraAccess_GetCameraId(ptr: *const (), unCameraIndex: u32, pchBuffer: *const u8, unBufferLen: u32) -> u32;
	pub fn VR_IVRCameraAccess_EnableCamera(ptr: *const (), unCameraIndex: u32, bEnabled: bool) -> bool;
	pub fn VR_IVRCameraAccess_GetCameraInfo(ptr: *const (), unCameraIndex: u32, pCameraInfo: CameraInfo_t) -> bool;
	pub fn VR_IVRCameraAccess_GetCameraImage(ptr: *const (), unCameraIndex: u32, pCameraImage: CameraImage_t) -> bool;
	pub fn VR_IVRChaperone_GetCalibrationState(ptr: *const (), ) -> ChaperoneCalibrationState;
	pub fn VR_IVRChaperone_GetSoftBoundsInfo(ptr: *const (), pInfo: ChaperoneSoftBoundsInfo_t) -> bool;
	pub fn VR_IVRChaperone_GetHardBoundsInfo(ptr: *const (), pQuadsBuffer: HmdQuad_t, punQuadsCount: *mut u32) -> bool;
	pub fn VR_IVRChaperone_GetSeatedBoundsInfo(ptr: *const (), pInfo: ChaperoneSeatedBoundsInfo_t) -> bool;
	pub fn VR_IVRChaperoneSetup_CommitWorkingCopy(ptr: *const (), pchCalibrationName: *const u8) -> bool;
	pub fn VR_IVRChaperoneSetup_RevertWorkingCopy(ptr: *const (), ) ;
	pub fn VR_IVRChaperoneSetup_GetWorkingSoftBoundsInfo(ptr: *const (), pInfo: ChaperoneSoftBoundsInfo_t) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingHardBoundsInfo(ptr: *const (), pQuadsBuffer: HmdQuad_t, punQuadsCount: *mut u32) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingSeatedZeroPoseToRawTrackingPose(ptr: *const (), pmatSeatedZeroPoseToRawTrackingPose: HmdMatrix34_t) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingStandingZeroPoseToRawTrackingPose(ptr: *const (), pmatStandingZeroPoseToRawTrackingPose: HmdMatrix34_t) -> bool;
	pub fn VR_IVRChaperoneSetup_SetWorkingSoftBoundsInfo(ptr: *const (), pInfo: ChaperoneSoftBoundsInfo_t) ;
	pub fn VR_IVRChaperoneSetup_SetWorkingHardBoundsInfo(ptr: *const (), pQuadsBuffer: HmdQuad_t, unQuadsCount: u32) ;
	pub fn VR_IVRChaperoneSetup_SetWorkingSeatedZeroPoseToRawTrackingPose(ptr: *const (), matSeatedZeroPoseToRawTrackingPose: HmdMatrix34_t) ;
	pub fn VR_IVRChaperoneSetup_SetWorkingStandingZeroPoseToRawTrackingPose(ptr: *const (), matStandingZeroPoseToRawTrackingPose: HmdMatrix34_t) ;
	pub fn VR_IVRChaperoneSetup_GetWorkingTagPoses(ptr: *const (), pTagPosesBuffer: HmdMatrix34_t, punTagPosesCount: *mut u32) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingTagPoseScales(ptr: *const (), pflScaleBuffer: *mut f32, punTagPosesCount: *mut u32) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingTagPoseNameByIndex(ptr: *const (), nIndex: u32, pchBuffer: *const u8, unBufferSize: u32) -> u32;
	pub fn VR_IVRChaperoneSetup_GetWorkingTagPoseByName(ptr: *const (), pchTagName: *const u8, pmatTagPose: HmdMatrix34_t, pflScale: *mut f32) -> bool;
	pub fn VR_IVRChaperoneSetup_SetWorkingTagPoseByName(ptr: *const (), pchTagName: *const u8, matSeatedZeroPoseToRawTrackingPose: HmdMatrix34_t, flScale: f32) ;
	pub fn VR_IVRChaperoneSetup_RemoveWorkingTagPoseByName(ptr: *const (), pchTagName: *const u8) ;
	pub fn VR_IVRChaperoneSetup_RemoveAllWorkingTagPoses(ptr: *const (), ) ;
	pub fn VR_IVRChaperoneSetup_ReloadFromDisk(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_GetLastError(ptr: *const (), pchBuffer: *const u8, unBufferSize: u32) -> u32;
	pub fn VR_IVRCompositor_SetVSync(ptr: *const (), bVSync: bool) ;
	pub fn VR_IVRCompositor_GetVSync(ptr: *const (), ) -> bool;
	pub fn VR_IVRCompositor_SetGamma(ptr: *const (), fGamma: f32) ;
	pub fn VR_IVRCompositor_GetGamma(ptr: *const (), ) -> f32;
	pub fn VR_IVRCompositor_SetGraphicsDevice(ptr: *const (), eType: Compositor_DeviceType, pDevice: *mut ()) ;
	pub fn VR_IVRCompositor_WaitGetPoses(ptr: *const (), pRenderPoseArray: TrackedDevicePose_t, unRenderPoseArrayCount: u32, pGamePoseArray: TrackedDevicePose_t, unGamePoseArrayCount: u32) -> VRCompositorError;
	pub fn VR_IVRCompositor_Submit(ptr: *const (), eEye: Hmd_Eye, pTexture: *mut (), pBounds: VRTextureBounds_t) -> VRCompositorError;
	pub fn VR_IVRCompositor_ClearLastSubmittedFrame(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_GetFrameTiming(ptr: *const (), pTiming: Compositor_FrameTiming, unFramesAgo: u32) -> bool;
	pub fn VR_IVRCompositor_FadeToColor(ptr: *const (), fSeconds: f32, fRed: f32, fGreen: f32, fBlue: f32, fAlpha: f32, bBackground: bool) ;
	pub fn VR_IVRCompositor_FadeGrid(ptr: *const (), fSeconds: f32, bFadeIn: bool) ;
	pub fn VR_IVRCompositor_CompositorBringToFront(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_CompositorGoToBack(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_CompositorQuit(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_IsFullscreen(ptr: *const (), ) -> bool;
	pub fn VR_IVRCompositor_SetTrackingSpace(ptr: *const (), eOrigin: TrackingUniverseOrigin) ;
	pub fn VR_IVRCompositor_GetTrackingSpace(ptr: *const (), ) -> TrackingUniverseOrigin;
	pub fn VR_IVRCompositor_GetCurrentSceneFocusProcess(ptr: *const (), ) -> u32;
	pub fn VR_IVRCompositor_CanRenderScene(ptr: *const (), ) -> bool;
	pub fn VR_IVROverlay_FindOverlay(ptr: *const (), pchOverlayKey: *const u8, pOverlayHandle: *mut VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_CreateOverlay(ptr: *const (), pchOverlayKey: *const u8, pchOverlayFriendlyName: *const u8, pOverlayHandle: *mut VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_DestroyOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetHighQualityOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetHighQualityOverlay(ptr: *const (), ) -> VROverlayHandle_t;
	pub fn VR_IVROverlay_GetOverlayErrorNameFromEnum(ptr: *const (), error: VROverlayError) -> *const u8;
	pub fn VR_IVROverlay_SetOverlayFlag(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eOverlayFlag: VROverlayFlags, bEnabled: bool) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayFlag(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eOverlayFlag: VROverlayFlags, pbEnabled: *mut bool) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayAlpha(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, fAlpha: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayAlpha(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pfAlpha: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayGamma(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, fGamma: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayGamma(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pfGamma: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayWidthInMeters(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, fWidthInMeters: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayWidthInMeters(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pfWidthInMeters: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTextureBounds(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pOverlayTextureBounds: VRTextureBounds_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTextureBounds(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pOverlayTextureBounds: VRTextureBounds_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTransformType(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, peTransformType: *mut VROverlayTransformType) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTransformAbsolute(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eTrackingOrigin: TrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTransformAbsolute(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, peTrackingOrigin: *mut TrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTransformTrackedDeviceRelative(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, unTrackedDevice: TrackedDeviceIndex_t, pmatTrackedDeviceToOverlayTransform: HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTransformTrackedDeviceRelative(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, punTrackedDevice: *mut TrackedDeviceIndex_t, pmatTrackedDeviceToOverlayTransform: HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayVisibility(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, peOverlayVisibility: *mut VROverlayVisibility) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayVisibility(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eOverlayVisibility: VROverlayVisibility) -> VROverlayError;
	pub fn VR_IVROverlay_ShowOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_HideOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_IsOverlayVisible(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> bool;
	pub fn VR_IVROverlay_PollNextOverlayEvent(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pEvent: VREvent_t) -> bool;
	pub fn VR_IVROverlay_GetOverlayInputMethod(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, peInputMethod: *mut VROverlayInputMethod) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayInputMethod(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eInputMethod: VROverlayInputMethod) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayMouseScale(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pvecMouseScale: HmdVector2_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayMouseScale(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pvecMouseScale: HmdVector2_t) -> VROverlayError;
	pub fn VR_IVROverlay_ComputeOverlayIntersection(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pParams: VROverlayIntersectionParams_t, pResults: VROverlayIntersectionResults_t) -> bool;
	pub fn VR_IVROverlay_HandleControllerOverlayInteractionAsMouse(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, unControllerDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVROverlay_SetOverlayTexture(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pTexture: *mut ()) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayRaw(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pvBuffer: *mut (), unWidth: u32, unHeight: u32, unDepth: u32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayFromFile(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pchFilePath: *const u8) -> VROverlayError;
	pub fn VR_IVROverlay_IsSystemOverlayVisible(ptr: *const (), ) -> bool;
	pub fn VR_IVROverlay_IsActiveSystemOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> bool;
	pub fn VR_IVROverlay_SetSystemOverlaySceneProcess(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, unProcessId: u32) -> VROverlayError;
	pub fn VR_IVROverlay_GetSystemOverlaySceneProcess(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, punProcessId: *mut u32) -> VROverlayError;
	pub fn VR_IVRRenderModels_LoadRenderModel(ptr: *const (), pchRenderModelName: *const u8, pRenderModel: RenderModel_t) -> bool;
	pub fn VR_IVRRenderModels_FreeRenderModel(ptr: *const (), pRenderModel: RenderModel_t) ;
	pub fn VR_IVRRenderModels_GetRenderModelName(ptr: *const (), unRenderModelIndex: u32, pchRenderModelName: *const u8, unRenderModelNameLen: u32) -> u32;
	pub fn VR_IVRRenderModels_GetRenderModelCount(ptr: *const (), ) -> u32;
	pub fn VR_IVRNotifications_GetLastError(ptr: *const (), pchBuffer: *const u8, unBufferSize: u32) -> u32;
	pub fn VR_IVRNotifications_NotificationStart(ptr: *const (), _type: *const u8, texture: NotificationBitmap, notificationId: *mut VRNotificationId) -> bool;
	pub fn VR_IVRNotifications_UpdateTexture(ptr: *const (), notificationId: VRNotificationId, texture: NotificationBitmap) -> bool;
	pub fn VR_IVRNotifications_UpdateBitmap(ptr: *const (), notificationId: VRNotificationId, texture: NotificationBitmap) -> bool;
	pub fn VR_IVRNotifications_GetPointerLocation(ptr: *const (), notificationId: VRNotificationId, pointerActive: *mut bool, pointerX: *mut i32, pointerY: *mut i32) -> bool;
	pub fn VR_IVRNotifications_DismissNotification(ptr: *const (), notificationId: VRNotificationId) -> bool;
}

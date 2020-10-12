use crate::sys;

/// MSFS Context. Hashable.
#[derive(PartialEq, Hash)]
pub struct FsContext(sys::FsContext);

impl From<sys::FsContext> for FsContext {
    fn from(ctx: sys::FsContext) -> FsContext {
        FsContext(ctx)
    }
}

/// `PanelServiceID` is used in `GaugeCallback`s and is generated from
/// `sys::PANEL_SERVICE_*` constants.
#[repr(u32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PanelServiceID {
    PreQuery = sys::PANEL_SERVICE_PRE_QUERY,
    PostQuery = sys::PANEL_SERVICE_POST_QUERY,
    PreInstall = sys::PANEL_SERVICE_PRE_INSTALL,
    PostInstall = sys::PANEL_SERVICE_POST_INSTALL,
    PreInitialize = sys::PANEL_SERVICE_PRE_INITIALIZE,
    PostInitialize = sys::PANEL_SERVICE_POST_INITIALIZE,
    PreUpdate = sys::PANEL_SERVICE_PRE_UPDATE,
    PostUpdate = sys::PANEL_SERVICE_POST_UPDATE,
    PreGenerate = sys::PANEL_SERVICE_PRE_GENERATE,
    PostGenerate = sys::PANEL_SERVICE_POST_GENERATE,
    PreDraw = sys::PANEL_SERVICE_PRE_DRAW,
    PostDraw = sys::PANEL_SERVICE_POST_DRAW,
    PreKill = sys::PANEL_SERVICE_PRE_KILL,
    PostKill = sys::PANEL_SERVICE_POST_KILL,
    ConnectToWindow = sys::PANEL_SERVICE_CONNECT_TO_WINDOW,
    Disconnect = sys::PANEL_SERVICE_DISCONNECT,
    PanelOpen = sys::PANEL_SERVICE_PANEL_OPEN,
    PanelClose = sys::PANEL_SERVICE_PANEL_CLOSE,
}

/// MSFS Gauges are managed using this lifecycle callback.
pub type GaugeCallback = fn(&FsContext, PanelServiceID) -> GaugeCallbackResult;
pub type GaugeCallbackResult = Result<(), ()>;

pub use msfs_derive::gauge;

/// Bindings to the Legacy/gauges.h API
pub struct Legacy {}
impl Legacy {
    /// aircraft_varget
    pub fn aircraft_varget(simvar: sys::ENUM, units: sys::ENUM, index: sys::SINT32) -> f64 {
        unsafe { sys::aircraft_varget(simvar, units, index) }
    }

    /// get_aircraft_var_enum
    pub fn get_aircraft_var_enum(name: &str) -> sys::ENUM {
        unsafe {
            let name = std::ffi::CString::new(name).unwrap();
            sys::get_aircraft_var_enum(name.as_ptr())
        }
    }

    /// get_units_enum
    pub fn get_units_enum(unitname: &str) -> sys::ENUM {
        unsafe {
            let name = std::ffi::CString::new(unitname).unwrap();
            sys::get_units_enum(name.as_ptr())
        }
    }
}

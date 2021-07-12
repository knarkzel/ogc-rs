//! The ``pad`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the gamecube controller functions found in ``pad.h``.

use alloc::boxed::Box;
use core::mem;
use core::ops::{BitAnd, BitOr, BitXor};

/// Represents the pad service. No gamecube controllers can be read until an instance of
/// this struct is created. This service can only be created once!
pub struct Pad;

/// The controller to be read for the `pad` service.
#[derive(Copy, Clone)]
pub enum Controller {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

impl PartialEq<Controller> for u16 {
    fn eq(&self, other: &Controller) -> bool {
        *self == *other as u16
    }
}

/// The button to be checked for the `pad` service. This implementation is more flexible than using
/// enum, but looks uglier (here).
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod Button {
    const None: u16 = 0;
    const Left: u16 = 1;
    const Right: u16 = 2;
    const Down: u16 = 4;
    const Up: u16 = 8;
    const Z: u16 = 16;
    const R: u16 = 32;
    const L: u16 = 64;
    const A: u16 = 256;
    const B: u16 = 512;
    const X: u16 = 1024;
    const Y: u16 = 2048;
    const Start: u16 = 4096;
}

impl Pad {
    /// Initialization of the pad service.
    pub fn init() {
        unsafe { ogc_sys::PAD_Init() };
    }

    /// Scan all pads. Must be called every time before checking buttons.
    pub fn scan_pads() -> u32 {
        unsafe { ogc_sys::PAD_ScanPads() }
    }

    pub fn buttons_down(controller: Controller) -> u16 {
        unsafe { ogc_sys::PAD_ButtonsDown(controller as _) }
    }

    pub fn buttons_held(controller: Controller) -> u16 {
        unsafe { ogc_sys::PAD_ButtonsHeld(controller as _) }
    }

    pub fn buttons_up(controller: Controller) -> u16 {
        unsafe { ogc_sys::PAD_ButtonsUp(controller as _) }
    }

    pub fn stick_x(controller: Controller) -> i8 {
        unsafe { ogc_sys::PAD_StickX(controller as _) }
    }

    pub fn stick_y(controller: Controller) -> i8 {
        unsafe { ogc_sys::PAD_StickY(controller as _) }
    }

    pub fn sub_stick_x(controller: Controller) -> i8 {
        unsafe { ogc_sys::PAD_SubStickX(controller as _) }
    }

    pub fn sub_stick_y(controller: Controller) -> i8 {
        unsafe { ogc_sys::PAD_SubStickY(controller as _) }
    }

    pub fn trigger_l(controller: Controller) -> u8 {
        unsafe { ogc_sys::PAD_TriggerL(controller as _) }
    }

    pub fn trigger_r(controller: Controller) -> u8 {
        unsafe { ogc_sys::PAD_TriggerR(controller as _) }
    }

    /// Registers a sampling callback function.
    pub fn set_sampling_callback<F>(callback: Box<F>)
    where
        F: Fn(u32) -> (),
    {
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn() = mem::transmute(ptr);
            ogc_sys::PAD_SetSamplingCallback(Some(code));
        }
    }

    fn clamp() {}
    fn control_motor() {}
    fn read() {}
    fn recalibrate() {}
    fn reset() {}
    fn set_spec() {}
    fn sync() {}
}

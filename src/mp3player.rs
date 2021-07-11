//! The ``mp3player`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the mp3-related functions found in ``mp3player.h``.

/// Represents the mp3player service.
/// This service can only be created once!
pub struct Mp3Player;

impl Mp3Player {
    pub fn init() {
        unsafe { ogc_sys::MP3Player_Init() };
    }

    pub fn play_buffer(sound_buffer: &[u8]) {
        unsafe {
            ogc_sys::MP3Player_PlayBuffer(
                sound_buffer.as_ptr() as *const _,
                sound_buffer.len() as _,
                None,
            )
        };
    }

    pub fn volume(volume: u8) {
        unsafe { ogc_sys::MP3Player_Volume(volume as _) };
    }

    pub fn is_playing() -> bool {
        unsafe { ogc_sys::MP3Player_IsPlaying() > 0 }
    }

    pub fn stop() {
        unsafe { ogc_sys::MP3Player_Stop() };
    }
}

impl Drop for Mp3Player {
    fn drop(&mut self) {
        Self::stop();
    }
}

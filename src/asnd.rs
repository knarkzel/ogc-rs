//! The ``asnd`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the audio functions found in ``asnd.h``.

use alloc::boxed::Box;
use core::mem;

pub struct VoiceOptions {
    /// Voice slot to use for this sound. Valid values are 0..MAX_SND_VOICES.
    voice: u32,
    /// Format to use for this sound.
    format: VoiceFormat,
    /// Frequency to use, in Hz.
    pitch: u32,
    /// Delay to wait before playing, in milliseconds.
    delay: u32,
    /// Voice volume of the left channel.
    volume_left: u8,
    /// Voice volume of the right channel.
    volume_right: u8,
    callback: Option<Box<fn(i32) -> ()>>,
}

impl VoiceOptions {
    pub fn new() -> Self {
        Self {
            voice: 0,
            format: VoiceFormat::Stereo16Bit,
            pitch: 48000,
            delay: 0,
            volume_left: 255,
            volume_right: 255,
            callback: None,
        }
    }

    pub fn voice(mut self, voice: u32) -> Self {
        self.voice = voice;
        self
    }

    pub fn format(mut self, format: VoiceFormat) -> Self {
        self.format = format;
        self
    }

    pub fn pitch(mut self, pitch: u32) -> Self {
        self.pitch = pitch;
        self
    }

    pub fn delay(mut self, delay: u32) -> Self {
        self.delay = delay;
        self
    }

    pub fn volume_left(mut self, volume_left: u8) -> Self {
        self.volume_left = volume_left;
        self
    }

    pub fn volume_right(mut self, volume_right: u8) -> Self {
        self.volume_right = volume_right;
        self
    }

    pub fn callback(mut self, callback: Box<fn(i32) -> ()>) -> Self {
        self.callback = Some(callback);
        self
    }
}

pub enum VoiceFormat {
    Mono8Bit,
    Mono16Bit,
    Mono16BitBe,
    Stereo8Bit,
    Stereo16Bit,
    Stereo16BitBe,
    Mono8BitU,
    Mono16BitLE,
    Stereo8BitU,
    Stereo16BitLe,
}

impl VoiceFormat {
    pub fn as_i32(self) -> i32 {
        match self {
            VoiceFormat::Mono8Bit => 0,
            VoiceFormat::Mono16Bit => 1,
            VoiceFormat::Mono16BitBe => 1,
            VoiceFormat::Stereo8Bit => 2,
            VoiceFormat::Stereo16Bit => 3,
            VoiceFormat::Stereo16BitBe => 3,
            VoiceFormat::Mono8BitU => 4,
            VoiceFormat::Mono16BitLE => 5,
            VoiceFormat::Stereo8BitU => 6,
            VoiceFormat::Stereo16BitLe => 7,
        }
    }
}

/// Represents the asnd service.
/// No audio control can be done until an instance of this struct is created.
/// This service can only be created once!
pub struct Asnd;

/// Implementation of the asnd service.
#[allow(unused_unsafe)]
impl Asnd {
    /// Initializes the asnd lib and fixes the hardware sample rate to 48000hz.
    pub fn init() -> Self {
        unsafe {
            ogc_sys::ASND_Init();
            Self
        }
    }

    /// De-initializes the asnd lib. This is also called when `Asnd` gets dropped.
    pub fn end() {
        unsafe {
            ogc_sys::ASND_End();
        }
    }

    /// Pauses if true and resumes if false.
    pub fn pause(should_pause: bool) {
        unsafe {
            ogc_sys::ASND_Pause(should_pause as i32);
        }
    }

    /// Returns true if paused, false if not paused.
    pub fn is_paused() -> bool {
        unsafe { ogc_sys::ASND_Is_Paused() > 0 }
    }

    /// Returns the global time in milliseconds. Time is updated from the IRQ.
    pub fn get_time() -> u32 {
        unsafe { ogc_sys::ASND_GetTime() }
    }

    /// Returns the global sample counter. Can be used to implement timers with high precision.
    pub fn get_sample_counter() -> u32 {
        unsafe { ogc_sys::ASND_GetSampleCounter() }
    }

    /// Returns the samples sent from the IRQ in one tick.
    pub fn get_samples_per_tick() -> u32 {
        unsafe { ogc_sys::ASND_GetSamplesPerTick() }
    }

    /// Sets the global time, in milliseconds.
    pub fn set_time(time: u32) {
        unsafe {
            ogc_sys::ASND_SetTime(time);
        }
    }

    /// Sets a global callback for general purposes. It is called by the IRQ.
    pub fn set_callback<F>(callback: Box<F>)
    where
        F: Fn() -> (),
    {
        // TODO: Check if this implementation can be changed.
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn() = mem::transmute(ptr);
            ogc_sys::ASND_SetCallback(Some(code));
        }
    }

    /// Returs the current audio rate. Default is 48000hz.
    pub fn get_audio_rate() -> i32 {
        unsafe { ogc_sys::ASND_GetAudioRate() }
    }

    /// Sets a PCM voice to play. This function stops one previous voice. Use
    /// `Asnd::status_voice()` to test status. The voices are played in 16-bit stereo,
    /// regardless of source format.
    pub fn set_voice(options: VoiceOptions, sound_buffer: &mut [u32]) {
        let callback = options.callback.map(|f| {
            let ptr = Box::into_raw(f);
            let code: unsafe extern "C" fn(i32) = unsafe { mem::transmute(ptr) };
            code
        });

        unsafe {
            ogc_sys::ASND_SetVoice(
                options.voice as i32,
                options.format.as_i32(),
                options.pitch as i32,
                options.delay as i32,
                sound_buffer.as_mut_ptr() as *mut _,
                sound_buffer.len() as i32,
                options.volume_left as i32,
                options.volume_right as i32,
                callback,
            );
        }
    }

    /// Sets a PCM voice to play infinitely. See `Asnd::set_voice()` as it is largely identical.
    pub fn set_infinite_voice(options: VoiceOptions, sound_buffer: &mut [u32]) {
        unsafe {
            ogc_sys::ASND_SetInfiniteVoice(
                options.voice as i32,
                options.format.as_i32(),
                options.pitch as i32,
                options.delay as i32,
                sound_buffer.as_mut_ptr() as *mut _,
                sound_buffer.len() as i32,
                options.volume_left as i32,
                options.volume_right as i32,
            );
        }
    }

    fn add_voice() {
        unsafe {}
    }

    fn stop_voice() {
        unsafe {}
    }

    fn pause_voice() {
        unsafe {}
    }

    fn status_voice() {
        unsafe {}
    }

    fn get_first_unused_voice() {
        unsafe {}
    }

    fn change_pitch_voice() {
        unsafe {}
    }

    fn change_volume_voice() {
        unsafe {}
    }

    fn get_tick_counter_voice() {
        unsafe {}
    }

    fn get_timer_voice() {
        unsafe {}
    }

    fn test_pointer() {
        unsafe {}
    }

    fn test_voicebuffer_ready() {
        unsafe {}
    }

    fn get_dsp_percentuse() {
        unsafe {}
    }

    fn get_dsp_processtime() {
        unsafe {}
    }
}

impl Drop for Asnd {
    fn drop(&mut self) {
        Self::end();
    }
}

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]

extern crate block;
#[macro_use]
extern crate cfg_if;
extern crate core_audio_types;
#[macro_use]
extern crate core_foundation;
extern crate core_graphics_types;
extern crate core_video;
extern crate libc;

pub type OSType = u32;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[cfg_attr(feature = "link", link(name = "CoreMedia", kind = "framework"))]
extern "C" {}

pub mod attachment;
#[cfg(target_os = "ios")]
pub mod audio_clock;
pub mod audio_device_clock;
pub mod base;
pub mod block_buffer;
pub mod buffer_queue;
pub mod format_description;
pub mod format_description_bridge;
pub mod sample_buffer;
pub mod sample_queue;
pub mod sync;
pub mod time;
pub mod time_range;

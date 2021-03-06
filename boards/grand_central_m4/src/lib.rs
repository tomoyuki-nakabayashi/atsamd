#![no_std]
#![recursion_limit = "1024"]

/// Pin definitions
pub mod pins;

use atsamd_hal as hal;

use hal::*;

pub use hal::common::*;
pub use hal::samd51::*;
pub use hal::target_device as pac;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
pub use pins::Pins;

#[cfg(feature = "use_uart_debug")]
pub use hal::dbgprint;

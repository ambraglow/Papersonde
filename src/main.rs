#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use modules::sonde::RSM4X2;
use {defmt_rtt as _, panic_probe as _}; // global logger

pub mod sonde;
#[cfg(feature = "stm32f1xx")]
use sonde::RSM4X2;
#[cfg(feature = "stm32l4xx")]
use sonde::RSM4X4;

#[entry]
fn main() -> ! {
    #[cfg(feature = "stm32f1xx")]
    let sonde = RSM4X2 {
        Sonde: Sonde::self_init()
    };
    #[cfg(feature = "stm32l4xx")]
    let sonde = RSM4X4 {
        Sonde: Sonde::self_init()
    };

} 
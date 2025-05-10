use defmt::*;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;
use {defmt_rtt as _, panic_probe as _};

struct SondePins {
    led_green : Pin,
    led_red : Pin,
    psu_shutdown : Pin,
    vbat : Pin,
    vbtn : Pin,
    si4032mosi : Pin,
    si4032miso : Pin,
    si4032sck : Pin,
    si4032cs : Pin,
    //heat_ref = PC6,
    thermal_reference : Pin,
    pullup_tm : Pin,
    pullup_hyg : Pin,
    spst1 : Pin,
    spst2 : Pin3,
    spst3 : Pin,
    spst4 : Pin,
    spdt1 : Pin,
    spdt2 : Pin,
    spdt3 : Pin,
    ring_oscillator_meas : Pin,
    heat_hum1 : Pin,
    heat_hum2 : Pin,
    gps_reset : Pin,
    gps_rx : Pin,
    gps_tx : Pin
}

enum PinsRSM4X4 {
    LedGreen = PC7,
    LedRed = PC8,
    PsuShudown = PA9,
    VBat = PA5,
    VBtn = PA6,
    Si4032mosi = PB15,
    Si4032miso = PB14,
    Si4032sck = PB13,
    Si4032cs = PC13,
    //heat_ref = PC6,
    ThermalReference = PB1,
    PullupTM = PB12,
    PullupHyg = PA2,
    Spst1 = PB3,
    Spst2 = PA3,
    Spst3 = PC14,
    Spst4 = PC15,
    Spdt1 = PC10,
    Spdt2 = PC11,
    Spdt3 = PC12,
    RingOscillatorMeas = PA1,
    HeatHum1 = PA7,
    HeatHum2 = PB8,
    GpsReset = PB9,
    GpsRX = PB7,
    GpsTX = PB6
    // TODO: other pins
}

enum PinsRSM4X2 {
    LedGreen = PB7,
    LedRed = PB8,
    PsuShudown = PA12,
    VBat = PA5,
    VBtn = PA6,
    Si4032mosi = PB15,
    Si4032miso = PB14,
    Si4032sck = PB13,
    Si4032cs = PC13,
    //heat_ref = false,
    ThermalReference = PB1,
    PullupTM = PB12,
    PullupHyg = PA2,
    Spst1 = PB6,
    Spst2 = PA3,
    Spst3 = PC14,
    Spst4 = PC15,
    Spdt1 = PB3,
    Spdt2 = PB4,
    Spdt3 = PB5,
    RingOscillatorMeas = PA1,
    HeatHum1 = PA7,
    HeatHum2 = PB9,
    GpsReset = PA15,
    GpsRX = PA10,
    GpsTX = PA9
    // TODO: other pins
}


pub trait Sonde {
    fn si4032_init(&self) -> !;
    fn gps_init(&self) -> !;

    fn self_init(&self) -> self {
        let p = embassy_stm32::init(Default::default());

        #[cfg(feature = "stm32f1xx")]
        SondePins {
            led_green : PinsRSM4X2.LedGreen,
            led_red : PinsRSM4X2.LedRed,
            si4032mosi : PinsRSM4X2.Si4032mosi,
            si4032miso : PinsRSM4X2.Si4032miso,
            si4032sck : PinsRSM4X2.Si4032sck,
            si4032cs : PinsRSM4X2.Si4032cs
        }
            #[cfg(feature = "stm32l4xx")]
        SondePins {
            led_green : PinsRSM4X4.LedGreen,
            led_red : PinsRSM4X4.LedRed,
            si4032mosi : PinsRSM4X4.Si4032mosi,
            si4032miso : PinsRSM4X4.Si4032miso,
            si4032sck : PinsRSM4X4.Si4032sck,
            si4032cs : PinsRSM4X4.Si4032cs
        }
    }
}

#[cfg(feature = "stm32f1xx")]
pub impl Sonde for RSM4X2 {
    fn si4032_init(&self) -> ! {
        let mut spi_config = Config::default();
        spi_config.frequency = Hertz(1_000_000);

        let mut spi = Spi::new_blocking(p.SPI3, self.Si4032sck, self.Si4032mosi, self.Si4032miso, spi_config);

        let mut cs = Output::new(self.Si4032cs, Level::High, Speed::VeryHigh);

        return 0;
    }

    fn gps_init(&self) -> ! {
        return 0;
    }
}

#[cfg(feature = "stm32l4xx")]
pub impl Sonde for RSM4X4 {
    fn si4032_init(&self) -> ! {
        let mut spi_config = Config::default();
        spi_config.frequency = Hertz(1_000_000);

        let mut spi = Spi::new_blocking(p.SPI3, self.Si4032sck, self.Si4032mosi, self.Si4032miso, spi_config);

        let mut cs = Output::new(self.Si4032cs, Level::High, Speed::VeryHigh);

        return 0;
    }

    fn gps_init(&self) -> ! {
        return 0;
    }
}

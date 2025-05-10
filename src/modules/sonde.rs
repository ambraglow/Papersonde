use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
struct RSM4X4Pins {
    green_led: Pin = PC7,
    red_led: Pin = PC8,
    psu_shutdown : Pin = PA9,
    vbat : Pin = PA5,
    vbtn : Pin = PA6,
    mosi_radio_spi : Pin = PB15,
    miso_radio_spi : Pin = PB14,
    sck_radio_spi : Pin = PB13,
    cs_radio_spi : Pin = PC13,
    // TODO: other pins
}

impl Sonde {
    
}
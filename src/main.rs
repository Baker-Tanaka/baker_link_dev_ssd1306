#![no_std]
#![no_main]

use defmt_rtt as _;
use embedded_graphics::{
    mono_font::{ascii::FONT_7X13, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};
use embedded_hal::delay::DelayNs;
use hal::fugit::RateExtU32;
use hal::pac;
use panic_probe as _;
use rp2040_hal as hal;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin = pins.gpio14.reconfigure();
    // SCLピンを再構成
    let scl_pin = pins.gpio15.reconfigure();

    // I2Cインターフェースを初期化
    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    // I2Cディスプレイインターフェースを作成
    let interface = I2CDisplayInterface::new(i2c);

    // SSD1306ディスプレイを初期化
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // テキストスタイルを設定
    let text_sytle = MonoTextStyleBuilder::new()
        .font(&FONT_7X13)
        .text_color(BinaryColor::On)
        .build();

    // 表示するテキストを設定
    let text_line1 = Text::with_baseline("Hello World.", Point::zero(), text_sytle, Baseline::Top);

    // 画面クリア用の矩形を設定
    let clear = Rectangle::new(Point::zero(), Size::new(128, 64))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off));

    // メインループ
    loop {
        // テキストを描画
        text_line1.draw(&mut display).unwrap();
        display.flush().unwrap();
        timer.delay_ms(1500u32);

        // 画面をクリア
        clear.draw(&mut display).unwrap();
        display.flush().unwrap();
        timer.delay_ms(1500u32);
    }
}

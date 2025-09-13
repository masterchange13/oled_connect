#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embedded_graphics::mono_font::ascii::FONT_6X13;
use embedded_graphics::pixelcolor;
use embedded_graphics::primitives::{Styled, PrimitiveStyleBuilder, Circle};
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, Rgb888},
    prelude::*,
    text::{Baseline, Text},
    primitives::rectangle::Rectangle,
    mock_display::{
        MockDisplay,
    },
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

use esp_hal::i2c::master::{I2c, Config};

use esp_hal::uart::{Uart, Config as UartConfig};

#[main]
fn main() -> ! {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let mut uart = Uart::new(_peripherals.UART0, UartConfig::default())
        .unwrap() // 初始化失败时直接 panic
        .with_rx(_peripherals.GPIO1)
        .with_tx(_peripherals.GPIO2);


    let sda = _peripherals.GPIO21;
    let scl = _peripherals.GPIO22;

    // 使用 match 处理 Result
    let i2c = match I2c::new(_peripherals.I2C0, Config::default()) {
        Ok(i2c) => i2c
            .with_sda(sda)
            .with_scl(scl),
        Err(e) => {
            // 处理错误，例如陷入循环或恐慌
            panic!("Failed to initialize I2C: {:?}", e);
            // 或者在嵌入式环境中：
            // loop {}
        }
    };
    
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X13)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();




    // TODO
    let style = PrimitiveStyleBuilder::new()
        //  .fill_color(Rgb888::WHITE)
        .fill_color(pixelcolor::BinaryColor::On)
        //  .stroke_color(Rgb888::BLACK)
         .stroke_width(6)
         .build();
    let mut display_rectangle = MockDisplay::new();
    // Draw a filled square
    Rectangle::new(Point::new(10, 10), Size::new(50, 50))
         .into_styled(style)
         .draw(&mut display_rectangle);




    let circle = Circle::new(Point::new(0, 0), 40);
    let mut display_circle = MockDisplay::from_points(circle.points(), BinaryColor::On);
    Rectangle::new(Point::new(10, 10), Size::new(50, 50))
        .into_styled(style)
        .draw(&mut display_circle);




    Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    let mut buf = [0u8; 1]; // 单字节缓冲

    loop {
        match uart.read(&mut buf) {
            Ok(n) if n > 0 => {         // n 是实际读取的字节数
                let byte = buf[0];      // 获取第一个字节

                display.clear(BinaryColor::On);

                match byte {
                    1 => {
                        Text::with_baseline("Hello!", Point::new(0, 0), text_style, Baseline::Top)
                            .draw(&mut display).unwrap();
                    },
                    2 => {
                        Text::with_baseline("World!", Point::new(0, 0), text_style, Baseline::Top)
                            .draw(&mut display).unwrap();
                    },
                    3 => {
                        Text::with_baseline("ESP32 + Rust!", Point::new(0, 0), text_style, Baseline::Top)
                            .draw(&mut display).unwrap();
                    },
                    _ => {}
                }

                display.flush().unwrap();
            },
            Err(_e) => {
                // 可以选择忽略错误或者做错误处理
            },
            _ => {} // 没有读取到字节
        }
    }


}

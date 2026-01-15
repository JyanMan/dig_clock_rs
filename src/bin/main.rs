#![no_std]
#![no_main]

#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]


use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_radio::ble::controller::BleConnector;
use log::{warn, info, error};
use trouble_host::prelude::*;
use dig_clock_rs::*;
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use static_cell::StaticCell;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

static STOPWATCH_UI_CH: StaticCell<
    Channel<CriticalSectionRawMutex, crate::lcd_graphics::ClockUiUpdate, 10>
> = StaticCell::new();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> () {
    // generator version: 1.1.0

    esp_idf_logger::init().unwrap();
    // esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    // LCD
    if let Ok(display) = lcd_graphics::display_init(
        peripherals.SPI2,
        peripherals.GPIO18,
        peripherals.GPIO19,
        peripherals.GPIO21,
        peripherals.GPIO22,
        peripherals.GPIO4,
        peripherals.GPIO5
    ).await {

        let stopwatch_ui_ch = STOPWATCH_UI_CH.init(
            Channel::<CriticalSectionRawMutex, crate::lcd_graphics::ClockUiUpdate, 10>::new());

        let _ = spawner.spawn(stopwatch_ui::increment_stopwatch(stopwatch_ui_ch.sender()));

        if let Err(err) = spawner.spawn(
            lcd_graphics::update_task(display, stopwatch_ui_ch.receiver())
        ) {
            error!("[lcd] failed to create task: {:?}", err);
        }
        else {
            info!("[lcd] successfully created task");
        }
    }

    // BLE
    // let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    // let (mut _wifi_controller, _interfaces) =
    //     esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
    //         .expect("Failed to initialize Wi-Fi controller");
    // let connector = BleConnector::new(&radio_init, peripherals.BT, Default::default()).unwrap();
    // let controller: ExternalController<_, 1> = ExternalController::new(connector);

    // ble_setup::ble_bas_peripheral_run(controller).await;
}


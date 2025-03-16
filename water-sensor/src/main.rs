use esp_idf_hal::gpio::{Pull, PinDriver};
use esp_idf_hal::peripherals::Peripherals;
use std::thread;
use std::time::Duration;
use esp_idf_svc::log::EspLogger;

fn main() {
    // Necesario para los parches de ESP-IDF
    esp_idf_svc::sys::link_patches();

    // Inicializar el sistema de logs
    EspLogger::initialize_default();

    // Obtener los periféricos del ESP32
    let peripherals = Peripherals::take().unwrap();

    // Configurar GPIO 5 como entrada con pull-up
    let mut sensor = PinDriver::input(peripherals.pins.gpio5).unwrap();
    sensor.set_pull(Pull::Up).unwrap();

    log::info!("✅ Sensor de agua iniciado. Leyendo datos...");

    loop {
        let estado = sensor.is_low(); // LOW = Agua detectada, HIGH = Sin agua
        if estado {
            log::info!("🚨 Agua detectada!");
        } else {
            log::info!("✅ No hay agua.");
        }

        // Esperar 1 segundo antes de leer de nuevo
        thread::sleep(Duration::from_secs(1));
    }
}

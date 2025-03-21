use esp_idf_hal::gpio::{PinDriver, Pull, Output, Input};
use esp_idf_hal::peripherals::Peripherals;
use std::thread;
use std::time::Duration;
use esp_idf_svc::log::EspLogger;

fn main() {
    // Necesario para parches del ESP-IDF
    esp_idf_svc::sys::link_patches();

    // Inicializar logs
    EspLogger::initialize_default();

    // Obtener periféricos del ESP32
    let peripherals = Peripherals::take().unwrap();

    // Configurar GPIO 5 como entrada con pull-up para el sensor de agua
    let mut sensor = PinDriver::input(peripherals.pins.gpio5).unwrap();
    sensor.set_pull(Pull::Up).unwrap();

    // Configurar GPIO 18 como salida para el relé
    let mut rele = PinDriver::output(peripherals.pins.gpio18).unwrap();
    rele.set_high().unwrap(); // Inicialmente, relé apagado

    log::info!("✅ Sistema iniciado. Monitoreando nivel de agua...");

    loop {
        let estado_agua = sensor.is_low(); // LOW = Agua detectada, HIGH = Sin agua

        if estado_agua {
            log::info!("🚨 Agua detectada. Cerrando la válvula.");
            rele.set_high().unwrap(); // Apagar el relé (válvula cerrada)
        } else {
            log::info!("✅ Nivel bajo. Abriendo la válvula...");
            rele.set_low().unwrap(); // Encender el relé (válvula abierta)
        }

        // Esperar 1 segundo antes de leer de nuevo
        thread::sleep(Duration::from_secs(1));
    }
}

use modbus::tcp::{Config, Transport};
mod modbus_registers;
use serde::Deserialize;

// Structure to hold inverter configuration
#[derive(Deserialize, Debug)]
struct InverterConfig {
    name: String,
    ip_address: String,
    port: u16,
}

fn load_inverter_config() -> Vec<InverterConfig> {
    let config_data = include_str!("config/inverter_config.json");
    let config: Vec<InverterConfig> =
        serde_json::from_str(config_data).expect("Failed to parse inverter config");
    config
}

fn main() {
    println!("Starting Sungrow Modbus Backend...");

    let register_config: modbus_registers::RegisterConfig =
        modbus_registers::load_register_config();

    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear console

        for inverter in &load_inverter_config() {
            let cfg = Config::default();
            let mut client = match Transport::new_with_cfg(&inverter.ip_address, cfg) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to connect: {:?}", e);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    continue;
                }
            };

            println!("Connected successfully.");

            println!("Reading registers...");
            let register_values =
                match modbus_registers::read_registers(&register_config, &mut client) {
                    Ok(values) => values,
                    Err(e) => {
                        eprintln!("Error reading registers: {:?}", e);
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        continue;
                    }
                };

            println!("Register values: {:?}", register_values);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

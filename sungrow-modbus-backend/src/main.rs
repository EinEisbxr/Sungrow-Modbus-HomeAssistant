use modbus::tcp::{Config, Transport};
mod modbus_registers;

fn main() {
    println!("Connecting to Sungrow Inverter at 192.168.178.131:502...");

    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear console

        let register_config: modbus_registers::RegisterConfig =
            modbus_registers::load_register_config();

        let cfg = Config::default();
        let mut client = match Transport::new_with_cfg("192.168.178.131", cfg) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to connect: {:?}", e);
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
        };

        println!("Connected successfully.");

        println!("Reading registers...");
        let register_values = match modbus_registers::read_registers(register_config, &mut client) {
            Ok(values) => values,
            Err(e) => {
                eprintln!("Error reading registers: {:?}", e);
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
        };

        println!("Register values: {:?}", register_values);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

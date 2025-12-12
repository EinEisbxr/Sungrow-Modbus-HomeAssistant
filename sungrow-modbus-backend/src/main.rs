use modbus::tcp::{Config, Transport};
mod modbus_registers;

fn main() {
    println!("Connecting to Sungrow Inverter at 192.168.178.131:502...");

    loop {
        let cfg = Config::default();
        let mut client = match Transport::new_with_cfg("192.168.178.131", cfg) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to connect: {:?}", e);
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
        };

        let _device_type = modbus_registers::read_device_type(&mut client);
        let _nominal_output_power = modbus_registers::read_nominal_output_power(&mut client);
        let _dc_power = modbus_registers::read_total_dc_power(&mut client);

        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

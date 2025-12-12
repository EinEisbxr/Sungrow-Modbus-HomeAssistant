use modbus::{Client, tcp};

pub fn read_device_type(client: &mut tcp::Transport) -> Result<Vec<u16>, modbus::Error> {
    match client.read_input_registers(4999, 1) {
        Ok(data) => {
            let device_code = data[0];
            println!(
                "Device Type Code: 0x{:02X} (decimal: {})",
                device_code, device_code
            );
            Ok(data)
        }
        Err(e) => {
            eprintln!("Failed to read device type: {:?}", e);
            Err(e)
        }
    }
}

pub fn read_nominal_output_power(client: &mut tcp::Transport) -> Result<Vec<u16>, modbus::Error> {
    match client.read_input_registers(5000, 1) {
        Ok(data) => {
            let power = data[0] as f32 / 10.0; // Value is in 0.1 kW
            println!("Nominal Output Power: {} kW", power);
            Ok(data)
        }
        Err(e) => {
            eprintln!("Failed to read nominal output power: {:?}", e);
            Err(e)
        }
    }
}

pub fn read_total_dc_power(client: &mut tcp::Transport) -> Result<Vec<u16>, modbus::Error> {
    match client.read_input_registers(5016, 1) {
        Ok(data) => {
            let power = data[0];
            println!("Total DC Power: {} W", power);
            Ok(data)
        }
        Err(e) => {
            eprintln!("Failed to read total dc power: {:?}", e);
            Err(e)
        }
    }
}

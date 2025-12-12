use modbus::{Client, tcp};
use serde::Deserialize;

// Structure to hold the register configuration
#[derive(Deserialize, Debug)]
pub struct RegisterConfig {
    pub registers: Vec<ModbusRegister>,
}

// Definition of a Modbus register
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ModbusRegister {
    pub name: String,
    pub label: String,
    pub address: u16,
    pub unit: Option<String>,
    pub scale: Option<f64>,
    pub output_mapping: Option<String>,
}

// Structure to hold device type information
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DeviceType {
    pub model: String,
    pub code: String,
    pub mppt_count: u8,
    pub string_per_mppt: String,
}

// Add enum to represent register values
#[derive(Debug)]
#[allow(dead_code)]
pub enum RegisterValue {
    Numeric(u16),
    Float(String),
    Text(String),
}

// Function to load register configuration from a JSON file
pub fn load_register_config() -> RegisterConfig {
    let config_data = include_str!("config/register_config.json");
    let registers: RegisterConfig =
        serde_json::from_str(config_data).expect("Failed to parse register config");
    registers
}

// Function to load device types from a JSON file
pub fn load_device_types() -> Vec<DeviceType> {
    let device_data = include_str!("config/device_type_mapping.json");
    let device_types: Vec<DeviceType> =
        serde_json::from_str(device_data).expect("Failed to parse device types");
    device_types
}

// Function to read registers based on the configuration
pub fn read_registers(
    registers: RegisterConfig,
    client: &mut tcp::Transport,
) -> Result<Vec<(String, RegisterValue)>, modbus::Error> {
    let mut values = Vec::new();

    // Read each register defined in the configuration
    for reg in registers.registers {
        let value: u16 = client.read_input_registers(reg.address, 1)?[0];

        if reg.name == "device_type" {
            let device_types = load_device_types();
            let hex_value = format!("0x{:X}", value);

            if let Some(device) = device_types.iter().find(|d| d.code == hex_value) {
                println!("Device Type: {} (Code: {})", device.model, device.code);
                values.push((reg.name, RegisterValue::Text(device.model.clone())));
            } else {
                println!("Unknown Device Type Code: {}", hex_value);
                values.push((reg.name, RegisterValue::Text("Unknown".to_string())));
            }
        } else {
            let final_value: RegisterValue = if let Some(scale) = reg.scale {
                let scaled = (value as f64) * scale;
                // Determine decimal places from scale value
                let decimal_places = if scale.fract() == 0.0 {
                    0
                } else {
                    // Use format to count actual decimal places needed
                    let formatted = format!("{:.10}", scale);
                    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
                    if let Some(pos) = trimmed.find('.') {
                        trimmed.len() - pos - 1
                    } else {
                        0
                    }
                };

                // Format to string with specific precision to force rounding and preserve trailing zeros
                let format_string = format!("{:.1$}", scaled, decimal_places);

                RegisterValue::Float(format_string)
            } else {
                RegisterValue::Numeric(value)
            };

            values.push((reg.name, final_value));
        }
    }
    Ok(values)
}

use crate::tools;
use std::time::Duration;
use tools::*;

pub fn start_device(
    speed: u32,
    reset: bool,
    timeout: u64,
) -> Result<mcp2221::Handle, mcp2221::Error> {
    let mut config = mcp2221::Config::default();
    config.i2c_speed_hz = speed;
    config.reset_on_open = reset;
    // For talking to a peripheral we might want a higher timeout, but for
    // scanning the bus, a short timeout is good since it allows us to scan all
    // addresses more quickly.
    config.timeout = Duration::from_millis(timeout);
    let dev = match mcp2221::Handle::open_first(&config) {
        Ok(x) => x,
        Err(err) => {
            return Err(err);
        }
    };
    Ok(dev)
}

#[allow(dead_code)]
pub fn write_device(
    dev: &mut mcp2221::Handle,
    address: u8,
    register: u8,
    data: &mut Vec<u8>,
) -> Result<(), mcp2221::Error> {
    let mut my_data = vec![register];
    my_data.append(data);
    dev.write_read_address(address, &mut my_data, &mut [0u8])
}

#[allow(dead_code)]
pub fn read_device(
    dev: &mut mcp2221::Handle,
    address: u8,
    register: u8,
    expected: usize,
) -> Result<Vec<u8>, mcp2221::Error> {
    //println!("addr: {:#02x} reg {:#02x}", addr, register);
    let mut read = vec![0; expected];
    let mut reg: [u8; 1] = [0; 1];
    reg[0] = register;
    match dev.write_read_address(address, &mut reg, &mut read) {
        Ok(()) => Ok(read),
        Err(x) => Err(x),
    }
}

pub fn set_pin_dir(dev: &mut mcp2221::Handle, address: u8, port: Port, pin: Pin, dir: Direction) {
    let reg = Register::Iodir;
    let mut result = read_register(dev, address, reg, port);

    result = match dir {
        Direction::Input => bit_set(result, pin),
        Direction::Output => bit_clear(result, pin),
    };
    write_register(dev, address, port, reg, result);
}

pub fn write_pin(dev: &mut mcp2221::Handle, address: u8, port: Port, pin: Pin, value: u8) {
    let reg = Register::Olat;
    let mut result = read_register(dev, address, reg, port);
    //println!("result before: 0x{:08b} set/clear: {}", result, value);

    result = match value {
        1 => bit_set(result, pin),
        _ => bit_clear(result, pin),
    };
    //println!("result after: 0x{:08b}", result);
    write_register(dev, address, port, reg, result);
}

pub fn read_pin(dev: &mut mcp2221::Handle, address: u8, port: Port, pin: Pin) -> u8 {
    let reg = Register::Gpio;
    let result = read_register(dev, address, reg, port);

    bit_read(result, pin)
}

pub fn read_register(dev: &mut mcp2221::Handle, address: u8, reg: Register, port: Port) -> u8 {
    let mut read: [u8; 1] = [0; 1];
    let mut result: [u8; 1] = [0; 1];
    read[0] = reg as u8 | port as u8;
    let _ = dev.write_read_address(address, &mut read, &mut result);

    result[0]
}

pub fn write_register(
    dev: &mut mcp2221::Handle,
    address: u8,
    port: Port,
    reg: Register,
    value: u8,
) {
    let mut result: [u8; 2] = [0; 2];
    result[0] = reg as u8 | port as u8;
    result[1] = value;
    match dev.write_read_address(address, &mut result, &mut [0u8]) {
        Ok(()) => (),
        Err(x) => println!("Error: {}", x),
    }
}

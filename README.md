# `IOExtender Control Center (IOCC) - MITAC Plugin`

<p align=center>
  <a href="https://github.com/Thiapalm/iocc"><img src="https://img.shields.io/badge/depends%20on-iocc-blue"></a>
  <a href="https://github.com/Thiapalm/usbioextender"><img src="https://img.shields.io/badge/hardware-USBIOExtender-orange"></a>
  <img src="https://img.shields.io/badge/version-v0.1.0-red" alt="version">
</p>

## Description
This project is a plugin-style CLI tool for the IOExtender Control Center ecosystem.
It controls MITAC ignition through an MCP23017-compatible I/O expander connected over I2C via USBIOExtender.

This project is a plugin and is not intended to be used as a standalone tool.
The `iocc-mitac` executable must be in the same directory as the main `iocc` executable.

The binary provides simple commands to:
- Turn ignition ON
- Turn ignition OFF
- Read current ignition status

## Features
- Command line operation (`on`, `off`, `status`)
- YAML-based configuration via `config.yaml`
- Configurable I2C address and communication parameters
- Configurable GPIO mapping for MITAC ignition control
- Built with Rust and uses MCP2221 for USB-I2C communication

## Version Revision
- 0.1.0 - First public version in this repository

## Prerequisites
- Main IOCC executable (`iocc`) available in the same folder where `iocc-mitac` will be deployed
- USBIOExtender hardware: https://github.com/Thiapalm/usbioextender
- MCP2221 USB-I2C bridge available and connected
- MITAC hardware wired to the configured GPIO pin
- Rust toolchain (only required to build): https://www.rust-lang.org/

## Installation
### Build from source
1. Clone this repository:
   ```bash
   git clone https://github.com/thiapalm/iocc-mitac.git
   ```
2. Enter the project directory.
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The executable will be generated in `target/release/`.

### Deploy as an IOCC companion tool
1. Build `iocc-mitac` and `iocc`.
2. Place both executables in the same folder.
3. Place `config.yaml` in this same folder.
4. Keep `iocc-mitac` together with `iocc` at runtime (required).
5. Optionally add that folder to your system `PATH`.

## Configuration
`iocc-mitac` reads `config.yaml` from the current working directory.

Expected section:
```yaml
mitac:
  address: 0x20
  verbose: false
  i2c_speed_hz: 400000
  reset_on_open: false
  timeout: 10
  ignition:
    port: portb
    pin: pin1
    direction: output
```

Notes:
- `address` must be a hexadecimal value (`0x..`).
- `timeout` is in milliseconds.
- If parsing fails or values are missing, built-in defaults are used.

## Usage
`iocc-mitac` is meant to be deployed as an IOCC plugin binary (same folder as `iocc`) and executed through IOCC.

Show plugin help:
```bash
iocc mitac --help
```

Turn ignition ON:
```bash
iocc mitac on
```

Turn ignition OFF:
```bash
iocc mitac off
```

Read ignition status:
```bash
iocc mitac status
```

## Troubleshooting
- If you see an error like `check USB connection`, confirm the MCP2221 device is connected.
- Verify I2C address and pin mapping in `config.yaml`.
- Ensure no other process is locking the USB-I2C bridge.

## Contributing
1. Fork the repository.
2. Create a branch for your changes (`git checkout -b feature/AmazingFeature`).
3. Commit your updates.
4. Push your branch (`git push origin feature/AmazingFeature`).
5. Open a pull request.

## Contact
For questions or issues:
palmieri@adastra.aleeas.com

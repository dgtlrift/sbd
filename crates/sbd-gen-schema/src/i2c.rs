//! I2C related structs of the SBD schema.

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

/// Represents the MCU side of I2C Bus.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct I2cBus {
    /// Serial Clock Line pin.
    pub scl_pin: String,
    /// Serial Data Line pin.
    pub sda_pin: String,
    /// User-defined aliases for the bus.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// List of possible MCU I2C controller peripherals usable for this bus.
    #[serde(default)]
    pub possible_peripherals: Vec<String>,
    /// Devices attached to this bus.
    ///
    /// DISCUSSION SKETCH: not yet validated end-to-end, see sbd#134 follow-up.
    #[serde(default)]
    pub devices: Vec<I2cDevice>,
}

/// A device attached to an [`I2cBus`].
///
/// DISCUSSION SKETCH: devices are distinguished by `address` on the shared
/// bus, unlike [`crate::SpiDevice`] which is distinguished by a dedicated
/// `cs` pin. `interrupt` and `enable` are the two extra-pin shapes found on
/// real hardware so far (an interrupt line, as several sensors expose; a
/// separate enable/control pin, as the nRF9160 DK's `PCAL6408A` I/O expander
/// needs) -- open question whether that's the right generalization or
/// whether this should be a more open-ended extra-data field instead.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct I2cDevice {
    /// The device's type, e.g. a chip name like `hts221` or `pcal6408a`.
    #[serde(rename = "type")]
    pub type_: String,
    /// User-defined aliases for the device.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// The device's 7-bit I2C slave address, e.g. `"0x5F"`.
    pub address: String,
    /// Optional interrupt line from the device to the MCU.
    pub interrupt: Option<String>,
    /// Optional dedicated enable/control pin, separate from the I2C bus
    /// itself.
    pub enable: Option<String>,
}

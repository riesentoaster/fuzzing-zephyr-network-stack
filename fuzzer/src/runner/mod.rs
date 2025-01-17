#![allow(dead_code)]
use std::{ffi::CStr, sync::LazyLock, time::Duration};

use libafl::Error;
use libafl_bolts::shmem::ShMemDescription;
use smoltcp::wire::IpAddress;

#[cfg(feature = "coverage_stability")]
pub mod calibration_log_stage;

mod client;
mod executor;
pub mod feedback;
mod fuzzer;
mod input;
mod objective;
mod observer;

pub use {
    client::connect_to_zephyr,
    executor::ZepyhrExecutor,
    fuzzer::fuzz,
    observer::packet::{PacketMetadataFeedback, PacketObserver},
};

pub const ZEPHYR_IP: IpAddress = IpAddress::v4(192, 0, 2, 1);
pub const ZEPHYR_PORT: u16 = 4242;
pub const CLIENT_PORT: u16 = 13377;
pub const SETUP_TIMEOUT: Duration = Duration::from_millis(300 + 200); // time waited until client attempts to send data, the second is taken from the zephyr startup wait time
pub const INTER_SEND_WAIT: Duration = Duration::from_millis(200);

pub static IPV6_LINK_LOCAL_ADDR: LazyLock<IpAddress> = LazyLock::new(|| {
    IpAddress::v6(
        0xfe80, 0x0000, 0x0000, 0x0000, 0x0200, 0x5eff, 0xfe00, 0x53ff,
    )
});
pub const CLIENT_MAC_ADDR: [u8; 6] = [0x00, 0x00, 0x5e, 0x00, 0x53, 0xff];

pub(crate) fn get_path(shmem_desc: &ShMemDescription) -> Result<&str, Error> {
    CStr::from_bytes_until_nul(&shmem_desc.id)
        .map_err(|e| {
            Error::illegal_argument(format!("Error parsing path from shmem desc: {:?}", e))
        })?
        .to_str()
        .map_err(|e| {
            Error::illegal_argument(format!("Could not parse string from shmmem path: {:?}", e))
        })
}

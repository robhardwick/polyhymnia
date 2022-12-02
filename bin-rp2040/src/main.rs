#![no_std]
#![no_main]

use defmt::{debug, panic};
use defmt_rtt as _;
use panic_probe as _;

use libpoly::Poly;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[cortex_m_rt::entry]
fn main() -> ! {
    debug!("Program start");

    let mut poly = match Poly::new(0, 44100) {
        Err(_) => panic!("Init failed"),
        Ok(p) => p,
    };

    loop {
        debug!("{}", poly.next());
    }
}

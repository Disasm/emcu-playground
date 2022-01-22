#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m::asm::delay;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let dp = emcu_pac::Peripherals::take().unwrap_or_else(|| loop{});

    let gpio = dp.GPIO;
    unsafe {
        gpio.outenset.write(|w| w.bits(1));
    }

    let mut value = true;
    loop {
        for _ in 0..4 {
            gpio.dataout.modify(|r, w| unsafe {
                if value {
                    w.bits(r.bits() | 1)
                } else {
                    w.bits(r.bits() & !1)
                }
            });
            value = !value;
            delay(0x100000);
        }
        delay(0x200000);
    }
}

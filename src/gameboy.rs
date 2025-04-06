use std::time;
pub const CPU_CLOCK_HZ: u128 = 4_194_304;//一秒間に4194304クロック
pub const M_CYCLE_CLOCK: u128 = 4;//gbマシンサイクルが4クロック
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;//1マシンサイクル

pub fn run() {
    let time = time::Instant::now();
    let mut elapsed = 0;
    loop {
        let e = time.elapsed().as_nanos();
        for _ in 0..(e - elapsed) / M_CYCLE_NANOS {
            println!("{}", elapsed);
            elapsed += M_CYCLE_NANOS;
        }
    }
}

#[cfg(test)]
mod unit_test {
}
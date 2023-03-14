use std::thread::sleep;
use std::time::Duration;

use pigpio_sys as gpio;

fn main () {
	match unsafe { gpio::gpioInitialise() } {
		r if r < 0 => {
		   eprintln!("gpioInitialise failed: {}", r);
		   return;
		}
		_ => {}
	}

	let pin = 12;
	match unsafe { gpio::gpioSetMode(pin, gpio::PI_ALT0) } {
		r if r < 0 => {
		   eprintln!("gpioSetMode failed: {}\n", r);
		   return;
		}
		_ => {}
	}
	assert_eq!(unsafe { gpio::gpioGetMode(pin) } as u32, gpio::PI_ALT0);

	match unsafe { gpio::gpioSetPullUpDown(pin, gpio::PI_PUD_OFF) } {
		r if r < 0 => {
			eprintln!("gpioSetPullUpDown failed: {}.\n", r);
			return;
		}
		_ => {}
	}

	let range = 250;
	match unsafe { gpio::gpioSetPWMrange(pin, range) } {
		r if r != range as i32 => {
			eprintln!("gpioSetPWMrange failed: ({},{}): {}.\n", pin, range, r);
			return;
		}
		_ => {}
	}

	let delay = Duration::from_micros(20000);
	for i in 0..100 {
		sleep(delay);
		let dutycycle: f64 = (range as f64) / (100 as f64) * (i as f64);
		match unsafe { gpio::gpioPWM(pin, dutycycle as u32) } {
			r if r < 0 => {
				eprintln!("gpioPWM failed: ({},{}): {}.", pin, dutycycle, r);
				return ;
			}
			_ => {}
		}
	}
	for i in (0..100).rev() {
		sleep(delay);
		let dutycycle: f64 = (range as f64) / (100 as f64) * (i as f64);
		match unsafe { gpio::gpioPWM(pin, dutycycle as u32) } {
			r if r < 0 => {
				eprintln!("gpioPWM failed: ({},{}): {}.", pin, dutycycle, r);
				return ;
			}
			_ => {}
		}
	}
	sleep(Duration::from_secs(1));
}


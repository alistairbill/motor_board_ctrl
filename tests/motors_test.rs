extern crate motor_board_ctrl;

use motor_board_ctrl::motors::Motors;

use std::time::Duration;
use std::thread;

#[test]
fn motor_test() {
    let mut motors = Motors::init();
    for x in -20..21 {
        motors.motors_write(x * 10, x * 10);
        thread::sleep(Duration::from_millis(100));
    }
    motors.motors_stop();
}

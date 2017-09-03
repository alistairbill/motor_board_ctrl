extern crate motor_board_ctrl;

use motor_board_ctrl::motors::Motors;

use std::time::Duration;
use std::thread;

#[test]
fn motor_test() {
    let mut motors = Motors::init();
    let forwards = (0..21).chain((0..21).rev());
    let backwards = (-20..1).rev().chain((-20..1));

    for x in forwards.chain(backwards) {
        motors.motors_write(x * 10, x * 10);
        thread::sleep(Duration::from_millis(100));
    }

    motors.motors_stop();
}

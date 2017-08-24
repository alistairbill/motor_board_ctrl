extern crate motor_board_ctrl;

use motor_board_ctrl::motors::Motors;

use std::time::Duration;
use std::thread;

#[test]
fn left_motor_test() {
    let mut motors = Motors::init();
    motors.motors_write(200, 0);
    thread::sleep(Duration::from_secs(5));
    motors.motors_stop();
}

#[test]
fn right_motor_test() {
    let mut motors = Motors::init();
    motors.motors_write(0, 200);
    thread::sleep(Duration::from_secs(5));
    motors.motors_stop();
}

#[test]
fn motor_test() {
    let mut motors = Motors::init();
    for x in -20..21 {
        motors.motors_write(x * 10, x * 10);
        thread::sleep(Duration::from_secs(1));
    }
    motors.motors_stop();
}

extern crate motor_board_ctrl;

use motor_board_ctrl::motor_board::MotorBoard;

use std::time::Duration;
use std::thread;

#[test]
#[ignore]
fn motor_test() {
    let mut motors = MotorBoard::init();
    let forwards = (0..21).chain((0..21).rev());
    let backwards = (-20..1).rev().chain((-20..1));

    for x in forwards.chain(backwards) {
        motors.motors_write(x * 10, x * 10);
        thread::sleep(Duration::from_millis(100));
    }

    motors.motors_stop();
}

#[test]
fn trim_test() {
    let mut motor_board = MotorBoard::init();
    let trim = motor_board.read_trim();
    println!("trim = {}", trim);
    assert!(trim >= 0);
}

#[test]
fn ir_test() {
    let mut motor_board = MotorBoard::init();
    let ir = motor_board.read_ir();
    for x in ir.iter() {
        println!("ir = {}", x);
        assert!(x >= &0);
    }
}

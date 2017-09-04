extern crate motor_board_ctrl;

#[macro_use]
extern crate lazy_static;

use motor_board_ctrl::motor_board::MotorBoard;

use std::time::Duration;
use std::thread;
use std::sync::Mutex;

lazy_static! {
    static ref MOTOR_BOARD: Mutex<MotorBoard> = Mutex::new(MotorBoard::init());
}

#[test]
#[ignore]
fn motor_test() {
    let forwards = (0..21).chain((0..21).rev());
    let backwards = (-20..1).rev().chain((-20..1));

    for x in forwards.chain(backwards) {
        MOTOR_BOARD.lock().unwrap().motors_write(x * 10, x * 10);
        thread::sleep(Duration::from_millis(100));
    }

    MOTOR_BOARD.lock().unwrap().motors_stop();
}

#[test]
fn trim_test() {
    let trim = MOTOR_BOARD.lock().unwrap().read_trim();
    println!("trim = {}", trim);
    assert!(trim >= 0);
}

#[test]
fn ir_test() {
    let ir = MOTOR_BOARD.lock().unwrap().read_ir();
    for x in ir.iter() {
        println!("ir = {}", x);
        assert!(x >= &0);
    }
}

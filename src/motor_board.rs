use communication::{SerialConnection, Message};

use std::time::Duration;
use std::thread;

// const COMMAND_SWITCH_MODE: u8 = 0;
const COMMAND_MOTORS_RUN: u8 = 10;
const COMMAND_MOTORS_STOP: u8 = 11;
// const COMMAND_ANALOG_WRITE: u8 = 20;
const COMMAND_DIGITAL_WRITE: u8 = 30;
const COMMAND_ANALOG_READ: u8 = 40;
const COMMAND_ANALOG_READ_RE: u8 = 41;
const COMMAND_DIGITAL_READ: u8 = 50;
const COMMAND_DIGITAL_READ_RE: u8 = 51;
const COMMAND_READ_IR: u8 = 60;
const COMMAND_READ_IR_RE: u8 = 61;
// const COMMAND_ACTION_DONE: u8 = 70;
const COMMAND_READ_TRIM: u8 = 80;
const COMMAND_READ_TRIM_RE: u8 = 81;
// const COMMAND_PAUSE_MODE: u8 = 90;
// const COMMAND_LINE_FOLLOW_CONFIG: u8 = 100;

pub struct MotorBoard(SerialConnection);

impl MotorBoard {
    pub fn init() -> MotorBoard {
        let serial = SerialConnection::open("/dev/serial0").unwrap();
        MotorBoard(serial)
    }

    pub fn motors_stop(&mut self) {
        let mut buf = Message::new();
        buf.queue_byte(COMMAND_MOTORS_STOP);
        buf.create_checksum();
        self.0.send_data(&buf).unwrap();
    }

    pub fn motors_write(&mut self, speed_left: i16, speed_right: i16) {
        let mut buf = Message::new();
        buf.queue_byte(COMMAND_MOTORS_RUN);
        buf.queue_int(speed_left);
        buf.queue_int(speed_right);
        buf.create_checksum();
        self.0.send_data(&buf).unwrap();
    }

    pub fn turn_left(&mut self, speed: i16) {
        self.motors_write(speed, 255);
    }

    pub fn turn_right(&mut self, speed: i16) {
        self.motors_write(255, speed);
    }

    pub fn request_digital_read(&mut self, port: u8) -> u8 {
        let mut buf = Message::new();
        buf.queue_byte(COMMAND_DIGITAL_READ);
        buf.queue_byte(port); // B_TK1 - B_TK4
        buf.create_checksum();
        self.0.send_data(&buf).unwrap();
        thread::sleep(Duration::from_millis(10));
        let mut recv = self.0.receive_data().unwrap();
        recv.decreate_checksum();
        let cmd = recv.dequeue_byte();
        if cmd != COMMAND_DIGITAL_READ_RE {} // fail
        let _ = recv.dequeue_byte(); // pt: bottom tk port codename
        recv.dequeue_byte()
    }

    pub fn request_analog_read(&mut self, port: u8) -> i16 {
        let mut buf = Message::new();
        buf.queue_byte(COMMAND_ANALOG_READ);
        buf.queue_byte(port); // B_TK1 - B_TK4
        buf.create_checksum();
        self.0.send_data(&buf).unwrap();
        thread::sleep(Duration::from_millis(10));
        let mut recv = self.0.receive_data().unwrap();
        recv.decreate_checksum();
        let cmd = recv.dequeue_byte();
        if cmd != COMMAND_ANALOG_READ_RE {} // fail
        let _ = recv.dequeue_byte();
        recv.dequeue_int()
    }

    pub fn request_digital_write(&mut self, selector: u8, value: u8) {
        let mut buf = Message::new();
        buf.queue_byte(COMMAND_DIGITAL_WRITE);
        buf.queue_byte(selector); // B_TK1 - B_TK4
        buf.queue_byte(value);
        buf.create_checksum();
        self.0.send_data(&buf).unwrap();
    }

    pub fn read_ir(&mut self) -> [i16; 5] {
        let mut buf = Message::new();
        buf.queue_byte(COMMAND_READ_IR);
        buf.create_checksum();
        self.0.send_data(&buf).unwrap();
        thread::sleep(Duration::from_millis(10));
        let mut recv = self.0.receive_data().unwrap();
        recv.decreate_checksum();
        let cmd = recv.dequeue_byte();
        if cmd != COMMAND_READ_IR_RE {} // fail
        let mut ir_array: [i16; 5] = [0; 5];
        for i in 0..5 {
            ir_array[i] = recv.dequeue_int();
        }
        ir_array
    }

    pub fn read_trim(&mut self) -> i16 {
        let mut buf = Message::new();
        buf.queue_byte(COMMAND_READ_TRIM);
        buf.create_checksum();
        self.0.send_data(&buf).unwrap();
        thread::sleep(Duration::from_millis(10));
        let mut recv = self.0.receive_data().unwrap();
        recv.decreate_checksum();
        let cmd = recv.dequeue_byte();
        if cmd != COMMAND_READ_TRIM_RE {} //fail
        recv.dequeue_int()

    }
}

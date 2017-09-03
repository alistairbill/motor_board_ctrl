use std::io::prelude::*;
use std::time::Duration;

use serial::{self, SerialPort, SystemPort};

pub struct SerialConnection(SystemPort);

impl SerialConnection {
    pub fn open(port: &str) -> serial::Result<SerialConnection> {
        let mut serial = serial::open(port)?;
        serial.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            Ok(())
        }).and_then(|_| {
            serial.set_timeout(Duration::from_millis(100))
        })?;
        let mut buffer = Vec::new();
        let _ = serial.read_to_end(&mut buffer)?;
        let serial_connection = SerialConnection(serial);
        Ok(serial_connection)
    }

    pub fn send_data(&mut self, buf: &Message) -> serial::Result<()> {
        let &mut SerialConnection(ref mut serial) = self;
        serial.write(&buf.0)?;
        Ok(())
    }

    pub fn receive_data(&mut self) -> serial::Result<Message> {
        let &mut SerialConnection(ref mut serial) = self;
        let mut buf = Message::new();
        serial.read_to_end(&mut buf.0)?;
        Ok(buf)
    }
}

pub struct Message(Vec<u8>);

impl Message {
    pub fn new() -> Message {
        Message(Vec::new())
    }

    pub fn queue_byte(&mut self, dat: u8) {
        if self.0.len() < 20 {
            self.0.push(dat);
        }
    }

    pub fn queue_int(&mut self, dat: i16) {
        if self.0.len() < 19 {
            self.0.push(((dat >> 8) as u8));
            self.0.push((dat as u8));
        }
    }

    pub fn dequeue_byte(&mut self) -> u8 {
        self.0.remove(0)
    }

    pub fn dequeue_int(&mut self) -> i16 {
        let dat_1 = (self.0.remove(0) as i16) << 8;
        let dat_2 = self.0.remove(0) as i16;
        (dat_1 | dat_2)
    }

    pub fn create_checksum(&mut self) {
        let data = self.0.clone();
        let size = data.len() as u8;

        let mut cs = size;
        let mut buf = vec![0x06, 0x85];
        buf.push(size);
        for i in data {
            cs ^= i;
            buf.push(i);
        }
        buf.push(cs);
        self.0 = buf;
    }

    pub fn decreate_checksum(&mut self) -> Option<()> {
        let mut buf = self.0.clone();
        if buf.len() < 3 { return None; }
        buf.reverse();
        while buf.pop().unwrap() != 0x06 {} // get rid of preamble junk
        if buf.pop().unwrap() != 0x85 { return None; } // fail
        let len = buf.pop().unwrap();
        self.0.clear();
        for _ in 0..len {
            self.0.push(buf.pop().unwrap());
        }
        let mut cs = len as u8;
        for i in self.0.clone() {
            cs ^= i;
        }
        if cs == buf.pop().unwrap() { // Checksum passed
            return Some(());
        }
        else {
            return None;
        }
    }
}

mod test {
    #[test]
    fn message_checksum() {
        let mut message = super::Message::new();
        message.queue_byte(0xAA);
        message.queue_int(1025);
        message.create_checksum();
        message.decreate_checksum();
        assert_eq!(message.0, vec![0xAA, 4, 1]);
    }
    #[test]
    fn message_queue() {
        let mut message = super::Message::new();
        message.queue_int(1025);
        message.queue_byte(0xAA);
        assert_eq!(message.dequeue_int(), 1025);
        assert_eq!(message.dequeue_byte(), 0xAA);
    }
}

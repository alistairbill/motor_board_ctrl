use std::ffi::{OsStr, OsString};
use std::time::Duration;

use std::io::prelude::*;
use serial::{self, SerialPort, SystemPort};

pub struct Port (OsString);

impl Port {
    pub fn new<N: Into<OsString>>(name: N) -> Port {
        Port(name.into())
    }

    pub fn name(&self) -> &OsStr {
        &self.0
    }

    pub fn open(&self) -> serial::Result<SystemPort> {
        serial::open(&self.name())
    }
}

pub struct SerialConnection (serial::SystemPort);

impl SerialConnection {
    pub fn open(port: &Port) -> serial::Result<SerialConnection> {
        let mut serial = port.open()?;
        serial.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            Ok(())
        }).and_then(|_| {
            serial.set_timeout(Duration::from_millis(100))
        })?;

        let serial_connection = SerialConnection(serial);
        Ok(serial_connection)

    }

    pub fn send_data(&mut self, buf: &Message) -> serial::Result<()> {
        let &mut SerialConnection(ref mut serial) = self;
        try!(serial.write(&buf.0));
        Ok(())
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

    pub fn create(&mut self) {
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
}

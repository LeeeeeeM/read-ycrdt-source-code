use crate::*;

pub const BIT8: u8 = 0b10000000;
pub const BIT7: u8 = 0b01000000;

pub struct Encoder {
    pub buf: Vec<u8>
}

impl Encoder {
    pub fn new () -> Encoder {
        Encoder {
            buf: Vec::with_capacity(10000)
        }
    }
    pub fn with_capacity (size: usize) -> Encoder {
        Encoder {
            buf: Vec::with_capacity(size)
        }
    }
    pub fn write (&mut self, byte: u8) {
        self.buf.push(byte)
    }

    pub fn write_var_u32 (&mut self, mut num: u32) {
        for _ in 0..4 {
            self.buf.push(num as u8);
            num >>= 8
        }
    }

    pub fn as_bytes (&self) -> &[u8] {
        &self.buf[..]
    }

    pub fn write_var_buffer (&mut self, buffer: &[u8]) {
        self.write_var_u32(buffer.len() as u32);
        for elemen in buffer.iter() {
            self.write(*elemen);
        }
    }
}


pub struct Decoder <'a> {
    pub buf: &'a[u8],
    next: usize
}

impl <'a> Decoder <'a> {
    pub fn new (buf: &'a [u8]) -> Self {
        Self {
            buf,
            next: 0
        }
    }
    pub fn read (&mut self) -> u8 {
        let b = self.buf[self.next];
        self.next += 1;
        b
    }

    pub fn read_var_u32 (&mut self) -> u32 {
        self.read() as u32 | (self.read() as u32) << 8 | (self.read() as u32) << 16 | (self.read() as u32) << 24
    }

    pub fn read_var_buffer (&mut self) -> &[u8] {
        let len = self.read_var_u32();
        let slice = &self.buf[self.next..(self.next + len as usize)];
        self.next += len as usize;
        slice
    }
}

pub struct UpdateEncoder {
    pub rest_encoder: Encoder
}

impl UpdateEncoder {
    pub fn new () -> Self {
        Self {
            rest_encoder: Encoder::new()
        }
    }
    pub fn buffer (&self) -> &[u8] {
        self.rest_encoder.as_bytes()
    }

    pub fn write_var_buffer (&mut self, buffer: &[u8]) {
        self.rest_encoder.write_var_buffer(buffer);
    }

    pub fn write_left_id (&mut self, id: &ID) {
        self.rest_encoder.write_var_u32(id.client);
        self.rest_encoder.write_var_u32(id.clock);
    }
    pub fn write_right_id (&mut self, id: &ID) {
        self.rest_encoder.write_var_u32(id.client);
        self.rest_encoder.write_var_u32(id.clock);
    }
    pub fn write_client (&mut self, client: u32) {
        self.rest_encoder.write_var_u32(client);
    }
    pub fn write_info (&mut self, info: u8) {
        self.rest_encoder.write(info);
    }
    pub fn write_string (&mut self, string: &str) {
        let bytes = string.as_bytes();
        self.write_var_buffer(bytes);
    }
    pub fn write_char (&mut self, string: char) {
        self.rest_encoder.write(string as u8)
    }

}


pub struct UpdateDecoder<'a> {
    pub rest_decoder: Decoder<'a>
}

impl <'a> UpdateDecoder <'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            rest_decoder: Decoder::new(buf)
        }
    }
    pub fn read_left_id (&mut self) -> ID {
        ID {
            client: self.rest_decoder.read_var_u32(),
            clock: self.rest_decoder.read_var_u32()
        }
    }
    pub fn read_right_id (&mut self) -> ID {
        ID {
            client: self.rest_decoder.read_var_u32(),
            clock: self.rest_decoder.read_var_u32()
        }
    }
    pub fn read_client (&mut self) -> u32 {
        self.rest_decoder.read_var_u32()
    }
    pub fn read_info (&mut self) -> u8 {
        self.rest_decoder.read()
    }    
    pub fn read_var_buffer (&mut self) -> &[u8] {
        self.rest_decoder.read_var_buffer()
    }

    pub fn read_string (&mut self) -> String {
        let buf = self.read_var_buffer();
        String::from_utf8(buf.to_vec()).expect("malformatted string")
    }
    pub fn read_char (&mut self) -> char {
        self.rest_decoder.read() as char
    }
}

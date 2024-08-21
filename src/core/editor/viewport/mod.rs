use crate::core::buffer::Buffer;


pub struct Viewport {
    buffer: Option<Box<Buffer>>,
    top: u16,
    left: u16,
    width: u16,
    height: u16
}

impl Viewport {
    pub fn new(size: (u16, u16)) -> Self {
        Self {
            buffer: None,
            top: 0,
            left: 0,
            width: size.0,
            height: size.1,
        }
    }
    pub fn init_buffer(&mut self, buf: Box<Buffer>) {
        self.buffer = Some(Box::from(buf));
    }
}
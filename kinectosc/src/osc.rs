use std::io::{self, Write};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use nalgebra::{Point3, Translation3, UnitQuaternion};

pub struct Client {
    socket: UdpSocket,
    target: SocketAddr,
    buf: Vec<u8>,
}

impl Client {
    pub fn new<A: ToSocketAddrs>(bind: A, target: SocketAddr) -> io::Result<Self> {
        let socket = UdpSocket::bind(bind)?;
        let client = Client {
            socket,
            target,
            buf: Vec::with_capacity(1024),
        };
        Ok(client)
    }

    pub fn send<M: Message>(&mut self, message: M) -> io::Result<()> {
        let writer = OscPadWriter::new(&mut self.buf);
        message.encode(writer)?;
        self.socket.send_to(&self.buf, self.target)?;
        self.buf.clear();
        Ok(())
    }
}

pub trait Message {
    fn encode<W: Write>(&self, w: OscPadWriter<W>) -> io::Result<usize>;
}

pub struct PoseMessage {
    pub id: u32,
    pub is_valid: bool,
    pub wfd_rotation: UnitQuaternion<f64>,
    pub wfd_translation: Translation3<f64>,
    pub position: Point3<f64>,
    pub orientation: UnitQuaternion<f64>,
}

impl Message for PoseMessage {
    fn encode<W: Write>(&self, mut w: OscPadWriter<W>) -> io::Result<usize>
    {
        let mut len = 0;
        len += w.write_string("/Tracker/Pose")?;
        len += w.write_string(",iidddddddddddddd")?;

        len += w.write_int(self.id as i32)?;
        len += w.write_int(if self.is_valid { 1 } else { 0 })?;

        len += w.write_double(self.wfd_rotation.w)?;
        len += w.write_double(self.wfd_rotation.i)?;
        len += w.write_double(self.wfd_rotation.j)?;
        len += w.write_double(self.wfd_rotation.k)?;

        len += w.write_double(self.wfd_translation.x)?;
        len += w.write_double(self.wfd_translation.y)?;
        len += w.write_double(self.wfd_translation.z)?;

        len += w.write_double(self.position.x)?;
        len += w.write_double(self.position.y)?;
        len += w.write_double(self.position.z)?;

        len += w.write_double(self.orientation.w)?;
        len += w.write_double(self.orientation.i)?;
        len += w.write_double(self.orientation.j)?;
        len += w.write_double(self.orientation.k)?;

        Ok(len)
    }
}

pub struct OscPadWriter<W> {
    inner: W,
}
impl<W> OscPadWriter<W>
where
    W: Write,
{
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    pub fn write_string(&mut self, s: &str) -> io::Result<usize> {
        let bytes = s.as_bytes();
        let pad_len = 0b100 - (bytes.len() & 0b11);
        let pad = &[0u8; 4][..pad_len];
        self.inner.write_all(bytes)?;
        self.inner.write_all(pad)?;
        Ok(bytes.len() + pad_len)
    }

    pub fn write_int(&mut self, u: i32) -> io::Result<usize> {
        self.inner.write_all(&u.to_be_bytes())?;
        Ok(4)
    }

    pub fn write_double(&mut self, d: f64) -> io::Result<usize> {
        self.inner.write_all(&d.to_be_bytes())?;
        Ok(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_string() {
        let mut buf = Vec::new();
        let mut w = OscPadWriter::new(&mut buf);
        w.write_string("abc").unwrap();
        assert_eq!(vec![0x61, 0x62, 0x63, 0x00], buf);
    }
}

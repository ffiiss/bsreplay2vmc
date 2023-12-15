use crate::osc;
use crate::pose;
use std::*;

pub struct Sender {
    buffer: Vec<u8>,
    socket: net::UdpSocket,
}

fn transform_args(buf: &mut Vec<u8>, transform: &pose::Transform) {
    for v in transform.pos.iter() {
        osc::f32(buf, *v);
    }
    for v in transform.rot.iter() {
        osc::f32(buf, *v);
    }
}

fn transform_message(buf: &mut Vec<u8>, addr: &[u8], serial: &[u8], transform: &pose::Transform) {
    osc::header(buf, addr, b"sfffffff");
    osc::str(buf, serial);
    transform_args(buf, transform);
}

impl Sender {
    pub fn new() -> io::Result<Self> {
        let socket = net::UdpSocket::bind("127.0.0.1:0")?;
        socket.connect("127.0.0.1:39540")?;
        Ok(Sender {
            buffer: Vec::new(),
            socket: socket,
        })
    }

    pub fn camera(&mut self, transform: &pose::Transform, fov: f32) -> io::Result<()> {
        self.buffer.clear();
        osc::header(&mut self.buffer, b"/VMC/Ext/Cam", b"sffffffff");
        osc::str(&mut self.buffer, b"3");
        transform_args(&mut self.buffer, transform);
        osc::f32(&mut self.buffer, fov);
        self.socket.send(&self.buffer)?;
        Ok(())
    }

    pub fn keyframe(&mut self, keyframe: &pose::Keyframe) -> io::Result<()> {
        self.buffer.clear();
        osc::bundle_header(&mut self.buffer, 1);
        let offset = osc::bundle_content_begin(&mut self.buffer);
        transform_message(&mut self.buffer, b"/VMC/Ext/Hmd/Pos", b"0", &keyframe.head);
        osc::bundle_content_end(&mut self.buffer, offset);
        let offset = osc::bundle_content_begin(&mut self.buffer);
        transform_message(&mut self.buffer, b"/VMC/Ext/Con/Pos", b"1", &keyframe.left);
        osc::bundle_content_end(&mut self.buffer, offset);
        let offset = osc::bundle_content_begin(&mut self.buffer);
        transform_message(&mut self.buffer, b"/VMC/Ext/Con/Pos", b"2", &keyframe.right);
        osc::bundle_content_end(&mut self.buffer, offset);
        self.socket.send(&self.buffer)?;
        Ok(())
    }

    pub fn calib_ready(&mut self) -> io::Result<()> {
        self.buffer.clear();
        osc::header(&mut self.buffer, b"/VMC/Ext/Set/Calib/Ready", b"");
        self.socket.send(&self.buffer)?;
        Ok(())
    }

    pub fn calib_exec(&mut self, n: i32) -> io::Result<()> {
        self.buffer.clear();
        osc::header(&mut self.buffer, b"/VMC/Ext/Set/Calib/Exec", b"i");
        osc::i32(&mut self.buffer, n);
        self.socket.send(&self.buffer)?;
        Ok(())
    }
}

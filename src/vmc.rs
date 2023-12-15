use crate::osc;
use crate::pose;
use std::*;

pub struct Sender {
    buffer: Vec<u8>,
    socket: net::UdpSocket,
}

fn write_transform_args(buf: &mut Vec<u8>, transform: &pose::Transform) {
    for v in transform.pos.iter() {
        osc::write_f32(buf, *v);
    }
    for v in transform.rot.iter() {
        osc::write_f32(buf, *v);
    }
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
        osc::write_header(&mut self.buffer, b"/VMC/Ext/Cam", b"sffffffff");
        osc::write_str(&mut self.buffer, b"3");
        write_transform_args(&mut self.buffer, transform);
        osc::write_f32(&mut self.buffer, fov);
        self.socket.send(&self.buffer)?;
        Ok(())
    }

    pub fn keyframe(&mut self, keyframe: &pose::Keyframe) -> io::Result<()> {
        let data = [
            (b"/VMC/Ext/Hmd/Pos", b"0", &keyframe.head),
            (b"/VMC/Ext/Con/Pos", b"1", &keyframe.left),
            (b"/VMC/Ext/Con/Pos", b"2", &keyframe.right),
        ];
        self.buffer.clear();
        osc::write_bundle_header(&mut self.buffer, 1);
        for (addr, serial, transform) in data {
            let offset = osc::write_bundle_content_begin(&mut self.buffer);
            osc::write_header(&mut self.buffer, addr, b"sfffffff");
            osc::write_str(&mut self.buffer, serial);
            write_transform_args(&mut self.buffer, transform);
            osc::write_bundle_content_end(&mut self.buffer, offset);
        }
        self.socket.send(&self.buffer)?;
        Ok(())
    }

    pub fn calib_ready(&mut self) -> io::Result<()> {
        self.buffer.clear();
        osc::write_header(&mut self.buffer, b"/VMC/Ext/Set/Calib/Ready", b"");
        self.socket.send(&self.buffer)?;
        Ok(())
    }

    pub fn calib_exec(&mut self, n: i32) -> io::Result<()> {
        self.buffer.clear();
        osc::write_header(&mut self.buffer, b"/VMC/Ext/Set/Calib/Exec", b"i");
        osc::write_i32(&mut self.buffer, n);
        self.socket.send(&self.buffer)?;
        Ok(())
    }
}

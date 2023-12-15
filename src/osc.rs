use std::*;

pub fn write_str(buf: &mut Vec<u8>, s: &[u8]) {
    buf.extend_from_slice(s);
    buf.resize((buf.len() + 4) & !0b11, 0);
}

pub fn write_i32(buf: &mut Vec<u8>, i: i32) {
    buf.extend_from_slice(&i.to_be_bytes());
}

pub fn write_f32(buf: &mut Vec<u8>, f: f32) {
    buf.extend_from_slice(&f.to_be_bytes());
}

pub fn write_header(buf: &mut Vec<u8>, addr: &[u8], tags: &[u8]) {
    write_str(buf, addr);
    buf.push(b',');
    write_str(buf, tags);
}

pub fn write_bundle_header(buf: &mut Vec<u8>, time: u64) {
    write_str(buf, b"#bundle");
    buf.extend_from_slice(&time.to_be_bytes());
}

pub fn write_bundle_content_begin(buf: &mut Vec<u8>) -> usize {
    write_i32(buf, -1); // dummy value.
    buf.len()
}

pub fn write_bundle_content_end(buf: &mut Vec<u8>, offset: usize) {
    let len = (buf.len() - offset) as i32;
    buf[offset - 4..offset].copy_from_slice(&len.to_be_bytes());
}

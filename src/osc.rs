use std::*;

pub fn str(buf: &mut Vec<u8>, s: &[u8]) {
    buf.extend(s);
    for _ in (buf.len() % 4)..4 {
        buf.push(0);
    }
}

pub fn i32(buf: &mut Vec<u8>, i: i32) {
    buf.extend(i.to_be_bytes());
}

pub fn f32(buf: &mut Vec<u8>, f: f32) {
    buf.extend(f.to_be_bytes());
}

pub fn header(buf: &mut Vec<u8>, addr: &[u8], tags: &[u8]) {
    str(buf, addr);
    buf.push(b',');
    str(buf, tags);
}

pub fn bundle_header(buf: &mut Vec<u8>, time: u64) {
    str(buf, b"#bundle");
    buf.extend(time.to_be_bytes());
}

pub fn bundle_content_begin(buf: &mut Vec<u8>) -> usize {
    buf.extend(0xdeadc0deu32.to_be_bytes());
    buf.len()
}

pub fn bundle_content_end(buf: &mut Vec<u8>, offset: usize) {
    let len = (buf.len() - offset) as i32;
    buf[offset - 4..offset].copy_from_slice(&len.to_be_bytes());
}

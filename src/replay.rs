use crate::pose;
use std::io::Seek;
use std::*;

fn read_i32(r: &mut impl io::Read) -> io::Result<i32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

fn read_f32(r: &mut impl io::Read) -> io::Result<f32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

fn read_vec<const N: usize>(r: &mut impl io::Read) -> io::Result<[f32; N]> {
    let mut xs = [0.0; N];
    for x in xs.iter_mut() {
        *x = read_f32(r)?;
    }
    Ok(xs)
}

fn read_transform(r: &mut impl io::Read) -> io::Result<pose::Transform> {
    let pos = read_vec(r)?;
    let rot = read_vec(r)?;
    Ok(pose::Transform { pos, rot })
}

pub fn load(r: &mut impl io::BufRead) -> Result<Vec<pose::Keyframe>, Box<dyn error::Error>> {
    let mut magic = [0; 28];
    r.read_exact(&mut magic)?;
    if magic[0..4] == [93, 0, 0, 128] {
        return Err("Legacy format".into());
    }
    let mut decompressed: Vec<u8> = Vec::new();
    lzma_rs::lzma_decompress(r, &mut decompressed)?;

    let mut cursor = io::Cursor::new(decompressed);
    let _offset_meta = read_i32(&mut cursor)?;
    let offset_pose = read_i32(&mut cursor)?;
    cursor.seek(io::SeekFrom::Start(offset_pose as u64))?;
    let size = read_i32(&mut cursor)?;
    let mut keyframes = Vec::new();
    for _ in 0..size {
        let head = read_transform(&mut cursor)?;
        let left = read_transform(&mut cursor)?;
        let right = read_transform(&mut cursor)?;
        let _fps = read_i32(&mut cursor)?;
        let time = read_f32(&mut cursor)?;
        keyframes.push(pose::Keyframe {
            head,
            left,
            right,
            time,
        });
    }
    Ok(keyframes)
}

mod osc;
mod pose;
mod replay;
mod vmc;
use std::io::Write;
use std::*;

fn median(it: impl iter::Iterator<Item = f32>) -> f32 {
    let mut xs: Vec<f32> = it.collect();
    assert!(xs.len() > 0);
    let ml = (xs.len() - 1) / 2;
    let mh = (xs.len() - 0) / 2;
    xs.select_nth_unstable_by(mh, |x, y| x.partial_cmp(y).unwrap());
    0.5 * (xs[ml] + xs[mh])
}

fn calibrate(sender: &mut vmc::Sender, keyframes: &[pose::Keyframe]) -> io::Result<()> {
    use std::f32::consts::FRAC_1_SQRT_2;
    let duration = time::Duration::from_millis(250);

    let head_height = median(keyframes.iter().map(|p| p.head.pos[1]));
    let hand_height = head_height - 0.2;
    let hand_width = head_height / 2.0;
    let calib_keyframe = pose::Keyframe {
        head: pose::Transform {
            pos: [0.0, head_height, 0.1],
            rot: [0.0, 0.0, 0.0, 1.0],
        },
        left: pose::Transform {
            pos: [-hand_width, hand_height, 0.0],
            rot: [0.0, 0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2],
        },
        right: pose::Transform {
            pos: [hand_width, hand_height, 0.0],
            rot: [0.0, 0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2],
        },
        time: 0.0,
    };
    let camera = pose::Transform {
        pos: [0.0, (3.0 / 4.0) * head_height, 2.0 * head_height],
        rot: [0.0, f32::sqrt(1.0 - 0.04 * 0.04), -0.04, 0.0],
    };

    sender.camera(&camera, 45.0)?;
    thread::sleep(duration);
    sender.keyframe(&calib_keyframe)?;
    thread::sleep(duration);
    sender.calib_ready()?;
    thread::sleep(duration);
    sender.keyframe(&calib_keyframe)?;
    thread::sleep(duration);
    sender.calib_exec(0)?;
    thread::sleep(duration);

    Ok(())
}

fn play(sender: &mut vmc::Sender, keyframes: &[pose::Keyframe]) -> io::Result<()> {
    let t0 = time::Instant::now();
    for keyframe in keyframes.iter() {
        let t = t0.elapsed().as_secs_f32();
        let dt = f32::max(keyframe.time - t, 0.0);
        thread::sleep(time::Duration::from_secs_f32(dt));
        sender.keyframe(keyframe)?;
        print!("\r{:7.2}s", keyframe.time);
        io::stdout().flush()?;
    }
    println!();

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut sender = vmc::Sender::new()?;

    for arg in env::args().skip(1) {
        println!("Loading \"{}\".", &arg);
        let keyframes = replay::load(&mut io::BufReader::new(fs::File::open(&arg)?))?;
        calibrate(&mut sender, &keyframes)?;
        play(&mut sender, &keyframes)?;
    }

    Ok(())
}

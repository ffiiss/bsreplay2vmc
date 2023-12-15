#[derive(Debug, Clone)]
pub struct Transform {
    pub pos: [f32; 3],
    pub rot: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub head: Transform,
    pub left: Transform,
    pub right: Transform,
    pub time: f32,
}

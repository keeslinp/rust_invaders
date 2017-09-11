pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Velocity {
        Velocity {
            dx,
            dy
        }
    }
}

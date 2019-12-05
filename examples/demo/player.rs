pub struct Player {
    pos: (f32, f32),
    speed: f32,
}

impl Player {
    pub fn new(speed: f32) -> Self {
        Self {
            pos: (0.0, 0.0),
            speed,
        }
    }
    pub fn move_to(&mut self, pos: (i32, i32)) {
        self.pos = (pos.0 as f32, pos.1 as f32);
    }
    pub fn move_left(&mut self) {
        self.pos.0 -= self.speed;
    }
    pub fn move_right(&mut self) {
        self.pos.0 += self.speed;
    }
    pub fn move_up(&mut self) {
        self.pos.1 -= self.speed;
    }
    pub fn move_down(&mut self) {
        self.pos.1 += self.speed;
    }
    pub fn pos(&self) -> (i32, i32) {
        (self.pos.0 as i32, self.pos.1 as i32)
    }
    pub fn left(&self) -> (i32, i32) {
        (self.pos.0 as i32 - 1, self.pos.1 as i32)
    }
    pub fn right(&self) -> (i32, i32) {
        (self.pos.0 as i32 + 1, self.pos.1 as i32)
    }
    pub fn up(&self) -> (i32, i32) {
        (self.pos.0 as i32, self.pos.1 as i32 - 1)
    }
    pub fn down(&self) -> (i32, i32) {
        (self.pos.0 as i32, self.pos.1 as i32 + 1)
    }
}

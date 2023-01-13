use doryen_rs::{Color, DoryenApi, ScanCode};

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
    pub fn move_from_input(&self, api: &mut dyn DoryenApi) -> (i32, i32) {
        let input = api.input();
        let mut mov = (0, 0);
        if input.key(ScanCode::Left) || input.key(ScanCode::A) {
            mov.0 = -1;
        } else if input.key(ScanCode::Right) || input.key(ScanCode::D) {
            mov.0 = 1;
        }
        if input.key(ScanCode::Up) || input.key(ScanCode::W) {
            mov.1 = -1;
        } else if input.key(ScanCode::Down) || input.key(ScanCode::S) {
            mov.1 = 1;
        }
        mov
    }
    pub fn move_to(&mut self, pos: (i32, i32)) {
        self.pos = (pos.0 as f32, pos.1 as f32);
    }
    pub fn move_by(&mut self, mov: (i32, i32), coef: f32) -> bool {
        let oldx = self.pos.0 as i32;
        let oldy = self.pos.1 as i32;
        self.pos.0 += self.speed * mov.0 as f32 * coef;
        self.pos.1 += self.speed * mov.1 as f32 * coef;
        oldx == self.pos.0 as i32 && oldy == self.pos.1 as i32
    }
    pub fn next_pos(&self, mov: (i32, i32)) -> (i32, i32) {
        (self.pos.0 as i32 + mov.0, self.pos.1 as i32 + mov.1)
    }
    pub fn pos(&self) -> (i32, i32) {
        (self.pos.0 as i32, self.pos.1 as i32)
    }
    pub fn render(&self, api: &mut dyn DoryenApi, light: Color) {
        let con = api.con();
        let pos = self.pos();
        con.ascii(pos.0, pos.1, '@' as u16);
        con.fore(pos.0, pos.1, light);
    }
}

use doryen_rs::{Color, DoryenApi, KeyEvent};

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
        if input.key(KeyEvent::ArrowLeft) || input.key(KeyEvent::Key('a')) {
            mov.0 = -1;
        } else if input.key(KeyEvent::ArrowRight) || input.key(KeyEvent::Key('d')) {
            mov.0 = 1;
        }
        if input.key(KeyEvent::ArrowUp) || input.key(KeyEvent::Key('w')) {
            mov.1 = -1;
        } else if input.key(KeyEvent::ArrowDown) || input.key(KeyEvent::Key('s')) {
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

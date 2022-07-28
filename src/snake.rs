use rand::{thread_rng, Rng};
use std::ops::Add;
use crate::{
    WHITE,
    GREEN,
    draw_rectangle,
    Texture2D,
    DrawTextureParams,
    clear_background,
    draw_texture_ex,
    Vec2,
    draw_circle,
    DARKGRAY,
    GRAY,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Pos(pub i32, pub i32);
pub fn pos(x: i32, y: i32) -> Pos {
    Pos(x, y)
}
impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Self) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

pub const DIRECTION_RIGHT: Pos = Pos(1, 0);
pub const DIRECTION_LEFT: Pos = Pos(-1, 0);
pub const DIRECTION_UP: Pos = Pos(0, -1);
pub const DIRECTION_DOWN: Pos = Pos(0, 1);

#[derive(Debug)]
pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: Vec<Pos>,
    food: Pos,
    tilesize: usize,
    add_next_tick: Option<Pos>,
    segment_texture: Texture2D,
    segment_draw_params: DrawTextureParams,
    food_texture: Option<Texture2D>,
    food_textures: Vec<Texture2D>,
    head_texture: Texture2D,
    pub alive: bool,
    pub direction: Pos,
}

impl SnakeGame {
    pub fn new(
        width: usize,
        height: usize,
        tilesize: usize,
        texture: Texture2D,
        food_textures: Vec<Texture2D>,
        head_texture: Texture2D,
        ) -> Self 
    {
        let mut rng = thread_rng();

        let draw_params = DrawTextureParams {
            dest_size: Some(Vec2::new(tilesize as f32, tilesize as f32)),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        let mut res = Self {
            width,
            height,
            snake: vec![
                pos((width - 2) as i32, (height / 2) as i32)],
            food: Pos(0, 0),
            direction: DIRECTION_LEFT,
            tilesize,
            alive: true,
            add_next_tick: None,
            segment_texture: texture,
            segment_draw_params: draw_params,
            food_textures: food_textures,
            food_texture: None,
            head_texture,
        };
        res.gen_food();
        res
    }

    pub fn respawn(&mut self) {
        self.snake = vec![
            pos((self.width - 2) as i32,
            (self.height / 2) as i32)];
        self.gen_food();
        self.direction = DIRECTION_LEFT;
        self.alive = true;
        self.add_next_tick = None;
    }

    pub fn head(&self) -> &Pos {
        &self.snake[0]
    }

    pub fn empties(&self) -> Vec<Pos> {
        let mut empties = vec![];
        for i in 0..self.width {
            for c in 0..self.height {
                if !self.snake.contains(&Pos(i as i32, c as i32)) {
                    empties.push(Pos(i as i32, c as i32));
                }
            }
        }
        empties
    }

    pub fn gen_food(&mut self) {
        let mut rng = thread_rng();
        let empties = self.empties();
        self.food = empties[rng.gen_range(0..empties.len())];
        self.food_texture = Some(self.food_textures[rng.gen_range(0..self.food_textures.len())].clone());
    }

    pub fn input(&mut self, direction: Pos) {
        if self.direction + direction == Pos(0, 0) {
            return;
        }
        self.direction = direction;
    }

    pub fn tail(&self) -> Vec<&Pos> {
        let mut tail = vec![];
        for i in 1..self.snake.len() {
            tail.push(&self.snake[i]);
        }
        tail
    }

    pub fn dead(&self) -> bool {
        let head = self.head();
        if head.0 < 0 || head.0 >= self.width as i32 {
            return true;
        }
        if head.1 < 0 || head.1 >= self.height as i32 {
            return true;
        }
        for segment in self.tail().iter() {
            if head == segment.clone() {
                return true;
            }
        }
        return false;
    }

    pub fn tick(&mut self) {
        if self.alive {
            let head = self.head();
            self.snake.insert(0, Pos (
                head.0 + self.direction.0,
                head.1 + self.direction.1)
                );
            self.snake.pop();
        }
        if let Some(to_add) = self.add_next_tick {
            self.snake.push(to_add);
            self.add_next_tick = None;
        }
        if self.dead() {
            self.respawn();
        }

    }

    pub fn draw(&self) {
        self.draw_bg();

        for (idx, segment) in self.snake.iter().enumerate() {
            let pos: (f32, f32) = ((segment.0 * self.tilesize as i32) as f32, (segment.1 * self.tilesize as i32) as f32);
            if idx == 0 {
                draw_texture_ex(self.head_texture, pos.0, pos.1, WHITE, self.segment_draw_params.clone());
            } else {
                draw_texture_ex(self.segment_texture, pos.0, pos.1, WHITE, self.segment_draw_params.clone());
            }
        }

        match self.food_texture {
            Some(_) => {
                draw_texture_ex(
                    self.food_texture.unwrap(),
                    (self.food.0 * self.tilesize as i32) as f32,
                    (self.food.1 * self.tilesize as i32) as f32,
                    WHITE,
                    self.segment_draw_params.clone()
                );
            },
            None => {
                draw_rectangle(
                    (self.food.0 * self.tilesize as i32) as f32,
                    (self.food.1 * self.tilesize as i32) as f32,
                    self.tilesize as f32,
                    self.tilesize as f32,
                    GREEN
                );
            }
        }
    }

    pub fn draw_bg(&self) {
        clear_background(DARKGRAY);
        for i in 0..self.width {
            for c in 0..self.height {
                draw_circle(
                    (i * self.tilesize) as f32 + self.tilesize as f32 * 0.5,
                    (c * self.tilesize) as f32 + self.tilesize as f32 * 0.5,
                    self.tilesize as f32 * 0.1,
                    GRAY
                    );
            }
        }
    }

    pub fn eat(&mut self) {
        let head = self.head();
        if head.clone() == self.food {
            self.add_next_tick = Some(
                self.snake[self.snake.len() - 1]
            );
            self.gen_food();
        }
    }
}

use ggez::{Context, GameResult, ContextBuilder};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, Drawable};
use ggez::mint::{Point2, Vector2};
use ggez::input::keyboard::{self, KeyCode};
use rand::{self, thread_rng, Rng};

const PADDING: f32 = 40.0;
const MIDDLE_LINE_WIDTH: f32 = 2.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 100.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let dt = ctx.time.delta().as_secs_f32();
    let screen_h = ctx.gfx.drawable_size().1;

    if ctx.keyboard.is_key_pressed(keycode) {
        pos.y -= y_dir * PLAYER_SPEED * dt
    }

    clamp(&mut pos.y, 0.0, screen_h-RACKET_HEIGHT);
}

fn randomize_vec(vec: &mut Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y
    };
}

struct MainState {
    player_1_pos: Point2<f32>,
    player_2_pos: Point2<f32>,
    ball_pos: Point2<f32>,
    ball_vel: Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let (screen_w_half, screen_h_half) = (screen_w*0.5, screen_h*0.5);
        let mut ball_vel = Vector2{x: 0.0, y: 0.0};
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_pos: Point2{x: PADDING, y: screen_h_half-RACKET_HEIGHT_HALF},
            player_2_pos: Point2{x: screen_w-RACKET_WIDTH-PADDING, y: screen_h_half-RACKET_HEIGHT_HALF},
            ball_pos: Point2{x: screen_w_half-BALL_SIZE, y: screen_h_half-BALL_SIZE},
            ball_vel: ball_vel,
            player_1_score: 0,
            player_2_score: 0,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

        move_racket(&mut self.player_1_pos, KeyCode::W, 1.0, ctx);
        move_racket(&mut self.player_1_pos, KeyCode::S, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Up, 1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Down, -1.0, ctx);

        // self.ball_pos += self.ball_vel * dt;
        self.ball_pos.y += self.ball_vel.y * dt;
        self.ball_pos.x += self.ball_vel.x * dt;

        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        }
        if self.ball_pos.x > screen_w {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
        }

        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.y > screen_h - BALL_SIZE {
            self.ball_pos.y = screen_h - BALL_SIZE;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }

        let intersects_player_1 = 
            self.ball_pos.x - BALL_SIZE_HALF < self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_1 {
            self.ball_vel.x = self.ball_vel.x.abs();
        }
        
        let intersects_player_2 = 
            self.ball_pos.x - BALL_SIZE_HALF < self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - RACKET_HEIGHT_HALF;
            
        if intersects_player_2 {
            self.ball_vel.x = -self.ball_vel.x.abs();
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let screen_w_half = screen_w*0.5;

        // Draw a filled rectangle mesh.

        let rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest::<Point2<f32>>(self.player_1_pos)
                .scale(rect.size())
                .color(Color::WHITE),
        );

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest::<Point2<f32>>(self.player_2_pos)
                .scale(rect.size())
                .color(Color::WHITE),
        );

        let ball = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest::<Point2<f32>>(self.ball_pos)
                .scale(ball.size())
                .color(Color::WHITE),
        );

        let middle_rect = graphics::Rect::new(-MIDDLE_LINE_WIDTH*0.5, 0.0, MIDDLE_LINE_WIDTH, screen_h);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(Point2{x:screen_w_half, y: 0.0})
                .scale(middle_rect.size())
                .color(Color::WHITE),
        );

        let score_text = graphics::Text::new(format!("{}        {}", self.player_1_score, self.player_2_score));
        let text_rect = score_text.dimensions(ctx).unwrap();
        let score_pos = Point2{x: screen_w_half - text_rect.w * 0.5, y: 40.0 - text_rect.h * 0.5};

        canvas.draw(&score_text, graphics::DrawParam::default().dest(score_pos));


        canvas.finish(ctx)
        // Ok(())
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Pong_0", "Khemio")
        .build()
        .expect("Couldn't create ggez context");

    ctx.gfx.set_window_title("PONG");

    let state = MainState::new(&mut ctx);

    event::run( ctx, event_loop, state);
}

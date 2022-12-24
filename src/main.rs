use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{self, Color};
use ggez::input::keyboard;
use ggez::{Context, ContextBuilder, GameResult};
use rand::{thread_rng, Rng};

const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 5.0;
const PLAYER_WIDTH: f32 = 10.0;
const PLAYER_HEIGHT: f32 = 100.0;
const PLAYER_SPEED: f32 = 10.0;
const PLAYER_OFFSET: f32 = 50.0;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("pong", "Dominic")
        .window_setup(WindowSetup::default().title("Pong"))
        .window_mode(
            WindowMode::default()
                .dimensions(800.0, 600.0)
                .resizable(true),
        )
        .build()
        .expect("could not create ggez context");

    let pong = Pong::new(&mut ctx)?;

    event::run(ctx, event_loop, pong)
}

struct Pong {
    ball: Ball,
    players: [Player; 2],
    score: [u32; 2],
}

struct Player {
    x: f32,
    y: f32,
    mesh: graphics::Mesh,
}

impl Player {
    fn new(x: f32, y: f32, ctx: &mut Context) -> Player {
        Player {
            x,
            y,
            mesh: graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, PLAYER_WIDTH, PLAYER_HEIGHT),
                Color::WHITE,
            )
            .unwrap(),
        }
    }

    fn update_from_move(&mut self, ctx: &mut Context, move_direction: PlayerMoveDirection) {
        match move_direction {
            PlayerMoveDirection::Up => self.y -= PLAYER_SPEED,
            PlayerMoveDirection::Down => self.y += PLAYER_SPEED,
        }

        if self.y < 0.0 {
            self.y = 0.0;
        } else if self.y + PLAYER_HEIGHT > ctx.gfx.drawable_size().1 {
            self.y = ctx.gfx.drawable_size().1 - PLAYER_HEIGHT;
        }
    }
}

enum PlayerMoveDirection {
    Up,
    Down,
}

struct Ball {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
    mesh: graphics::Mesh,
}

impl Ball {
    fn new(x: f32, y: f32, ctx: &mut Context) -> Ball {
        Ball {
            x,
            y,
            x_speed: BALL_SPEED,
            y_speed: BALL_SPEED,
            mesh: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                vec2(0.0, 0.0),
                BALL_RADIUS,
                2.0,
                Color::WHITE,
            )
            .unwrap(),
        }
    }

    fn update(&mut self, ctx: &mut Context, left_player: &Player, right_player: &Player) {
        self.x += self.x_speed;
        self.y += self.y_speed;

        if self.x - BALL_RADIUS < left_player.x + PLAYER_WIDTH
            && self.x - BALL_RADIUS > left_player.x
            && self.y + BALL_RADIUS > left_player.y
            && self.y - BALL_RADIUS < left_player.y + PLAYER_HEIGHT
        {
            self.x = left_player.x + 10.0 + BALL_RADIUS;
            self.x_speed = -self.x_speed;
        } else if self.x + BALL_RADIUS > right_player.x
            && self.x + BALL_RADIUS < right_player.x + PLAYER_WIDTH
            && self.y + BALL_RADIUS > right_player.y
            && self.y - BALL_RADIUS < right_player.y + PLAYER_HEIGHT
        {
            self.x = right_player.x - BALL_RADIUS;
            self.x_speed = -self.x_speed;
        }

        if self.y - BALL_RADIUS < 0.0 {
            self.y = BALL_RADIUS;
            self.y_speed = -self.y_speed;
        } else if self.y + BALL_RADIUS > ctx.gfx.drawable_size().1 {
            self.y = ctx.gfx.drawable_size().1 - BALL_RADIUS;
            self.y_speed = -self.y_speed;
        }
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.x = ctx.gfx.drawable_size().0 / 2.0;
        self.y = ctx.gfx.drawable_size().1 / 2.0;

        let mut rng = thread_rng();
        let direction = rng.gen_range(std::f32::consts::PI / 4.0..std::f32::consts::PI * 3.0 / 4.0)
            + if rng.gen() { std::f32::consts::PI } else { 0.0 };
        self.x_speed = BALL_SPEED * direction.cos();
        self.y_speed = BALL_SPEED * direction.sin();
    }

    fn left_player_goal(&self, _ctx: &mut Context) -> bool {
        self.x - BALL_RADIUS < 0.0
    }

    fn right_player_goal(&self, _ctx: &mut Context) -> bool {
        self.x + BALL_RADIUS > _ctx.gfx.drawable_size().0
    }
}

impl Pong {
    pub fn new(_ctx: &mut Context) -> GameResult<Pong> {
        let ball = Ball::new(
            _ctx.gfx.drawable_size().0 / 2.0,
            _ctx.gfx.drawable_size().1 / 2.0,
            _ctx,
        );

        let left_player = Player::new(PLAYER_OFFSET, 10.0, _ctx);
        let right_player = Player::new(_ctx.gfx.drawable_size().0 - PLAYER_OFFSET, 10.0, _ctx);

        Ok(Pong {
            ball,
            players: [left_player, right_player],
            score: [0, 0],
        })
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_pressed(keyboard::KeyCode::W) {
            self.players[0].update_from_move(ctx, PlayerMoveDirection::Up);
        } else if ctx.keyboard.is_key_pressed(keyboard::KeyCode::S) {
            self.players[0].update_from_move(ctx, PlayerMoveDirection::Down);
        }

        if ctx.keyboard.is_key_pressed(keyboard::KeyCode::Up) {
            self.players[1].update_from_move(ctx, PlayerMoveDirection::Up);
        } else if ctx.keyboard.is_key_pressed(keyboard::KeyCode::Down) {
            self.players[1].update_from_move(ctx, PlayerMoveDirection::Down);
        }

        self.ball.update(ctx, &self.players[0], &self.players[1]);

        if self.ball.left_player_goal(ctx) {
            self.score[1] += 1;
            self.ball.reset(ctx);
        } else if self.ball.right_player_goal(ctx) {
            self.score[0] += 1;
            self.ball.reset(ctx);
        }

        let title = format!("Pong - {} : {}", self.score[0], self.score[1]);
        ctx.gfx.set_window_title(&title);

        Ok(())
    }
}

impl EventHandler for Pong {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update(_ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&self.ball.mesh, Vec2::new(self.ball.x, self.ball.y));
        canvas.draw(
            &self.players[0].mesh,
            Vec2::new(self.players[0].x, self.players[0].y),
        );
        canvas.draw(
            &self.players[1].mesh,
            Vec2::new(self.players[1].x, self.players[1].y),
        );
        canvas.finish(ctx)
    }
}

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    End,
}
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,  //世界坐标
            y,
            velocity: 2.0,
        }
    }

    fn render(&self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity <= 1.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        // self.velocity = -2.0;
        self.velocity = 0.0;
        self.y -= 2;
    }
}

struct State {
    mode: GameMode,
    frame_time: f32,
    player: Player,
    obstacle:Obstacle,
    score:i32,
}

impl State {
    fn new() -> State {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle:Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quit(),
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        // let the player move too fast to be a flasher
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        //the user action must be trigger immediately
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        // self.player.gravity_and_move();
        ctx.print(0, 0, "Press SPace to Flap");
        ctx.print(0,1,&format!("Score: {}",self.score));
        self.obstacle.render(ctx, self.player.x);
        if self.player.x >self.obstacle.x{
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x+SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player){
            self.mode = GameMode::End;
        }
        // self.mode = GameMode::End;
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered( 5,"You are dead");
        ctx.print_centered(6, format!("You earned {} points",self.score));
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

struct Obstacle {
    x: i32, //世界空间
    gap_y: i32,
    size: i32,  // 洞口尺寸大小
}

impl Obstacle {
    /**
     * x: the x position of the obstacle
     * score: player play game gain score
     */
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, score),
        }
    }

    fn render(&self,ctx:&mut BTerm,player_x: i32){
        let screen_x = self.x - player_x;   // 屏幕空间
        let half_size = self.size / 2;
        for y in 0..self.gap_y-half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        for y in self.gap_y+half_size..SCREEN_HEIGHT{
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&mut self,player:&Player)->bool{
        // 获取空隙的一半
        let half_size = self.size /2;
        let dose_x_match = self.x == player.x;
        let player_above_gap = player.y < (self.gap_y - half_size);
        let player_blow_gap = player.y > (self.gap_y + half_size);
        dose_x_match && (player_above_gap||player_blow_gap)
    }
    
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon!")
        .build()?;
    main_loop(context, State::new())
}

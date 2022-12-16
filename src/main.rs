extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng;
use rand::thread_rng;

#[derive(Copy, Clone ,PartialEq, Eq)]
pub enum Player{
    Player1,
    Computer,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Area{
    NOTHING,
    UNKNOWN,
    SHIP,
    DESTROYEDSHIP,
    MISS,
    BLOCK,
}

pub struct Game {
    gl: GlGraphics,
    rows: u32,
    cols: u32,
    board: Board,
    square_width: u32,
    result: Player,
    turn: bool,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        const DARK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |context, graphics| {
            graphics::clear(DARK, graphics);
        });

        self.board.render(args);
        
                
    }

    fn update(&mut self, args: &UpdateArgs) -> bool {

        if self.turn{
            if !self.playerTurn(){
                self.result=Player::Player1;
                return false
            }
            if !self.computerTurn(){
                self.result=Player::Computer;
                return false
            }
            self.turn = false;
        }
        true
    }
    fn computerTurn(&mut self) -> bool{
        let mut r = thread_rng();
        let mut x = r.gen_range(0, 9);
        let mut y = r.gen_range(0, 9);
        self.board.shot(x,y,Player::Player1);
        return self.board.playerAlive(Player::Player1);
    }
    fn playerTurn(&mut self) -> bool{
        // pobierz jakoś x i y z getPointer :( )
        println!("Wybrałeś pole: {}:{}", self.board.pointerx,self.board.pointery);
        self.board.shot(self.board.pointerx as usize,self.board.pointery as usize,Player::Computer);
        return self.board.playerAlive(Player::Computer);
    }

    fn pressed(&mut self, btn: &Button) {
    
        match btn {
            &Button::Keyboard(Key::Up)   => { self.board.updatePointer(0,-1); },
            &Button::Keyboard(Key::Down) => { self.board.updatePointer(0,1); },
            &Button::Keyboard(Key::Left) => { self.board.updatePointer(-1,0); },
            &Button::Keyboard(Key::Right) => { self.board.updatePointer(1,0); },
            &Button::Keyboard(Key::Return) => {self.turn=true;}
            _ => {},
        };
    
    }
    
}

pub struct Board {
    gl: GlGraphics,
    fields: i8,
    fields_p1: [[Area;10];10],
    fields_c1: [[Area;10];10],
    size: u32,
    ships_num: i8,
    ships_num_p1: i8,
    ships_num_c1: i8,
    ships_p1: [Ship;10],
    ships_c1: [Ship;10],
    pointerx: i8,
    pointery: i8,
}


fn fun(context : graphics::context::Context, gl : &mut  GlGraphics){
    const BLOCK: [f32; 4] = [0.42,0.42, 0.42, 1.0];
    let transform = context.transform; 
    let s = 440;    
    let mut square = graphics::rectangle::square((20) as f64, (20) as f64, s as f64);
    

    return graphics::rectangle(BLOCK, square, transform, gl);                 
}

impl Board {

    pub fn render(&mut self, args: &RenderArgs) {
        
        const NOTHING: [f32; 4] = [0.2, 0.82, 0.82, 1.0];
        const SHIP: [f32; 4] = [0.0, 0.8, 0.0, 1.0];
        const DESTROYEDSHIP: [f32; 4] = [0.8, 0.0, 0.0, 1.0];
        const BLOCK: [f32; 4] = [0.42,0.42, 0.42, 1.0];
        const MISS: [f32; 4] = [0.62, 0.62, 0.62, 1.0];
        const UNKNOWN: [f32; 4] = [0.82, 0.82, 0.82, 1.0];
        const FOCUS: [f32; 4] = [1.0, 1.0, 0.0, 0.45];
        
        let mut squares: Vec<graphics::types::Rectangle> = Vec::new();
        //first player board
        
        let mut board = self.fields_p1.clone();
        let s = self.size;
        
        //let callback = self.ffun;
        // called like
        //let this = &self;
        //let this_ref = this.ffun;

        //self.gl.draw(args.viewport(), &fun  );

        //self.gl.draw(args.viewport(),  graphics::rectangle(BLOCK, square, transform, gl)  );
        
        
        
        self.gl.draw(args.viewport(), |context, gl| {

                let transform = context.transform;     
                        for x in 0..10 {
                            for y in 0..10{
                                let mut square = graphics::rectangle::square((x*s+3*x+s) as f64, (y*s+3*y+s) as f64, s as f64);
                                match board[x as usize][y as usize] as Area{
                                    Area::NOTHING =>graphics::rectangle(NOTHING, square, transform, gl),
                                    Area::UNKNOWN =>graphics::rectangle(UNKNOWN, square, transform, gl),
                                    Area::SHIP =>graphics::rectangle(SHIP, square, transform, gl),
                                    Area::DESTROYEDSHIP =>graphics::rectangle(DESTROYEDSHIP, square, transform, gl),
                                    Area::MISS =>graphics::rectangle(MISS, square, transform, gl),
                                    Area::BLOCK =>graphics::rectangle(BLOCK, square, transform, gl),          
                                }
                            }}
                    
                    });                
                
        //second player board
        let mut board = self.fields_c1.clone();
        let (p1,p2) = self.getPointerSquare(self.pointerx as u32, self.pointery as u32);
        let pointer = graphics::rectangle::square((p1 + (s*12) as f64 ), p2 as f64, (s as f64)*0.8);

        self.gl.draw(args.viewport(), |context, gl| {
            let transform = context.transform;             
                    for x in 0..10 {
                        for y in 0..10{
                            let mut square = graphics::rectangle::square((x*s+3*x+s*13) as f64, (y*s+3*y+s) as f64, s as f64);

                            match board[x as usize][y as usize] as Area{
                                    Area::NOTHING       =>graphics::rectangle(NOTHING, square, transform, gl),
                                    Area::UNKNOWN       =>graphics::rectangle(UNKNOWN, square, transform, gl),
                                    Area::SHIP          =>graphics::rectangle(NOTHING, square, transform, gl),
                                    //Area::SHIP          =>graphics::rectangle(SHIP, square, transform, gl),
                                    Area::DESTROYEDSHIP =>graphics::rectangle(DESTROYEDSHIP, square, transform, gl),
                                    Area::MISS          =>graphics::rectangle(MISS, square, transform, gl),
                                    Area::BLOCK         =>graphics::rectangle(BLOCK, square, transform, gl),          
                            }
                        }}
                        //mark focus pointer
                    graphics::rectangle(FOCUS,  pointer, transform, gl);
                });                
    }

   
    pub fn playerAlive(&self, player : Player) -> bool{
        match player{
            Player::Computer => { return self.ships_c1.iter().any(|v| v.alive == true)  },
            Player::Player1 => { return self.ships_p1.iter().any(|v| v.alive == true) },
            _=> {return false;},
        }
    }

    pub fn getPointerSquare(&mut self, x : u32, y : u32) -> (f64,f64) { //-> graphics::types::Rectangle
        let s = self.size as f64*0.8;
        let s2 = self.size as f64*0.1;
        let xx = x*self.size+3*x+self.size+(s2 as u32);
        let yy = y*self.size+3*y+self.size+(s2 as u32);
        
        return (xx as f64,yy as f64);
    }

    pub fn updatePointer(&mut self, x : i8, y : i8){
        if self.pointerx+x <= 9 && self.pointerx+x >= 0 {self.pointerx+=x}
        if self.pointery+y <= 9 && self.pointery+y >= 0 {self.pointery+=y}
        println!("{} {}",self.pointerx,self.pointery)
    }

    fn push_ship(mut self,ship:Ship,player:Player){
        if player==Player::Player1{
            
            self.ships_p1[self.ships_num_p1 as usize]=ship;
            self.ships_p1[self.ships_num_p1 as usize].find_around();
            self.ships_num_p1+=1;
            self.ships_num+=1;
        }else{
            
            self.ships_c1[self.ships_num_c1 as usize]=ship;
            self.ships_c1[self.ships_num_c1 as usize].find_around();
            self.ships_num_c1+=1;
            self.ships_num+=1;
        }
    }

    pub fn generate_ship(&mut self,ship_type:i8, n:i8, player:Player){
        
        let mut r = thread_rng();
        let mut newx: Vec<i8> = Vec::new();
        let mut newy: Vec<i8> = Vec::new();
        
        for s in 0..n{
            loop {

                let mut ship=Ship{
                    alive: true,
                    owner: player,
                    nominal_size: ship_type,
                    current_size: ship_type,
                    fieldsx: [usize::MAX;4],
                    fieldsy: [usize::MAX;4],
                    fieldsx_around: [usize::MAX;40],
                    fieldsy_around: [usize::MAX;40],
                   };

                let mut new_x:i8 = r.gen_range(0, 9);
                let mut new_y:i8 = r.gen_range(0, 9);
                let mut orient:i8 = r.gen_range(1, 4);
                
                match orient{
                    1=>{ for i in 0..(ship_type) {newx.push(new_x+i); newy.push(new_y); }},
                    2=>{ for i in 0..(ship_type) {newx.push(new_x-i); newy.push(new_y) ;}},
                    3=>{ for i in 0..(ship_type) {newx.push(new_x);   newy.push(new_y+i) ;}},
                    4=>{ for i in 0..(ship_type) {newx.push(new_x);   newy.push(new_y-i) ;}},
                    _=>{}
                }

                let mut err=0;
                for i in 0..newx.len(){
                    err += self.is_collide((newx[i]) as usize,(newy[i]) as usize, player);           
                }

                if err==0{
                    for i in 0..newx.len(){
                        
                        ship.fieldsx[i]=newx[i] as usize;
                        ship.fieldsy[i]=newy[i] as usize;
                        
                        if player==Player::Player1{
                            self.fields_p1[newx[i] as usize][newy[i] as usize]=Area::SHIP;

                        }
                        else{
                            self.fields_c1[newx[i] as usize][newy[i] as usize]=Area::SHIP;    
                        }
                        
                    }
                    newx.clear();
                    newy.clear();
                    if player==Player::Player1{
                        //ship.find_around();
                        self.ships_p1[self.ships_num_p1 as usize]=ship;
                        self.ships_num_p1+=1;
                        self.ships_num+=1;
                    }else{
                        //ship.find_around();
                        self.ships_c1[self.ships_num_c1 as usize]=ship;
                        self.ships_num_c1+=1;
                        self.ships_num+=1;
                    }
                    

                    break;
                }
                else{
                    newx.clear();
                    newy.clear();
                    continue;
                }
                    
                }
            }
    }
    
    fn is_collide(&self, x: usize, y: usize, player:Player) -> u8 {
        let mut counter : u8 = 0;
        
        if x>9 || x<0 || y>9 || y<0{
            return 2;
        }
        let mut board:[[Area;10];10];
        if player==Player::Player1 {board = self.fields_p1.clone();} 
        else {board = self.fields_c1.clone()}

        counter =   (y>0 &&        board[x][y-1]   !=  Area::NOTHING) as u8 +
                                  (board[x][y]     !=  Area::NOTHING) as u8 +
                    (y<9 &&        board[x][y+1]   !=  Area::NOTHING) as u8 +
                    (x<9 && y>0 && board[x+1][y-1] !=  Area::NOTHING) as u8 +
                    (x<9 &&        board[x+1][y]   !=  Area::NOTHING) as u8 +
                    (x<9 && y<9 && board[x+1][y+1] !=  Area::NOTHING) as u8 +
                    (x>0 && y>0 && board[x-1][y-1] !=  Area::NOTHING) as u8 +
                    (x>0 &&        board[x-1][y]   !=  Area::NOTHING) as u8 +
                    (x>0 && y<9 && board[x-1][y+1] !=  Area::NOTHING) as u8;
        
        return counter;
            
    }

    //shot into player
    pub fn shot(&mut self, x: usize, y: usize, player:Player ){
        let mut board:[[Area;10];10];
        let mut ships:&[Ship;10];

        if player==Player::Player1{
           board = self.fields_p1.clone();
        }
        else{
            board = self.fields_c1.clone();
        }
            
        if board[x][y] == Area::SHIP{
            //board[x][y] = Area::DESTROYEDSHIP;
            if player==Player::Player1{
                for i in 0..self.ships_p1.len(){
                    
                    for j in 0..self.ships_p1[i].nominal_size as usize{
                        if self.ships_p1[i].fieldsx[j]==x && self.ships_p1[i].fieldsy[j]==y{
                            self.ships_p1[i].decrement_size();
                            if !self.ships_p1[i].is_alive(){
                                println!("Statek zatopiony!");
                                let mut k=0;
                                self.ships_p1[i].find_around();
                                while self.ships_p1[i].fieldsx_around[k]!=usize::MAX{
                                    let xx:usize = self.ships_p1[i].fieldsx_around[k];
                                    let yy:usize = self.ships_p1[i].fieldsy_around[k];
                                    //print!("x={}|y={} ",xx,yy);
                                    if board[xx][yy]==Area::NOTHING { board[xx][yy]=Area::BLOCK; }
                                    k+=1;
                                }
                                //println!("k:{}",k)
                            }
                        }
                    }
                    //println!("{},{}",self.ships_p1[i].current_size,self.ships_p1[i].is_alive());
                }
            }
            else{
                for i in 0..self.ships_c1.len(){
                    
                    for j in 0..self.ships_c1[i].nominal_size as usize{
                        if self.ships_c1[i].fieldsx[j]==x && self.ships_c1[i].fieldsy[j]==y{
                            self.ships_c1[i].decrement_size();
                            if !self.ships_c1[i].is_alive(){
                                println!("Statek zatopiony!");
                                let mut k=0;
                                self.ships_c1[i].find_around();
                                while self.ships_c1[i].fieldsx_around[k]!=usize::MAX{
                                    let xx:usize = self.ships_c1[i].fieldsx_around[k];
                                    let yy:usize = self.ships_c1[i].fieldsy_around[k];
                                    //print!("x={}|y={} ",xx,yy);
                                    if board[xx][yy]==Area::NOTHING { board[xx][yy]=Area::BLOCK; }
                                    k+=1;
                                }
                                //println!("k:{}",k)
                            }
                        }
                        //println!("{},{}",self.ships_c1[i].current_size,self.ships_c1[i].is_alive());
                    }
                }
            
            }
        }
        board[x][y] = match board[x][y]{
            Area::NOTHING =>Area::MISS,
            Area::UNKNOWN =>Area::MISS,
            Area::SHIP =>Area::DESTROYEDSHIP,
            Area::DESTROYEDSHIP =>Area::DESTROYEDSHIP,
            Area::MISS => Area::MISS,
            Area::BLOCK => Area::BLOCK,
            _=>Area::NOTHING
        }; 

        if player==Player::Player1{
            self.fields_p1 = board;
            //self.ships_p1 = *ships; 
             }
        else{
            self.fields_c1 = board;
            //self.ships_c1 = *ships;
             }
    }  

}


#[derive(Copy,Clone, PartialEq)]
pub struct Ship{
    alive: bool,
    nominal_size: i8,
    current_size: i8,
    owner: Player,
    fieldsx: [usize;4], 
    fieldsy: [usize;4],
    fieldsx_around: [usize;40],
    fieldsy_around: [usize;40],
}

impl Ship{

    fn new() -> Self {
        Ship {
            alive: false,
            nominal_size: 0,
            current_size: 0,
            owner: Player::Computer,
            fieldsx: [0;4],
            fieldsy: [0;4],
            fieldsx_around: [0;40],
            fieldsy_around: [0;40],
        }
    }

    pub fn is_alive(&self)->bool{
        return self.alive;
    }
    pub fn decrement_size(&mut self){
        self.current_size=self.current_size-1;
        if self.current_size==0{
            self.alive=false;
        }
    }
    
    pub fn find_around(&mut self){
        let mut pointer=0;
        for i in 0..self.nominal_size as usize{
            if self.fieldsy[i]>0 
            {self.fieldsx_around[pointer]=self.fieldsx[i];   pointer+=0; self.fieldsy_around[pointer]=self.fieldsy[i]-1;  pointer+=1; }
            if self.fieldsy[i]<9 
            {self.fieldsx_around[pointer]=self.fieldsx[i];   pointer+=0; self.fieldsy_around[pointer]=self.fieldsy[i]+1;  pointer+=1; }
            if self.fieldsx[i]>0 && self.fieldsy[i]>0 
            {self.fieldsx_around[pointer]=self.fieldsx[i]-1; pointer+=0; self.fieldsy_around[pointer]=self.fieldsy[i]-1; pointer+=1; }
            if self.fieldsx[i]>0 
            {self.fieldsx_around[pointer]=self.fieldsx[i]-1; pointer+=0; self.fieldsy_around[pointer]=self.fieldsy[i];   pointer+=1; }
            if self.fieldsx[i]>0 && self.fieldsy[i]<9 
            {self.fieldsx_around[pointer]=self.fieldsx[i]-1; pointer+=0; self.fieldsy_around[pointer]=self.fieldsy[i]+1; pointer+=1; }
            if self.fieldsx[i]<9 && self.fieldsy[i]>0 
            {self.fieldsx_around[pointer]=self.fieldsx[i]+1; pointer+=0; self.fieldsy_around[pointer]=self.fieldsy[i]-1; pointer+=1; }
            if self.fieldsx[i]<9 
            {self.fieldsx_around[pointer]=self.fieldsx[i]+1; pointer+=0; self.fieldsy_around[pointer]=self.fieldsy[i];   pointer+=1; }
            if self.fieldsy[i]<9 && self.fieldsx[i]<9 
            {self.fieldsx_around[pointer]=self.fieldsx[i]+1; pointer+=0; self.fieldsy_around[pointer]=self.fieldsy[i]+1; pointer+=1; } 
            }
        }    
        
}

fn main() {
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 25;
    const ROWS: u32 = 15;
    const SQUARE_WIDTH: u32 = 40;

    let width = COLS * SQUARE_WIDTH;
    let height = ROWS * SQUARE_WIDTH;

    let mut window: GlutinWindow = WindowSettings::new("Ship Game", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rows: ROWS,
        cols: COLS,
        square_width: SQUARE_WIDTH,
        result: Player::Computer,
        turn: false,
        board :Board{
            gl: GlGraphics::new(opengl),
            fields: 10,
            fields_p1: [[Area::NOTHING;10];10],
            fields_c1: [[Area::NOTHING;10];10],
            size: SQUARE_WIDTH,
            ships_num: 0,
            ships_num_p1: 0,
            ships_num_c1: 0,
            ships_p1:  [Ship::new();10],
            ships_c1:  [Ship::new();10],
            pointerx: 0,
            pointery: 0,
        },
        
    };
    
    game.board.generate_ship(3,4,Player::Player1);
    game.board.generate_ship(4,1,Player::Player1);
    game.board.generate_ship(2,2,Player::Player1);
    game.board.generate_ship(3,4,Player::Computer);
    game.board.generate_ship(4,1,Player::Computer);
    game.board.generate_ship(2,2,Player::Computer);
    
    let mut events = Events::new(EventSettings::new()).ups(5);
    while let Some(e) = events.next(&mut window) {
       
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
           
    }
    if game.result==Player::Player1{
        println!("Gratuluje wygrałeś! Wszystkie statki wroga zostały zatopione!");
    }
    else {
        println!("Zostałeś pokonany!");
    }
    
}





use std::io::{self, Write};
use std::thread;
use std::time::Duration;
fn load_map(map: &Vec<Vec<String>>)->String{
    let mut buffer = String::new();
    for _ in &map[0]{buffer.push_str("=-");}
    buffer.push_str("=\n");
    for row in map{
        for i in row{
            buffer.push_str(&format!("|{}",i));
        }
        buffer.push_str("|\n");
    }
    for _ in &map[0]{buffer.push_str("=-");}
    buffer.push_str("=\n");
    buffer
}

struct Ball {
    i:i32,
    j:i32,
    old_i:i32,
    old_j:i32,
    move_way:i32,
    ball_skin:String,
    trajectory_skin:String,
}
impl Ball {
    fn update(&mut self,map: &Vec<Vec<String>>){
        self.old_i = self.i;self.old_j = self.j;
        self.j+=self.move_way;
        if self.j < 0 || self.j >= map[self.i as usize].len() as i32{
            self.i+=self.move_way;
            if self.i<0 || self.i >= map.len() as i32{
                self.move_way = if self.move_way==-1 {1} else {-1};
                self.i += self.move_way;
            }
            self.j = if self.move_way == 1 {0} else {map[self.i as usize].len() as i32-1};
            if map[map.len()-1].len() !=1{
                if self.i==map.len() as i32 -1 && self.j==(map[map.len()-1].len() as i32-1) {
                    self.j = map[map.len()-1].len() as i32-2;
                }
                if self.i==0 && self.j==0 {self.j = 1;}
            }
        }
    }
    fn draw_a_ball(&mut self,map: &mut Vec<Vec<String>>){
        map[self.old_i as usize][self.old_j as usize]=self.trajectory_skin.to_string();
        map[self.i as usize][self.j as usize]=self.ball_skin.to_string();
    }
    fn clean_a_ball(&mut self,map: &mut Vec<Vec<String>>){
        map[self.old_i as usize][self.old_j as usize]=' '.to_string();
        map[self.i as usize][self.j as usize]=' '.to_string();
    }
}
fn a_ball_move(map: &mut Vec<Vec<String>>,ball: &mut Ball){
    ball.clean_a_ball(map);
    ball.update(map);
    ball.draw_a_ball(map);
}
fn main() {
    let mut map:Vec<Vec<String>> = vec![];
    for _ in 0..5{
        let mut row: Vec<String> = vec![];
        for _ in 0..5{
            row.push(" ".to_string());
        }
        map.push(row);
    }
    let mut ball = Ball{i:0,j:0,old_i:0,old_j:0,move_way:1,ball_skin:"\x1B[32mo\x1B[0m".to_string(),trajectory_skin:"\x1B[92m.\x1B[0m".to_string()};
    let mut ball2 = Ball{i:1,j:0,old_i:0,old_j:0,move_way:-1,ball_skin:"\x1B[33mo\x1B[0m".to_string(),trajectory_skin:"\x1B[93m.\x1B[0m".to_string()};
    loop{
        
        a_ball_move(&mut map,&mut ball);
        a_ball_move(&mut map, &mut ball2);
        print!("\x1B[2J\x1B[H{}",load_map(&map));
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs_f32(0.262144));
        

    }
}
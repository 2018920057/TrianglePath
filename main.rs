use std::cmp;
use std::io;

static mut triangle: [[i32;10];10] = [[0;10];10];

fn main() {
  unsafe{
    //입력받기
    let mut input: String = String::new();
    println!("삼각형의 높이:");
    io::stdin().read_line(&mut input).expect("input error");
    let height: usize = input.trim().parse().unwrap();
    for i in 0..height {
      println!("{}층의 숫자 {}개 입력:",i,i+1);
      input = String::from("");
      io::stdin().read_line(&mut input).expect("input error");
      let mut split = input.split_whitespace();
      for j in 0..i+1 {
        triangle[i][j] = split.next().unwrap().parse().unwrap();
      }
    }
    //삼각형 출력
    for i in 0..height {
      for j in 0..i+1 {
        print!("{}",triangle[i][j]);
      }
      println!();
    }
    //최장경로 출력
    println!("최장경로: {}",pathSum(0,0,0));
  }
}

//현재 위치가 (y,x)이고 현재까지의 합이 sum일 때, 최대 합 구하기
unsafe fn pathSum(y:usize, x:usize, sum:i32) -> i32 {
  if y==9 {return sum+triangle[y][x];}
  cmp::max(pathSum(y+1,x,sum+triangle[y][x]),pathSum(y+1,x+1,sum+triangle[y][x]))

}
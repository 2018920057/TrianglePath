use std::cmp;


fn main() {
  let mut triangle: [[i32;10];10] = [[0;10];10];
  triangle[0][0]=5;
  println!("{:?}",triangle);
  println!("{}",pathSum(0,0,0,&triangle))
}

//현재 위치가 (y,x)이고 현재까지의 합이 sum일 때, 최대 합 구하기
fn pathSum(y:usize, x:usize, sum:i32, triangle: &[[i32;10];10]) -> i32 {
  if(y==9) {return sum+triangle[y][x];}
  cmp::max(pathSum(y+1,x,sum+triangle[y][x],triangle),pathSum(y+1,x+1,sum+triangle[y][x],triangle))
}
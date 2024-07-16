

macro_rules! swap {
  ($a:expr,$b:expr)=> {{
    $a=$a^$b;
    $b=$a^$b;
    $a=$a^$b;
  }};
}

pub struct Solution;
impl Solution {
  pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    // transpose
    for i in 0..matrix.len() {
      for j in i+1..matrix[i].len() {
        swap!(matrix[i][j],matrix[j][i]);
      }
    }

    // reflection
    for row in matrix {
      let len=row.len();
      for i in 0..(len/2) {
        swap!(row[i],row[len-i-1]);
      }
    }
  }
}


const _MATRIX: [[i32;3];3]=[
  [1,2,3],
  [4,5,6],
  [7,8,9]
];

fn main() {
  let mut matrix=_MATRIX.into_iter()
  .map(|row| row.to_vec())
  .collect();

  Solution::rotate(&mut matrix);
  println!("{matrix:?}");
}




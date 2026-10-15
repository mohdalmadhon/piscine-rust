#[derive(Debug)]
struct Matrix ((i32,i32),(i32,i32));

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
}

pub fn transpose(m: Matrix) -> Matrix {
    let res = Matrix(
        (m.0.0, m.1.0),
        (m.0.1, m.1.1)
    );
    
    res
}
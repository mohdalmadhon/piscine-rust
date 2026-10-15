struct Matrix ((i32,i32),(i32,i32));

fn transpose(m: Matrix) -> Matrix {
    let mut res = Matrix(
        (m.0.0, m.1.0),
        (m.0.1, m.1.1)
    );
    
    res
}
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];
    for x in 0..3 {
        for y in 0..3 {
            transposed[y][x] = matrix[x][y];
        }
    }
    transposed
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix.iter() {
        println!("{row:?}");
    }
}

fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    println!("matrix");
    pretty_print(&matrix);

    let transposed = transpose(matrix);

    println!("trasnposed:");
    pretty_print(&transposed);
}

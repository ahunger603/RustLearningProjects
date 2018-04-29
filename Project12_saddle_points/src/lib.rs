fn get_maxtrix_row_max_col_min(input: &[Vec<u64>]) -> (Vec<u64>, Vec<u64>) {
    let mut row_max: Vec<u64> = Vec::new();
    let mut col_min: Vec<u64> = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, &element) in row.iter().enumerate() {
            if i == 0 {
                col_min.push(element);
            } else {
                if element < col_min[j] {
                    col_min[j] = element;
                }
            }
            if j == 0 {
                row_max.push(element);
            } else {
                if element > row_max[i] {
                    row_max[i] = element;
                }
            }
        }
    }
    (row_max, col_min)
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();
    let row_max_col_min = get_maxtrix_row_max_col_min(input);
    for (i, row) in input.iter().enumerate() {
        for (j, &element) in row.iter().enumerate() {
            if element <= row_max_col_min.1[j] && element >= row_max_col_min.0[i] {
                saddle_points.push((i, j));
            }
        }
    }
    
    saddle_points
}

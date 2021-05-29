pub fn read_vec(filename: String) -> Vec<i32> {
    return std::fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

pub fn copy(a: &Vec<i32>, b: &mut Vec<i32>) {
    *b = Vec::new();
    for x in a {
        b.push(*x);
    }
}

pub fn read_vecvec(filename: String) -> Vec<Vec<i32>> {
    let tmp = std::fs::read_to_string(filename).unwrap();
    let mut itr = tmp.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let h = itr.next().unwrap() as usize;
    let w = itr.next().unwrap() as usize;
    let mut a: Vec<Vec<i32>> = Vec::new();
    for i in 0..h {
        a.push(Vec::new());
        for _j in 0..w {
            a[i].push(itr.next().unwrap());
        }
    }

    return a;
}

pub fn read_vec(filename: String) -> Vec<i32> {
    return std::fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

pub fn copy(a: &Vec<Vec<i32>>, b: &mut Vec<Vec<i32>>) {
    *b = Vec::new();
    for i in 0..a.len() {
        b.push(Vec::new());
        for x in &a[i] {
            b[i].push(*x);
        }
    }
}

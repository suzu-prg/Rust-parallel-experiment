pub fn read_vec(filename: String) -> Vec<Vec<i32>> {
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

pub fn read_2vec(filename: String) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let tmp = std::fs::read_to_string(filename).unwrap();
    let mut itr = tmp.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let h1 = itr.next().unwrap() as usize;
    let w1 = itr.next().unwrap() as usize;
    let mut a: Vec<Vec<i32>> = Vec::new();
    for i in 0..h1 {
        a.push(Vec::new());
        for _j in 0..w1 {
            a[i].push(itr.next().unwrap());
        }
    }
    let h2 = itr.next().unwrap() as usize;
    let w2 = itr.next().unwrap() as usize;
    let mut b: Vec<Vec<i32>> = Vec::new();
    for i in 0..h2 {
        b.push(Vec::new());
        for _j in 0..w2 {
            b[i].push(itr.next().unwrap());
        }
    }

    return (a, b);
}

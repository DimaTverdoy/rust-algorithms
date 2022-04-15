pub fn search(t: &str, p: &str) -> Option<usize> {
    let mut j;

    for i in 0..(t.len() - p.len() + 1) {
        j = 0;
        while (j < p.len()) && (t.chars().nth(i + j) == p.chars().nth(j)) {
            j += 1
        }

        if j == p.len() {
            return Some(i);
        }
    }

    None
}

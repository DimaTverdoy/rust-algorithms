use std::mem::swap;

pub fn sort<T: Ord>(a: &mut [T]) {
    let mut j = 0;
    for i in 1..a.len() {
        j = i;
        while (j > 0 && a[j] < a[j - 1]) {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}
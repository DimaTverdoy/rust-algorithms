pub(crate) fn sort<T: std::cmp::PartialOrd>(a: &mut [T]) {
    let mut min;

    for i in 0..a.len() {
        min = i;
        for j in (i + 1)..a.len() {
            if a[min] > a[j] {
                min = j;
            }
        }
        a.swap(i, min);
    }
}
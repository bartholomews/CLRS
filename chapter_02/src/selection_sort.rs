pub fn apply<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let s = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[s] {
                arr.swap(j, s)
            }
        }
    }
}
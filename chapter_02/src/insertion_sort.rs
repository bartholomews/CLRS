pub fn inc<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        /*
        insert arr[j] into the sorted sequence arr[0..j-1]
        arr[0..j-1] is a loop invariant, always in sorted order:
        - before the first iteration (Initialization)
        - before the next iteration (Maintenance)
        - when the loop terminates (Termination)
        It can be proved inductively, with the base case at initialization
        and the inductive step on each iteration.
         */
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn dec<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] < arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn recursive<T: Ord>(arr: &mut [T], j: usize) {
    if j > 0 {
        recursive(arr, j - 1);
        let mut i: usize = j - 1;
        while i > 0 && arr[i] > arr[i + 1] {
            arr.swap(i, i + 1);
            i -= 1;
        }
    }
}

mod insertion;

fn main() {
    let v = &mut vec![3, 2, 42, 1, 232, 23];
    insertion::sort(v);
    println!("{:?}", v)
}

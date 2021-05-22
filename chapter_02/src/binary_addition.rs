pub fn binary_addition(a: &Vec<bool>, b: &Vec<bool>) -> Vec<i8> {
    let mut carry = false;
    let mut c = Vec::new();
    for i in (0..a.len()).rev() {
        if a[i] == false && b[i] == false {
            c.push(carry);
            carry = false;
        }
        else if a[i] == true && b[i] == true {
            c.push(carry);
            carry = true;
        }
        else {
            c.push(!carry);
        }
    }
    c.push(carry);
    return c.iter().map(|&b| b as i8).rev().collect();
}
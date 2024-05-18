fn main() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // let third = v[2];
    // let fifth = v[4];

    let fifth = v.pop().expect("vector empty!");
    println!("Fifth element is {}", fifth);

    let second = v.swap_remove(1);
    println!("Second element is {}", second);

    let third = std::mem::replace(&mut v[2], "replaced".to_string());
    println!("Third element is {}", third);

    println!("Vector: {:?}", v);
}

/*
fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next)
    }
    println!("P(1..10 = {:?}", padovan);
} // dropped here
*/
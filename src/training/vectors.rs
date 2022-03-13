use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> =vec![1, 2, 3, 4];
    let mut other_numbers: Vec<i32> = Vec::from([100, 101, 102, 103]);
    println!("{:#?}", numbers);
    numbers.push(5);
    println!("{:#?}", numbers);
    let mut finished:bool = false;

    while !finished {
        match other_numbers.pop() {
            Some(x) => println!("Found value! {}", x),
            None    => {
                println!("Found no value, maybe raise some error or something ?");
                finished = true;
            },
        }
    }
    // What the fuck is "Some"  => https://doc.rust-lang.org/std/option/
    //Seems to be some kind of wrapper for functions that we dont know whether they actually returned something or not.
    //Basically a proxy for a null pointer ? except, it's not a null pointer which is very obviously good because it forces you to handle it explicitly ?

    assert_eq!( other_numbers.capacity() * mem::size_of::<i32>(), mem::size_of_val(&other_numbers));

    println!("Vector capacity (in bytes) = {}", other_numbers.capacity() * mem::size_of::<i32>());
    let slice = &[1, 2, 3, 4, 5];
    other_numbers.extend_from_slice(slice);
    println!("Vector capacity after extending of {} bytes : {}", mem::size_of_val(slice), other_numbers.capacity() * mem::size_of::<i32>());

}
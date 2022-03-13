pub fn run() {
    let name = "Dominic";
    let mut age  = 31;

    println!("My name is {} and I am {}", name, age);
    age = 32;

    println!("My name is {} and I am {}", name, age);


    // Define constant
    const ID:i32 = 001;
    println!("ID = {}", ID);

    // Assign multiple vars

    let ( my_name, my_age ) = ("Dominic", 31);
    println!("{} is {}", my_name, my_age);
    let mut x:i8 = 0;
    // This will work in release, but not in debug !
    // loop {
    //     x+=1;
    //     println!("{}", x);
    // }

    //to explicitly handle overflows, 4 methods

    //1: Checked : Will return an Option, so either Some(x) if no overflow, or None if overflow :
    assert_eq!(100u8.checked_add(200), None);
    assert_eq!(100u8.checked_add(100), Some(200));

    //2: Wrapping : This is basically the default behavior during release build : the value will overflow and "wrap" around to the other extreme.
    assert_eq!(100u8.wrapping_add(200), 44);
    assert_eq!(100u8.wrapping_add(100), 200);

    //3: Saturating : in case of an overflow, will "stop" at the last value before overflowing ; e.g the ::MAX or ::MIN of whatever type we're overflowing from
    assert_eq!(100u8.saturating_add(200), u8::MAX);
    assert_eq!(100u8.saturating_add(100), 200);

    //4: Overflowing : will return a tuple : first item is the wrapped result, and second is a bool telling us if an overflow happened
    let (result, overflew) = 100u8.overflowing_add(200);
    if overflew {
        println!("overflow by {}", result);
    }
}
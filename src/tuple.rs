// max 12 elements (???)
//that info seems outdated

pub fn run() {
    let some_tuple:(i8, i8, i8) = (1, 2, 3);
    println!("{:?}", some_tuple);

    println!("{}, {}, {}", some_tuple.0, some_tuple.1, some_tuple.2);

    let _other_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);

    //assert_eq!(13, other_tuple.len())     There is no way to get a tuple length (?)
}
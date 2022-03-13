//Fixed length (Vectors are the same but variable length)

use std::mem::size_of_val;      //kind of a namespace

pub fn run() {
    let numbers: [i8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("{}", numbers[0]);
    
    let mut other_numbers:  [i16; 4] = [21, 22, 23, 24];
    other_numbers[3] = 25;
    println!("{}", other_numbers[3]);
    println!("Array length : {}", numbers.len());

    // Check stack allocation

    println!("Here is how to get stack byte allocation : {}", size_of_val(&other_numbers));

    // Arrays are just blobs of memory in the stack, not pointers like C. (are C arrays stored in the heap ? either way, this promises some decent performance uplift compared to C for sure)

    //Get slice
    let slice: &[i16] =&other_numbers;  //This is more like C arrays : pointer that only knows the type of element it's pointing to, but not necesarily its size (although it still does uhh maybe it just deduces it idk)
    println!("Slice: {:#?}", slice);       //{:#?} for pretty print debug trait    => why cant it take *slice ? starting to really behave like a C array...

    let partial_slice: &[i16] = &other_numbers[0..2];       //for slices, first is included, last is excluded.
    println!("{:#?}", partial_slice);
    //println!("{}", partial_slice.capacity());             //no capacity method
    //This would imply that a pointer to the stack does not have a capacity, which makes sense since it would basically be like an array in C : a pointer + a size.
    //A capacity only makes sense in the heap as the length can grow and shrink, and capacity has to always be bigger than it.
    //in the stack, you cannot do dynamic memory allocation ; allocated memory is static and immutable. so capacity and length are always equal, since the exact capacity will always be used for a certain length.
}
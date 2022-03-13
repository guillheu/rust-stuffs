//two types of strings :
// str = immutable in stack (?)
// String = Vector in heap


pub fn run() {
    let mut hello = String::from("Hello");

    let mut hullo = "Hullo";

    // Get length

    println!("{}, and, {}", hello, hullo);
    println!("{} is {} characters long", hello, hello.len());

    println!("{} is {} characters long", hullo, hullo.len());


    hello = String::from("Hola");

    hullo = "Buenos dias senorita";

    println!("{}, and, {}", hello, hullo);


    hello.push(' ');
    hello.push('w');
    println!("{}", hello);
    hello.push_str("orld!");
    println!("{}", hello);


    // Capacity in bytes
    println!("{}", hello.capacity());
    //println!("{}", hullo.capacity()); does not work

    println!("{}", hello.is_empty());
    println!("{}", hello.contains(hullo));
    println!("{}", hello.replace("Hola", hullo));   //creates and returns a copy
    println!("{}", hello);

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s:String = String::with_capacity(7);       //creates a new empty string similar to String::new(), but pre-allocates some memory for it.
                                                        //if you know how big your string is going to be, like a hex of a hash, you might wanna use this.
                                                        //Although this begs the question, why not just use an &str ? I suppose time will tell 
                                                        // Capacity will double if length exceeds capacity

    println!("{}", s.capacity());

    s.push_str("1234567");
    println!("{}, {}", s, s.capacity());
    s.push('8');

    println!("{}, {}", s, s.capacity());
    s.push_str("9012345");

    println!("{}, {}", s, s.capacity());

    s = String::new();
    println!("{}", s.capacity());
    s.push('a');
    println!("{}", s.capacity());       //String::new() creates String with capacity 0. then, first push pushes it to 8.
    s = String::with_capacity(1);
    println!("{}", s.capacity());
    s.push('h');
    println!("{}", s.capacity());
    s.push('i');
    println!("{}", s.capacity());   //interesting, capacity jumps from 1 to 8. I was expecting it to go to 2. there must be some limit between 1 and 7 where it stops jumping to 8 and starts doubling instead.

    s = String::with_capacity(2);
    println!("{}", s.capacity());
    s.push_str("hi!");
    println!("{}", s.capacity());
    s = String::with_capacity(3);
    println!("{}", s.capacity());
    s.push_str("hi! ");
    println!("{}", s.capacity());

    s = String::with_capacity(4);
    println!("{}", s.capacity());
    s.push_str("hi! H");
    println!("{}", s.capacity());

    s = String::with_capacity(5);
    println!("{}", s.capacity());
    s.push_str("hi! Ho");
    println!("{}", s.capacity());       //That limit is 5. makes sense, since at 4, doubling or jumping to 5 are the same thing. so it's basically just "double or jump to 8, whichever is greater"

    //Capacity seem very important in some application that handles an array of strings for instance. can save memory on the capacity


    //Good video https://www.youtube.com/watch?v=tUG_NItm2HY
}
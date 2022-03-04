pub fn run() {
    let n = 8;
    let x = &8;
    let y = &n;
    assert_eq!(n, *x);
    assert_eq!(y, x);
    let mut z = y;
    println!("{}, -> {}", z, *z);
    z = &5;
    println!("{:?}", (n, x, y, z, *x, *y, *z));


    let mut x = String::from("hello");
    let a = x;  //This UNSETS x, because '=' will try to use a copy, but because String uses the heap, and rust does not want two pointers pointing to the same value in the heap, this will MOVE the pointer from x to a.
    //Alternatively, we can force the String::clone operation
    x = a.clone(); 
    println!("{}, {}", x, a);

    // Functions hold ownership, and ownership is dropped when a function ends
    // So, a pointer passed to a function WILL BE DROPPED AT THE END OF THE FUNCTION
    // Part of the logic is to avoid having a function that modifies it own input (in this case, the pointer)
    // You can however force a function to return ownership of the function

    let x = String::from("Goodbye");

    println!("{}", x);
    drop_pointer(x);
    //println!("{}", x);    <- This wouldnt work, because we moved pointer ownership, remember ?
    let x = String::from("Hello");
    let mut a = return_ownership(x);
    println!("{}", a);


    // let mut z = &mut a;
    // let mut y = &mut a;

    // println!("{} and {} both point to {}", y, z, a);

    // *z = String::from("Huh?!");

    // println!("{} and {} both point to {}", y, z, a);

    /*
    These lines up here will not work ! Rust will not let you create two mutable references to the same value in the heap
    Error message :

            error[E0499]: cannot borrow `a` as mutable more than once at a time
            --> src/pointers.rs:35:17
            |
            34 |     let mut z = &mut a;
            |                 ------ first mutable borrow occurs here
            35 |     let mut y = &mut a;
            |                 ^^^^^^ second mutable borrow occurs here
            36 | 
            37 |     println!("{} and {} both point to {}", y, z, a);
            |                                               - first borrow later used here

    */

    {
        let _z = &mut a;            //bad practice, dont use _ to remove warnings for production. only doing it to clarify compile output of this demo/test thingy
                                    //Note from the future : this _ thing seems to be doing more than just removing a warning... which isnt exactly surprising ; but what does it do ?! Will have to investigate later.
        let y = &mut a;
        //println!("{} and {}", y, z);
        /*
            This will not work. Technically, we cant have multiple mutable borrows at a time.
            However, it looks like we can set them, but not use them.
            This code will compile, but the print wont, giving us this error :
                error[E0499]: cannot borrow `a` as mutable more than once at a time
                --> src/pointers.rs:62:17
                |
                61 |         let z = &mut a;
                |                 ------ first mutable borrow occurs here
                62 |         let y = &mut a;
                |                 ^^^^^^ second mutable borrow occurs here
                63 |         println!("{} and {}", y, z);
                |                                  - first borrow later used here

                For more information about this error, try `rustc --explain E0499`.
                error: could not compile `rust_app` due to previous error
        */
        println!("{}", y);      //we can ONLY use the LAST borrowing. this makes sense ; only having a single instance of a pointer.
        //This means that
        //      let z = &mut a;
        //      let y = &mut a;
        //is strictly the same as
        //      let z = &mut a;
        //      let y = z;
        //println!("{}", z);
    }
    {
        //However, we should be able to do what we attempted up there, but with immutable values. This would prevent a lot of ugliness (like a pointer changing the value to which another pointer is pointing)
        //So, my understanding is, we can have as many of this :
        let mut a = String::from("Hello!");
        let b = &a;
        let c = &a;
        let d = &a;
        let e = &a;
        println!("{}, {}, {}, {}, {} !", a, b, c, d, e);
        //We cannot modify a, because that would also modify b, c, d and e !
        //a = String::from("Hola!");    //This will fail!
        //This can be a problem, because you might want to create pointers to a value, and THEN modify that value once you're done with the pointers and drop them
        //you could use a function, declare your new pointers in there, use them for whatever you like, then at the end of the function, the pointers are dropped and your value has no pointer pointing to it, so you can modify it.
        //Here's another way to do it than using a function :
        let mut x = String::from("Read!");
        {
            let y = &x;
            let z = &x;
            println!("{}, {} and {}", x, y, z);
        }
        x = String::from("Write!");     //This works because the pointers y and z are dropped after this... appendice ? thingamajig. -> scope
                                        //This is indeed hard to code with but quite error proof 
                                        //This is essentially like a temporary read-only borrowage, then once all the readers are dead, you can overwrite your variable
        println!("{}", x);

        //AND, we can only have ONE pointer to mutable
        let y = &mut a;
        let z = y;

        //println!("{} and {}", y, z)       //This obviously wont work, not the final mutable borrowage.
        //println!("{}", y);                //Neither will this

        println!("{}", z);                  //But this should ! This is the last borrowage !



        //Ok but, what about functions ? can functions use pointers ? and pointers to mutables ?
        //Yes they can ! in fact we already did a function that uses a pointer whith drop_pointer and return_ownership, since String is actually a Vector which uses a pointer pointing to the heap to have dynamic size.
        //Lets use a proper function with i32s to demonstrate this concept :
        let n = 10;
        println!("{}", n);
        let m = &n;
        println!("{}", *m);
        println!("{}", n);        //This will still work because, remember, we can have as many read-only borrows of a variable as we want. m and n are just reading from the same value, not changing it. this is actually fine !
        pointer_argument(&n);
        //So what now, can i use n ???? i mean, it was immutable, wasnt it ? so i'm not passing ownership, just creating another read-only borrow in a function. i think this should work
        println!("{}", n);      //And I'm right ! I'm getting the hang of this ^^
        //However i wont be able to do a mutable borrow (passing of ownership, read-write borrow, same thing), yet, because i cant do that so long as other variables are borrowing it, or in other words, as long as other read-only pointers are pointing to it.
        //...Right ? I mean, doubly not because also, n is currently immutable. There's no way you can pass an immutable variable to a pointer to mutable.
        //........... RIGHT ????
        //lets try

        //first, with clean (unborrowed) immutable variable to pointer to mutable:
        let some_val = 69; //Nice.
        //let mut_to_val = &mut some_val;       //Yep, this doesnt work as expected. error output :
        /*

            error[E0596]: cannot borrow `some_val` as mutable, as it is not declared as mutable
            --> src/pointers.rs:144:26
                |
            143 |         let some_val = 69; //Nice.
                |             -------- help: consider changing this to be mutable: `mut some_val`
            144 |         let mut_to_val = &mut some_val;
                |                          ^^^^^^^^^^^^^ cannot borrow as mutable


        */

        //Ok now lets make this mutable :
        let mut some_val = 69;       
        //Then, have some read-only pointers point to it
        let immut_point = &some_val;
        //Now, can I assign a pointer to mutable to it ?
        let mut_point = &mut some_val;
        //Yes I can !
        //But can i actually modify the value now ? i dont think so. lets try.
        //some_val = 70; //Not Nice.
        //Huh... we can...
        //AH, this is some rusty weirdness. basically ONCE we actually try to READ from BOTH the value and the pointer does the program tell us that we cant actually modify the value.
        //So the idea is, "is a value isnt read, was it pointed to at all ?🧘"
        //I'm willing to bet that, now that we actually read from both the pointer and the value, the compiler wont even let us set a mutable pointer to it.
        //Ok, not quite. instead, we... i mean, obviously, we just passed ownership when we set the pointer to mutable.
        //println!("{}, {}", some_val, mut_point);
        println!("{}", mut_point);

        //Note to self : INVESTIGATE THE _ MADNESS

        //ok now, lets finally try using that function:
        pointer_to_mutable_argument(mut_point);         //This works, i'm not sure why. I'm scared. Maybe there are some answers in _ ?
        println!("{}", *mut_point);                     //Surely this wont work, right ? we literally passed ownership AND modified the value. there's NO WAY a pointer can point to a value that's been changed by a different pointer.
        //What, the actual, fuck
        //Maybe this has to do with the "if unused, then non existant 🧘" (lets give this a name... "The Fallen Tree Conjecture" ! "TFTC") concept from earlier ? maybe it extends to mutability as well ? maybe a pointer to mutable is really only a pointer to mutable if we actually use it to mutate the value it's pointing to ?
        //And maybe, using _ is actually a GOOD practice, as it creates a dummy observer for the tree, forcing it to actually emit sound
        //or in quantum mechanics terms, using _ creates an observer and forces the wave function to collapse
        //this would make a lot of sense ! lets investigate
        //https://stackoverflow.com/questions/48361537/why-do-underscore-prefixed-variables-exist
        // This seems interesting : RAII. You might want to have a variable exist for its destructor side effect, but not use it otherwise. It is not possible to use simply _ for this use-case, as _ is not a variable binding and the value would not be dropped at the end of the enclosing block as with variable binding.
        // https://doc.rust-lang.org/rust-by-example/scope/raii.html        //Ok not RAII, thats not whats going on here 

        let mut _x = 69;
    }
}

fn drop_pointer(s: String) {
    println!("Dropping {} !", s);
}

fn return_ownership(s: String) -> String {
    println!("Returning {}!", s);
    s   //We're technically modifying the input, but like, thats the return statement so thats fine. We still avoid having 2 pointers to the same thing.
        // This is so great, it avoids so much of the ugliness of C !
}

fn pointer_argument(n: &i32) {
    println!("{}", *n);
    //*n = *n - 1;        //obviously this wont work ; n is immutable and the pointer isnt declared as a pointer to mutable.
}

fn pointer_to_mutable_argument(n: &mut i32) {
    *n = *n - 1;
    println!("{}", n);
}
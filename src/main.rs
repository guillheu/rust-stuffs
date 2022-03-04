mod print;
mod variables;
mod types;
mod strings;
mod assert;
mod tuple;
mod arrays;
mod pointers;

fn main() {
    print::run();
    variables::run();
    types::run();
    strings::run();
    assert::run();
    tuple::run();
    arrays::run();
    pointers::run();
}

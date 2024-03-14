// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a = [
        2, 3, 4, 2, 3, 1, 3, 1, 22, 3, 1, 232, 3, 2, 3, 2, 23, 23, 34, 423, 423, 42, 34, 234, 2434,
        23, 423, 423, 423, 4, 23, 423, 423, 423, 4, 324, 34, 3, 43, 4, 34, 3, 43, 43, 4, 34, 34,
        34, 34, 3, 43, 43, 43, 43, 242, 34, 234, 23, 423, 423, 2, 3, 23, 23, 2, 3, 23, 23, 23, 23,
        2, 32, 3, 23, 2, 32, 32, 32, 3, 23, 21, 32, 312, 312, 312, 3, 123, 123, 123, 12, 312, 312,
        3, 123, 12, 312, 312, 3, 123, 12, 321, 312, 312, 3,
    ];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}

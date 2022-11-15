fn main() {
    // Variable whose length is known at compile time is stored in stack memory
    // Variable whose length is not known at compile time are stored in heap memory


    let str1 = "Hello"; // stack
    let str2 = String::from("Kunal"); // Heap
    /*
    "Hello" is stored in stack memory
    It is mutable but it's not growable that means we can't change it's lenght
    & -> borrowed string
    */

    /*
    "Kunal" is stored in heap memory pointed by str2
    str2 is a pointer stored in stack memory
    It is mutable as well as growable
    String -> owned string
    */

    /*
    Ownership Rules
    1. Variable is the owner of the value
    2. Every value can have one owner at a time
    3. When owner goes out of scope, compiler calls the drop method
    */

    let a = 10;
    let b = a; // deep Copy

    /*
    Rust creates deep copy of the values stored on stack memory 
    */

    let s1 = String::from("Kunal");
    let s2 = s1; // move
    /*
    Rust creates a shalow copy for the values stored in the heap memory
    But one value cannot have more than one owner at a time
    so rust drops the orignal varaible and new variable becomes the owner of value
    This is move
    */

    /*
    Pass reference of an value in heap in rust so that ownership don't transfer
    */

    let str_concat = String::from("Hello") + "World";

        /*
        We can only add borrowed string to owned string as it is growable and we can directly append another string to it.
        We cant' add something to borrowed as it is not growable
        We can't add two owned string as the resultant can't own two values
        */

    let str_copy = str_concat.clone(); // clone() creates deep copy of values in heap

    // tuple
    let myTup = (1,2,String::from("Hello"), "Hi", true);

    println!("{}",myTup.2);

    // borowed -> owned
    let bo = "Kunal".to_owned();

    // owned -> Borrowed 
    let ob = String::from("Kuna;").as_str();
}

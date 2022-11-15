fn main() {
    let string: &str = "Kunal";
    let slice1 = &string[..]; // used to access protions of data stored in a data type
    let slice2 = &string[1..3];
    let slice3 = &string[1..=3];
    println!("{}", slice1);
    println!("{}", slice2);
    println!("{}", slice3);

    // unit type - it's void type
    // let void: () = ();

    // panic!("I want to terminate program");
    let sum = add(10, 11);
    println!("sum {sum}");

    // only_panic();
    // only_unimplemented();

    let a = 9;

    if a > 20 {
        if a < 30 {
            println!("Big Number");
        }else {
            println!("Very Big Number");
        }
    }else if a < 10 {
        println!("Small number");
    }else {
        println!("Supply another number");
    }


    let a = 1;
    match a {
        1 => println!("This is 1"),
        12 => println!("This is 12"),
        _ => println!("supply another")

    }

    // strings
    let str1: &str = "Kunal"; // string with fixed length stored in stack with type as string pointer

    let str2 = String::from("Kunal"); // String made from string object
    /* 
    let str2: str = "Kunal";
    string whose length is not known at compile time is stored in heap memory
    by using smart boxed pointer Box<str>
    & used to convert Box<str> to &str
    
    */
    let str3: Box<str> = "Kunal".into();
    println!("Boxed string: {}", str3);

    // loops
    // loop {
    //     println!("This is an infiite loop");
    // }

    let mut i = 0;
    while i < 10 {
        println!("{i}");
        i += 1;
    }

    for i in 1..10 {
        println!("{i}");
    }

    let range = 1..10;
    println!("range: {:?}", &range);

    // expressions
    let x = 1;
    let y = {
        let x_sq = x * x;
        let x_cu = x_sq * x;
        x_cu + x_sq + x
    };

    println!("Y is {y}");
}

fn add (a:i32, b:i32)-> i32 {
    // println!("Hello Function")
    let sum = a+b;
    sum // we can return either putting return statement or by putting the value without semicolon
    // return sum;

    
}

fn only_panic() -> ! {
    panic!("Hi I panicked")
}

fn only_unimplemented() {
    unimplemented!("Hi I am not implemented")
}
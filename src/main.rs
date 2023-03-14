// Goals:
// How to generate Result
// How to handle Result
// How to handle Option
// --------------------------

// Create a function that takes bool as in input
// and returns an Ok<i32> or Err<i32>
// if input is true, then return Ok, else return Err
fn get_result(yes : bool) -> Result<i32,i32>
{
    if yes {
        return Ok(100);
    }
    return Err(333);
}

// Create a function that returns a result using the "?" operator
fn get_result2(yes : bool) -> Result<i32,i32> {
    let x = get_result(yes)?;
    return Ok(x+1000);
}

// Create function that returns an Option<bool>
// The function must return true, false or None
fn get_option(x : i32) -> Option<bool> {
    let y = x&3;
    match y {
        0 => None,
        1 => Some(true),
        2 => Some(false),
        _ => todo!(),
    }
}

// Demonstrate how to use unwrap_or with a Result<>
fn demo1() {
    println!("demo1 ----- unwrap_or() ----- ");
    for val in vec![true, false] {
        let x = get_result(val);
        println!("input is {0:5} is_ok()          -> {1}",val,x.is_ok());
        println!("         {0:5} is_err()         -> {1}","",x.is_err());
        println!("         {0:5} unwrap_or(999)   -> {1}","",x.unwrap_or(999));
    }
}

// Demonstrate how to get at a result using is_ok(), is_error(), and unwrap() methods
fn demo2() {
    println!("demo2 ----- is_ok(), is_err(), etc. ----- ");
    for val in vec![true, false] {
        let x = get_result(val);
        println!("input is {0:5} is_ok()  -> {1}",val,x.is_ok());
        println!("         {0:5} is_err() -> {1}","",x.is_err());
        if x.is_ok() {
            println!("         {0:5} unwrap() -> {1}","",x.unwrap());
            println!("         {0:5} ok()     -> {1:?}","",x.ok());
            println!("         {0:5} err()    -> {1:?}","",x.err());
        }
        if x.is_err() {
            println!("         {0:5} err()    -> {1:?}","",x.err());
            println!("         {0:5} ok()     -> {1:?}","",x.ok());
        }
    }

}

// Demonstrate how to handle Result with a match statement
fn demo3() {
    println!("demo3 ----- match() ----- ");
    for val in vec![true, false] {
        let x = get_result(val);
        println!("input is {0:5} is_ok()          -> {1}",val,x.is_ok());
        println!("         {0:5} is_err()         -> {1}","",x.is_err());
        match x {
            Ok(_x) => {
                println!("         {0:5} Ok() x.unwwrap() -> {1}","",x.unwrap());
                println!("         {0:5} Ok(_x) _x        -> {1}","",_x);
                },
            Err(_x) => {
                println!("         {0:5} Err()->err()    -> {1:?}","",x.err());
                println!("         {0:5} Err(_x) _x      -> {1}","",_x);
            }

        }
    }
}

// Demonstrate unwrapping with "?"
fn demo4() {
    println!("demo4 ----- unwrap with ?  ----- ");
    for val in vec![true, false] {
        let x = get_result2(val);
        println!("input is {0:5} is_ok()          -> {1}",val,x.is_ok());
        println!("         {0:5} is_err()         -> {1}","",x.is_err());
        match x {
            Ok(_x) => {
                println!("         {0:5} match x.unwwrap() -> {1}","",x.unwrap());
                println!("         {0:5} match _x          -> {1}","",_x);
                },
            Err(_x) => {
                println!("         {0:5} match x.err()    -> {1:?}","",x.err());
                println!("         {0:5} match _x         -> {1}","",_x);
            }
        }
    }
}

// Demonstrate how to handle an Option()
fn demo5() {
    println!("demo5 ----- Option ----- ");
    for val in vec![0,1,2] {
        let x = get_option(val);
        println!("{0}: x={1:?}",val,x);
    }
}

fn main() {
    demo1();
    demo2();
    demo3();
    demo4();
    demo5();
}

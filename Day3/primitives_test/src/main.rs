fn main() {
    // // Decalaring a tuple
    // let tup: (u8, i32, char) = (8, -2342, 'x');

    // // Destructuring
    // let (a,b,c) = tup;

    // // retrive tuple values using indices tup.0...tup.2 etc
    // println!();
    // println!("Stored values of tup: a:{} b:{} c:{}", tup.0, tup.1, tup.2);
    // println!("Stored values of tup: a:{} b:{} c:{}", a, b, c);

    // let t1 = (8, 'a', 0.5);
    // let t2 = ('b', t1);

    // println!("t2 contains: {:#?}, {} {:?}", t2, t2.0, t2.1);
    println!();


    // Unsigned 8-bit Integer (Positive ints only) array consisting 5 elements
    let arr1:[u8; 5] = [2, 3, 5, 7, 10];

    // Creates a char array and initializes all indices to 'X'
    let arr2:[char; 5] = ['X'; 5]; 

    // Prints array to console. {:#?} --> debug flag(:?) with prettify(#) makes the output prettier
    println!("Array 1 contains: {:#?}", arr1);
    println!("Array 2 contains: {:?}", arr2);

    println!();

    println!("Length of Array 1: {}", arr1.len());
    /* Using std::mem module we can check the array size allocation in stack with size_of_val function.
       array is passed into the function as a reference (&arr1). I'll share more on this (Borrowing) later.
    */
    println!("Memory allocated to Array 1 (u8) in stack- {} bytes", std::mem::size_of_val(&arr1));
    println!("Memory allocated to Array 2 (char) in stack- {} bytes", std::mem::size_of_val(&arr2));
    println!();

}

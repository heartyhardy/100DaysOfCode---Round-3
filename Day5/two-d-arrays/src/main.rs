fn main() {
    
    // Unintialized array
    let arr_2d:[[u8;2];3];

    // This line will not compile as it is not initialized
    //println!("{:?}", arr2d);
    
    // Creates a 2D array consisting 3 Rorws and 2 Columns
    let arr_2d:[[u8; 2];3] = [[0,1],[0,1], [0,1]];

    println!("2D array: {:?} ",arr_2d);

    /* Another method of initializing
        [[1;2]...] means we initialize both elements of the colum to 1
        [...;3] means all the 3 rows will use above initializing setup
    */
    let arr_2d:[[u8; 2];3] = [[1;2];3];

    // Prints the array with Debug flag with prettify
    println!("2D array: {:#?} ",arr_2d);
    
    // Prints the array to console using a nested for loop
    // i = Rows = 3
    // j = Cols = 2
    // arr2d.len() =length / the row count of the array
    // arry2d[i] = directs to each row defined by i
    // array2d[i][j] = directs to j column in i row
    for i in 0..arr_2d.len(){
        for j in 0..arr_2d[i].len(){
            print!(" {} ", arr_2d[i][j]);
        }
        println!();
    }
    
    // Since we know the array lengths we can also write this as...
    for i in 0..3{ // 0, 1, 2
        for j in 0..2{ // 0 , 1
            print!(" {} ", arr_2d[i][j]);
        } 
        println!();
    }

}

fn main() {

    let array : [i32; 4] = [ 5,6,7,2];

    let _first_ele: i32 = array[1];

    println!("This is the value of the first_ele: {}", _first_ele);

    //access the first element using index notation
    //  

    let tuple : (u32, i32, bool) = ( 21, -31, true );
    let _first_element: u32 = tuple.0;

    println!("This is the value of the first eleemt: {}", _first_element);
}

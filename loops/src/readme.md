Loops

Used to execute a block of code forever

Example 
loop{
    println!("I loop forever");
    break;
}


While 

Conditional runs a blocks until a condition is met

Example

let mut number  = 3

while number != 0{
    println!("{}!", number);
    number -= 1;
}

println!("Off to the Moon")



For 

They iterate over element in a collection like an array, each loop as pass extracts a value.


Example

let a  =  [1,23,49,44,43,3]

for element in a.iter(){
    println!("The value is: {}", element)
}
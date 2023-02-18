

Using Panic is the simplest way to handle unrecoverable errors or states

Example

fn main(){
    panic!("Farewell")
}



Using Option enum is the simplest way to handle recoverable errors to handle the absence of values

fn main(){

    enum Option<T> { //The type T is generic and associated with the Some variant
        None, // None indicates that no element was found
        Some(T) // means that an element of type T was found
    }
}

Using Result enum for returning and propagating errors   .It used when a function fails or when an operation you would expect to work doesnt and used when
failures are expected.

fn main(){
    enum Result <T , E>{
        Ok(T), // this variant represents a success and contains a value
        Err(E) // this varant represents an error
    }
}

// The result enum is used for input/output operations (I/O)

- Parsing Strings
- File Access
- Data Validation


Unwrap and Expect are helper methods for the Result type

Unwrap returns the value inside the Ok Variant and returns a panic! macro for the Err variant.

Expect returns a value or calls the panic! macro with a detailed error message.



? operator -  it is similar to a match operator


For Result type
- Unwraps the value if Ok variant
- Returns an error if Err variant
  
For Option type
- Return a vlaue is with the Some variant
- Returns nothing for the None variant



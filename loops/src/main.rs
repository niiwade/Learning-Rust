fn main() {
    

    //loop

    let mut i = 1;


    let sum = loop {
        i*=2;
        if i  > 100{
            break i;
        } 
    };
    assert_eq!(sum, 128);
    println!("{}",sum );


    //while 

    let mut counter = 0 ;

    while counter < 10{
        println!("Hello");
        counter = counter + 1;
    }


    //for loop


    for item in 0..5{
        println!("{}", item *2)
    }



}

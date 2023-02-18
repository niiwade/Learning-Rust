

fn main() {
    

    let price   = 21;

    if price <  3 {
        println!("The price is less :{}", price);
    }

    let  adult  = 19;
    let _child = if adult <= 18{
        println!("Youa are a child")
    }else {
        println!("You are an adult")
    };




    if 5 == 2{
        println!("This can multiple")
    }else if  2 == 5 {
        println!("This can also multiple 5")
    }else{
        println!("this cannot multiple")
    };


    let  ages  =  20;

    match ages{
        10 => println!("Ten"),
        15 => println!("fifteen"),
        17..=20 => println!("seveneteen to twenty"),
        _ => println!("Not a valid age")
    };


}

// declaring variables in rust

//variables in rust are immutable by default  but you can use the mut keyword to make an variable mutable

fn main(){
    let _x = 5;   
}


//constant are values that cannot change

fn main1(){
    const SCORE_LIMIT: u32  = 100;
}


//shadowing a variable -  declare a new variable with the same name as the previous creating a new binding
 fn main2(){
    let y = 6
    let y = y + 1
 }
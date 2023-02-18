

enum WebEvent{
    PageLoad(String),
    PageUnLoad(String),
    KeyPress(char),
    Paste(String),
    Click{x:i64 , y:i64},
    LoadTime(u32)
}


enum Option<T>{
    Some(T),
    None
}


fn main() {


    let quit = WebEvent::KeyPress('q');

    let something = Some(1);

   

    // println!("Hello, world!");
}

fn main() {
    last_char(String::from("Heilos"));
    
}

fn last_char(string: String) -> char{

 if string.is_empty(){
    return '1';
 }

 string.chars().next_back().unwrap()
}

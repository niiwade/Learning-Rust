

struct Student {
    name: String,
    class: String,
    age: u32,
    course: String
    
}


//tuple struct

struct Point2D (u32,u32);


fn main() {
   
    let student1 = Student {
        name: String::from("wade"),
        class: String::from("Level 100"),
        age: 22,
        course: String::from("Science")
    };

    println!("Student details for  Student 1 are :{}", student1.name);
    println!("Student details for  Student 1 are :{}", student1.class);
    println!("Student details for  Student 1 are :{}", student1.age,);
    println!("Student details for  Student 1 are :{}", student1.course,);


    let origin = Point2D(12,28);

    println!("The Points in the plane Point2D are:{:?}x and {:?}y", origin.0, origin.1);


    // destruturing a tuple struct

    let  Point2D(x,y) = origin;
    println!("The Points in the plane Point2D are:{:?}x and {:?}y", x, y)



}



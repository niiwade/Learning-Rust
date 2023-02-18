

A struct is a data type for grouping related values.

- A type is composed of other types
- It can contain different types
- There are classic Structs - They are used commonly , each field has a name and a type
- Tuple Structs -   They are similar to classic structs but their fields has no names
- and Unit Structs - They have no fields, they are similar to thre unit type

Example -- 
 struct Car{
    make: String,
    model: String,
    year: u32
 }
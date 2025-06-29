#[derive(Debug)]
struct  Person {
    name: String,
    age: u8
}

#[derive(Debug)]
struct  Parents<'a, 'b> {
    father: &'a Person,
    mother: &'b Person
}

impl<'a, 'b> Parents<'a, 'b> {
    fn new(father: &'a Person, mother: &'b Person) -> Parents<'a, 'b> {
        Parents{father, mother}
    }
}

fn main() {
    let mut taro = Person {
        name: String::from("taro"),
        age: 54
    };
    let hanako = Person {
        name: String::from("hanako"),
        age: 48
    };

    let sato: Parents = Parents { father: &taro, mother: &hanako };

    // Person { name: "taro", age: 10 }
    println!("{:?}", taro);  

    // Parents { father: Person { name: "taro", age: 54 }, mother: Person { name: "hanako", age: 48 } }
    println!("{:?}", sato);  
    
    taro.age = 30;

    let hi = Parents::new(&taro, &hanako);
    println!("{:?}", hi);
}

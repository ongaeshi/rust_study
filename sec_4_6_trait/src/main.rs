#[derive(Debug)]
struct  Person {
    name: String,
    age: u8
}

#[derive(Debug)]
struct  Parents {
    father: Person,
    mother: Person
}

fn main() {
    let taro = Person {
        name: String::from("taro"),
        age: 54
    };
    let hanako = Person {
        name: String::from("hanako"),
        age: 48
    };

    // ここで所有権が移動する
    let sato: Parents = Parents { father: taro, mother: hanako };

    // Person { name: "taro", age: 10 }
    // println!("{:?}", taro);  

    // Parents { father: Person { name: "taro", age: 54 }, mother: Person { name: "hanako", age: 48 } }
    println!("{:?}", sato);  
}


struct First {
    prop: String, 
}

fn main() {
    
    let instance = First { 
        prop: String::from("Hello World!")
    };

    println!("Phrase: {}", instance.prop)
}
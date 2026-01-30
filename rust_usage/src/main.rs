use dictionary::*;
fn main() {
    println!("\nRust program\n");

    let mut d = Dictionary::new();

    d.insert(10, MyString::from_str("ten"));
    d.insert(20, MyString::from_str("twenty"));
    d.insert(67, MyString::from_str("six-seven"));

    println!("has 67? {}", d.contains(10));
    println!("has 42? {}", d.contains(42));

    if let Some(s) = d.get(67) {
        println!("d1[67] = {} (len={})", s.as_str(), s.len());
    } else {
        println!("no 67");
    }

    println!("remove 67");
    d.remove(67);
    println!("has 67? {}", d.contains(67));

    let mut d2 = dict!({1,"a"}, {2,"abc"} , {3, "g"});

    println!("has d2 {}", d2.contains(2));

    if let Some(s) = d2.get(1) {
        println!("d2[1] = {}", s.as_str());
    }

    d2.insert(4, MyString::from_str("rust"));
    if let Some(s) = d2.get(4) {
        println!("d2[4] = {}", s.as_str());
    }

    for i in 1000..1010 {
        d2.insert(i, MyString::from_str(&format!("number {}", i)));
    }

    for i in 1000..1010 {
        if let Some(x) = d2.get(i){
            println!("d2[{}] = {}", i, x.as_str());
        }
    }
}

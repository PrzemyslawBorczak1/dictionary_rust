use dictionary::*;
fn main() {
    // 1) Ręczne tworzenie i użycie
    let mut d = Dictionary::new();

    d.insert(10, SimpleString::new_from_str("ten"));
    d.insert(20, SimpleString::new_from_str("twenty"));
    d.insert(5, SimpleString::new_from_str("five"));

    println!("contains 10? {}", d.contains(10));
    println!("contains 99? {}", d.contains(99));

    if let Some(s) = d.get(20) {
        println!("key 20 -> '{}' (len={})", s.as_str(), s.len());
    } else {
        println!("key 20 not found");
    }

    d.remove(10);
    println!("after remove 10, contains 10? {}", d.contains(10));

    // 2) Użycie makra dict!
    let mut d2 = dict!(
        1 => "hello",
        2 => "world",
        3 => "!"
    );

    println!("d2 contains 2? {}", d2.contains(2));
    if let Some(s) = d2.get(1) {
        println!("d2[1] = {}", s.as_str());
    }

    d2.insert(4, SimpleString::new_from_str("rust"));
    if let Some(s) = d2.get(4) {
        println!("d2[4] = {}", s.as_str());
    }

    // 3) Szybki test większej liczby elementów
    for i in 100..110 {
        d2.insert(i, SimpleString::new_from_str(&format!("number {}", i)));
    }

    for i in 100..110 {
        let s = d2.get(i).unwrap();
        println!("d2[{}] = {}", i, s.as_str());
    }

    println!("Done.");
}

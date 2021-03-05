fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // s is no longer valid, cause takes_ownership function argument tool ownershipt of s value
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
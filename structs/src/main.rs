struct User{
    active: bool,
    name: String,
    age: u64,
    email: String

}
fn main() {
let mut zuck = User{
    active: true,
    name: String::from("ZUCK"),
     age: 67,
     email: String::from("ZUCK@gmail.com")
};
}

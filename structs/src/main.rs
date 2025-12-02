struct User{
    active: bool,
    name: String,
    age: u64,
    email: String

}
fn main() {
let zuck = User{
    active: true,
    name: String::from("ZUCK"),
    age: 67,
    email: String::from("ZUCK@gmail.com")
};
    zuck.email = String::from("anotheremail@example.com");
}

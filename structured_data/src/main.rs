fn basic_struct() {
    println!("Hello, world!");
    let user_name = String::from("shekhar.jaigaon@gmail.com");
    let user1 = User{
        active: true,
        username: &user_name,
        email: &user_name,
        sign_in_count: 1,
    };
    println!("User: {} ({})", user1.username, user1.email);
}

struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rect{
    len: u32,
    wid: u32,
}

impl Rect{
    fn New(len: u32, wid:u32) -> Self {
        Self{
            len: len,
            wid: wid,
        }
    }

    fn GetArea(&self) -> u32{
        self.len * self.wid
    }
}

impl Rect{
    fn can_hold(&self, other: Rect) -> bool {
        self.len > other.len && self.wid > other.wid
    }
}

fn main() {
    // code for basic struct demonstration
    basic_struct();

    // code to print structure and other stuffs
    let rec1 = (30, 50);

    println!(
        "the area of rec: {}", rec1.0 * rec1.1
    );

    let rec2 = Rect{
        len: 12, wid: 45
    };
    println!("the aread of rect from struct is: {rec2:?} area: {}", rec2.len*rec2.wid);
    dbg!(&rec2);

    // custom Constructor
    let rec3 = Rect::New(12,23);
    dbg!(&rec3);

    // Methods declaration and usages
    println!("get area: {}", rec3.GetArea());

    println!("can hold: {}", rec3.can_hold(Rect::New(10,20)));
}


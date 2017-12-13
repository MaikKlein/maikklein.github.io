trait Say {
    fn say(&self);
}

impl Say for u32 {
    fn say(&self) {
        println!("I am an u32 {}", *self);
    }
}

fn main() {
    1.say();
}

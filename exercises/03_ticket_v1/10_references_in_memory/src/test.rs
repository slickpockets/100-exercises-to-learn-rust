

fn main() {
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }
    use std::mem::size_of;
    assert_eq!(size_of::<&u16>(), 8);
    assert_eq!(size_of::<&mut u64>(), 8);
    assert_eq!(size_of::<&Ticket>(), 8);

}
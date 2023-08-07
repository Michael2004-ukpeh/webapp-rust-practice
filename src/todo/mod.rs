pub mod enums;
pub mod structs;
pub mod traits;
pub use enums::*;
pub use structs::{done::Done, pending::Pending};
pub use traits::*;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn todo_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
    }
}

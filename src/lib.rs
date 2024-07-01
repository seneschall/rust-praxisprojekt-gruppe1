
pub mod traits;

pub mod graph;
pub mod wt;

// Enums
#[derive(PartialEq, Debug)]
pub enum Edit<T> {
    Add(T),
    Delete(T),
}


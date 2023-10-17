extern crate chrono;

mod time_related;
pub use time_related::time_related_fn;

fn main() {
   time_related_fn(); 
}

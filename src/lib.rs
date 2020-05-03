extern crate uuid;
extern crate mime;
extern crate iso639_1;
extern crate chrono;

// Principal interface
pub mod language;
pub mod server;
pub mod extension;
pub mod problem;
pub mod tag;
pub mod answer;
pub mod challenger;

// API interface
pub mod api;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::language::{Mfunction, Limitation};
        assert_eq!(Mfunction::new("2X*3+100123-").evaluate(3).unwrap(), 100114);
        assert_eq!(Mfunction::new("+++++++----////XXXX").evaluate(0).is_err(), true);
    }
}

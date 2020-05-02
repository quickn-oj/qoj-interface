extern crate uuid;
extern crate mime;
extern crate iso639_1;
extern crate chrono;

mod language;
mod server;
mod extension;
mod problem;
mod tag;
mod answer;
mod challenger;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::language::{Mfunction, Limitation};
        assert_eq!(Mfunction::new("2X*3+100123-").evaluate(3).unwrap(), 100114);
        assert_eq!(Mfunction::new("+++++++----////XXXX").evaluate(0).is_err(), true);
    }
}

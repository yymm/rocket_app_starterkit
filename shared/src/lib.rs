extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod settings;

#[cfg(test)]
mod tests {
    use crate::settings::Settings;
    #[test]
    fn it_works() {
        let cfg = Settings::new();
        dbg!(cfg);
    }
}

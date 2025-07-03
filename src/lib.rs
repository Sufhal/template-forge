pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod errors;
mod forge;
mod server;

pub use forge::forge::{Forge,ForgeConfig};
pub use forge::template::ForgeTemplate;
pub use handlebars;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

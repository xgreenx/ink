#[ink::contract]
mod contract {
    #[ink(storage)]
    pub struct Contract {}

    #[derive(
        Debug, PartialEq, Eq, parity_scale_codec::Encode, parity_scale_codec::Decode,
    )]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        Foo,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn constructor() -> Result<u8, Error> {
            Ok(5_u8)
        }

        #[ink(message)]
        pub fn message(&self) {}
    }
}

fn main() {}

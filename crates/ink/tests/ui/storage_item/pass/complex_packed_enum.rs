use ink::storage::traits::Storable;
use ink_prelude::{
    collections::{
        BTreeMap,
        BTreeSet,
    },
    vec::Vec,
};

#[derive(
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    parity_scale_codec::Encode,
    parity_scale_codec::Decode,
)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
enum Deep2 {
    #[default]
    None,
    A(u8),
    B(u16),
    C(u32),
    D(u64),
    E(u128),
    F(String),
    G {
        a: u8,
        b: String,
    },
    H((u16, u32)),
}

#[derive(Default, parity_scale_codec::Encode, parity_scale_codec::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
enum Deep1 {
    #[default]
    None,
    A(Deep2),
    B((Deep2, Deep2)),
    C(Vec<Deep2>),
    D(BTreeMap<Deep2, Deep2>),
    E(BTreeSet<Deep2>),
}

#[derive(Default, parity_scale_codec::Encode, parity_scale_codec::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
struct Contract {
    a: Deep1,
    b: Deep2,
    c: (Deep1, Deep2),
}

fn main() {
    let _: Result<Contract, _> = Storable::decode(&mut &[][..]);
}

// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    AutoStorableHint,
    Packed,
    StorableHint,
    StorageKey,
};
use core::{
    fmt::Debug,
    marker::PhantomData,
};
use ink_primitives::{
    Key,
    KeyComposer,
};

/// The private trait helping identify the [`AutoKey`] key type.
trait KeyType {
    /// It is `true` for [`AutoKey`] and `false` for [`ManualKey`].
    /// It helps the [`ResolverKey`] select between the user-specified (left key)
    /// and the auto-generated (right key) keys.
    const IS_AUTO_KEY: bool;
}

/// Auto key type means that the storage key should be calculated automatically.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct AutoKey;

impl StorageKey for AutoKey {
    const KEY: Key = 0;
}

impl KeyType for AutoKey {
    const IS_AUTO_KEY: bool = true;
}

impl Debug for AutoKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AutoKey")
            .field("key", &<Self as StorageKey>::KEY)
            .finish()
    }
}

/// Manual key type specifies the storage key.
#[derive(Default, Copy, Clone, Eq, PartialEq, PartialOrd)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ManualKey<const KEY: Key, ParentKey: StorageKey = ()>(
    PhantomData<fn() -> ParentKey>,
);

impl<const KEY: Key, ParentKey: StorageKey> StorageKey for ManualKey<KEY, ParentKey> {
    const KEY: Key = KeyComposer::concat(KEY, ParentKey::KEY);
}

impl<const KEY: Key, ParentKey: StorageKey> KeyType for ManualKey<KEY, ParentKey> {
    const IS_AUTO_KEY: bool = false;
}

impl<const KEY: Key, ParentKey: StorageKey> Debug for ManualKey<KEY, ParentKey> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ManualKey")
            .field("key", &<Self as StorageKey>::KEY)
            .finish()
    }
}

/// Resolver key type selects between preferred key and autogenerated key.
/// If the `L` type is `AutoKey` it returns auto-generated `R` else `L`.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ResolverKey<L, R>(PhantomData<fn() -> (L, R)>);

impl<L, R> StorageKey for ResolverKey<L, R>
where
    L: StorageKey + KeyType,
    R: StorageKey + KeyType,
{
    /// If the left key is [`AutoKey`], then use the right auto-generated storage key.
    /// Otherwise use the left [`ManualKey`].
    const KEY: Key = if L::IS_AUTO_KEY { R::KEY } else { L::KEY };
}

impl<L, R> KeyType for ResolverKey<L, R>
where
    L: KeyType,
    R: KeyType,
{
    /// The right key is always an auto-generated key, the user can specify only the left key.
    /// So the left key defines the [`KeyType::IS_AUTO_KEY`] of the [`ResolverKey`].
    const IS_AUTO_KEY: bool = L::IS_AUTO_KEY;
}

type FinalKey<T, const KEY: Key, ParentKey> =
    ResolverKey<<T as StorableHint<ParentKey>>::PreferredKey, ManualKey<KEY, ParentKey>>;

// `AutoStorableHint` trait figures out that storage key it should use.
// - If the `PreferredKey` is `AutoKey` it will use an auto-generated key passed as generic
// into `AutoStorableHint`.
// - If `PreferredKey` is `ManualKey`, then it will use it.
impl<T, const KEY: Key, ParentKey> AutoStorableHint<ManualKey<KEY, ParentKey>> for T
where
    T: StorableHint<ParentKey>,
    <T as StorableHint<ParentKey>>::PreferredKey: KeyType,
    T: StorableHint<FinalKey<T, KEY, ParentKey>>,
    ParentKey: StorageKey,
{
    type Type = <T as StorableHint<FinalKey<T, KEY, ParentKey>>>::Type;
}

impl<P> super::storage::private::Sealed for P where P: scale::Decode + scale::Encode {}
impl<P> Packed for P where P: scale::Decode + scale::Encode {}

impl<P> StorageKey for P
where
    P: Packed,
{
    const KEY: Key = 0;
}

impl<P, Key> StorableHint<Key> for P
where
    P: Packed,
    Key: StorageKey,
{
    type Type = P;
    type PreferredKey = AutoKey;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates test to verify that the primitive types are packed.
    #[macro_export]
    macro_rules! storage_hint_works_for_primitive {
        ( $ty:ty ) => {
            paste::item! {
                #[test]
                #[allow(non_snake_case)]
                fn [<$ty _storage_hint_works>] () {
                    assert_eq!(
                        ::core::any::TypeId::of::<$ty>(),
                        ::core::any::TypeId::of::<<$ty as $crate::StorableHint<$crate::ManualKey<123>>>::Type>()
                    );
                }
            }
        };
    }
    mod arrays {
        use crate::storage_hint_works_for_primitive;

        type Array = [i32; 4];
        storage_hint_works_for_primitive!(Array);

        type ArrayTuples = [(i32, i32); 2];
        storage_hint_works_for_primitive!(ArrayTuples);
    }

    mod prims {
        use crate::storage_hint_works_for_primitive;
        use ink_primitives::AccountId;

        storage_hint_works_for_primitive!(bool);
        storage_hint_works_for_primitive!(String);
        storage_hint_works_for_primitive!(AccountId);
        storage_hint_works_for_primitive!(i8);
        storage_hint_works_for_primitive!(i16);
        storage_hint_works_for_primitive!(i32);
        storage_hint_works_for_primitive!(i64);
        storage_hint_works_for_primitive!(i128);
        storage_hint_works_for_primitive!(u8);
        storage_hint_works_for_primitive!(u16);
        storage_hint_works_for_primitive!(u32);
        storage_hint_works_for_primitive!(u64);
        storage_hint_works_for_primitive!(u128);

        type OptionU8 = Option<u8>;
        storage_hint_works_for_primitive!(OptionU8);

        type ResultU8 = Result<u8, bool>;
        storage_hint_works_for_primitive!(ResultU8);

        type BoxU8 = Box<u8>;
        storage_hint_works_for_primitive!(BoxU8);

        type BoxOptionU8 = Box<Option<u8>>;
        storage_hint_works_for_primitive!(BoxOptionU8);
    }

    mod tuples {
        use crate::storage_hint_works_for_primitive;

        type TupleSix = (i32, u32, String, u8, bool, Box<Option<i32>>);
        storage_hint_works_for_primitive!(TupleSix);
    }

    #[test]
    fn storage_key_types_works() {
        assert_eq!(<AutoKey as StorageKey>::KEY, 0);
        assert_eq!(<ManualKey<123> as StorageKey>::KEY, 123);
        assert_eq!(<ManualKey<0> as StorageKey>::KEY, 0);
        assert_eq!(<ResolverKey<AutoKey, AutoKey> as StorageKey>::KEY, 0);
        assert_eq!(
            <ResolverKey<AutoKey, ManualKey<123>> as StorageKey>::KEY,
            123
        );
        assert_eq!(
            <ResolverKey<ManualKey<456>, ManualKey<123>> as StorageKey>::KEY,
            456
        );
        assert_eq!(
            <ResolverKey<ManualKey<0>, ManualKey<123>> as StorageKey>::KEY,
            0
        );
    }
}

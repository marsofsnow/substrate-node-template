#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
    codec::{Decode, Encode},
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
    ensure,
};
use frame_system::{self as system, ensure_signed};

use sp_runtime::RuntimeDebug;

#[cfg(test)]
mod tests;

decl_event!(
    pub enum Event<T>
    where
        //<T as system::Config>::AccountId,
        <T as system::Config>::Hash,
        <T as balances::Config>::Balance,

        {
            NewInnerThing(u32,Hash,Balance),
            NewSuperThingByExistingInner(u32, u32, Hash, Balance),
            NewSuperThingByNewInner(u32, u32, Hash, Balance),
        }
);

pub trait Config: system::Config + balances::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

//  自定义结构体且是范型
#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct InnerThing<Hash, Balance> {
    number: u32,
    hash: Hash,
    balance: Balance,
}

type InnerThingOf<T> = InnerThing<<T as system::Config>::Hash, <T as balances::Config>::Balance>;
#[derive(Encode, Decode, Default, RuntimeDebug)]
pub struct SuperThing<Hash, Balance> {
    super_number: u32,
    inner_thing: InnerThing<Hash, Balance>,
}

decl_module! {
    pub struct Module<T:Config> for enum Call where origin:T::Origin{
        fn deposit_event()=default;

        #[weight=10_000]
        fn insert_inner_thing(origin,number:u32,hash: T::Hash, balance:T::Balance)->DispatchResult{
            let _ = ensure_signed(origin)?;
            let thing = InnerThing{number:number,hash:hash,balance:balance};
            <InnerThingsByNumbers<T>>::insert(number,thing);
            Self::deposit_event(RawEvent::NewInnerThing(number,hash,balance));

            Ok(())
        }

        #[weight = 10_000]
        fn insert_super_thing_with_existing_inner(origin, inner_number: u32, super_number: u32)->DispatchResult {
            let _ = ensure_signed(origin)?;
            let inner_thing = Self::inner_things_by_numbers(inner_number); //注意这里使用storge里面设置的get函数
            let super_thing = SuperThing {
                super_number,
                inner_thing: inner_thing.clone(),
            };
            <SuperThingsBySuperNumbers<T>>::insert(super_number, super_thing);
            Self::deposit_event(RawEvent::NewSuperThingByExistingInner(super_number, inner_thing.number, inner_thing.hash, inner_thing.balance));
            Ok(())
        }


        #[weight = 10_000]
        fn insert_super_thing_with_new_inner(origin, inner_number: u32, hash: T::Hash, balance: T::Balance, super_number: u32) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            let inner_thing = InnerThing {
                number: inner_number,
                hash,
                balance,
            };
            <InnerThingsByNumbers<T>>::insert(inner_number, inner_thing.clone());
            Self::deposit_event(RawEvent::NewInnerThing(inner_number, hash, balance));
            let super_thing = SuperThing {
                super_number,
                inner_thing,
            };
            <SuperThingsBySuperNumbers<T>>::insert(super_number, super_thing);
            Self::deposit_event(RawEvent::NewSuperThingByNewInner(super_number, inner_number, hash, balance));
            Ok(())
        }


    }
}

decl_error! {
    pub enum Error for Module<T:Config>{
        AlreadInitialized,
        InsufficientFunds,
    }
}

decl_storage! {
    trait Store for Module<T: Config> as NestedStructs {
        InnerThingsByNumbers get(fn inner_things_by_numbers):map hasher(blake2_128_concat) u32 => InnerThingOf<T>;

        SuperThingsBySuperNumbers get(fn super_things_by_super_numbers):  map hasher(blake2_128_concat) u32 => SuperThing<T::Hash, T::Balance>;

    }
}

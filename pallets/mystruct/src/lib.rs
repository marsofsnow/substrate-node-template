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

decl_module! {
    pub struct Module<T:Config> for enum Call where origin:T::Origin{
        fn deposit_event()=default;

        #[weight=10_000]
        // 为了使token可用，账户必须初始化它，初始化token有很多方法，如进行创世（genesis）配置、声明存储过程、lockdrop等。
        // 这里使用一个非常简单的方法，第一个调用init函数的账户将收到所有token，类似于在EOISO区块链上发行通证时调用issue Action。
        fn helloword(origin)->DispatchResult{
            let sender = ensure_signed(origin)?;
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

decl_storage! {
    trait Store for Module<T: Config> as Token {
        //每个持有token的账户在该映射中均表示为键（key），其值（value）就是其持有的token数量。
        pub Balances get(fn get_balance):map hasher(blake2_128_concat) T::AccountId=>u64;
        // TotalSupply设置token的总供应量，
        pub TotalSupply get(fn total_supply):u64=21000000;
        // Init跟踪token是否已初始化。
        Init get(fn is_init):bool;
    }
}

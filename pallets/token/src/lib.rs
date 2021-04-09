#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod tests;

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Config>::AccountId,
    {
        Initialized(AccountId),
        Transfer(AccountId, AccountId, u64),
    }
);

pub trait Config: system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_module! {
    pub struct Module<T:Config> for enum Call where origin:T::Origin{
        fn deposit_event()=default;

        #[weight=10_000]
        // 为了使token可用，账户必须初始化它，初始化token有很多方法，如进行创世（genesis）配置、声明存储过程、lockdrop等。
        // 这里使用一个非常简单的方法，第一个调用init函数的账户将收到所有token，类似于在EOISO区块链上发行通证时调用issue Action。
        fn init(origin)->DispatchResult{
            let sender = ensure_signed(origin)?;
            // 先检查执行条件，以确保token只被初始化一次，然后修改StorageMap中的相关数据并触发Initialized事件。
            ensure!(!Self::is_init(),<Error<T>>::AlreadInitialized);
            <Balances<T>>::insert(&sender,Self::total_supply());
            Init::put(true);
            Self::deposit_event(RawEvent::Initialized(sender));
            Ok(())

        }

        #[weight=10_000]
        fn transfer(origin,to: T::AccountId,value:u64)->DispatchResult{
            let sender = ensure_signed(origin)?;
            let sender_balance = Self::get_balance(&sender);
            let receiver_balance = Self::get_balance(&to);
            let updated_from_balance = sender_balance.checked_sub(value).ok_or(<Error<T>>::InsufficientFunds)?;
            let updated_to_balance =   receiver_balance.checked_add(value).expect("Entire supply fits in u64;qed");

            <Balances<T>>::insert(&sender,updated_from_balance);
            <Balances<T>>::insert(&to,updated_to_balance);
            Self::deposit_event(RawEvent::Transfer(sender,to,value));

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
    trait Store for Module<T: Config> as Token {
        //每个持有token的账户在该映射中均表示为键（key），其值（value）就是其持有的token数量。
        pub Balances get(fn get_balance):map hasher(blake2_128_concat) T::AccountId=>u64;
        // TotalSupply设置token的总供应量，
        pub TotalSupply get(fn total_supply):u64=21000000;
        // Init跟踪token是否已初始化。
        Init get(fn is_init):bool;
    }
}

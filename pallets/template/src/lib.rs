#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
    traits::Randomness,
};
use frame_system::ensure_signed;
use sp_runtime::print;
use sp_std::prelude::*;

/// 随机数
use codec::Encode;
use sp_core::H256;
use sp_std::vec::Vec;
/// 随机数

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const MAX_MEMBERS: usize = 16;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type RandomnessSource: Randomness<H256>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Config> as TemplateModule {
        // Learn more about declaring storage items:
        // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
        Something get(fn something): Option<u32>;
        /*
        这段代码包含的要点有：
        SimpleMap：存储数据的HashMap名称；
        get(fn simple_map)：一个getter函数，返回从哈希表中获取的值；
        map hasher(blake2_128_concat)：声明使用的哈希表类型，这里使用的是blake2_128_concat hasher；
        T::AccountId => u32：哈希表的键和值的数据类型，这里是键的类型是AccountId，值的类型是u32；

        Substrate提供三种哈希表类型：blake2_128_concat、twox_64_concat、identity，一般使用blake2_128_concat即可。
        */
        SimpleMap get(fn simple_map):map hasher(blake2_128_concat) T::AccountId=>u32;


        Members get(fn members):Vec<T::AccountId>;

        Nonce get(fn nonce):u32;

    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, AccountId),
        EmitInput(AccountId, u32),
        MemberAdded(AccountId),
        MemberRemoved(AccountId),
        RandomnessConsumed(H256, H256),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Config> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,

        AlreadyMember,
        NotMember,
        MembershipLimitReached,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        #[weight=10_000]
        fn consume_randomness(origin)->dispatch::DispatchResult{
            let _=ensure_signed(origin)?;
            let subject = Self::encode_and_update_nonce();
            let random_seed = T::RandomnessSource::random_seed(); // random_seed方法不带任何参数，返回一个随机种子，该种子在每个区块更改一次，如果在同一个区块中调用此方法两次会得到相同的结果
            let randon_result = T::RandomnessSource::random(&subject);// random方法采用字节数组&[u8]类型作为参数，与随机种子一起使用以计算最终的随机值，这种方式可以在同一个区块中获得不同的随机值。
            Self::deposit_event(RawEvent::RandomnessConsumed(random_seed,randon_result));
            Ok(())
        }


        #[weight = 10_000]
        pub fn add_member(origin)->dispatch::DispatchResult{
            let new_member = ensure_signed(origin)?;
            let mut members = Members::<T>::get();
            ensure!(members.len()<MAX_MEMBERS,Error::<T>::MembershipLimitReached);
            match members.binary_search(&new_member){
                Ok(_)=> Err(Error::<T>::AlreadyMember.into()),
                Err(index)=>{
                    members.insert(index,new_member.clone());
                    Members::<T>::put(members);
                    Self::deposit_event(RawEvent::MemberAdded(new_member));
                    Ok(())

                }
            }
        }

        #[weight = 10_000]
        fn remove_member(origin) -> dispatch::DispatchResult {
            let old_member = ensure_signed(origin)?;
            let mut members = Members::<T>::get();

            match members.binary_search(&old_member) {
                Ok(index) => {
                    members.remove(index);
                    Members::<T>::put(members);
                    Self::deposit_event(RawEvent::MemberRemoved(old_member));
                    Ok(())
                },
                Err(_) => Err(Error::<T>::NotMember.into()),
            }
        }



        #[weight = 10_000]
        pub fn say_hello(origin) -> dispatch::DispatchResult {
            let caller = ensure_signed(origin)?;
            print("Hello World");
            debug::info!("Request sent by: {:?}", caller);
            Ok(())
        }

        #[weight = 10_000]
        fn set_single_entry(origin,entry:u32)->dispatch::DispatchResult {
            let user = ensure_signed(origin)?;
            <SimpleMap<T>>::insert(&user,entry);
            Self::deposit_event(RawEvent::EmitInput(user,entry));//  触发事件
            Ok(())
        }





        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let who = ensure_signed(origin)?;

            // Update storage.
            Something::put(something);

            // Emit an event.
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            // Return a successful DispatchResult
            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn cause_error(origin) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match Something::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    Something::put(new);
                    Ok(())
                },
            }
        }
    }
}

impl<T: Config> Module<T> {
    fn encode_and_update_nonce() -> Vec<u8> {
        let nonce = Nonce::get();
        Nonce::put(nonce.wrapping_add(1));
        nonce.encode()
    }
}

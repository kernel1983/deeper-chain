
#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::{pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	// use sp_core::H256;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn files)]
	pub type Files<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Vec<Vec<u8>>
	>;

	#[pallet::storage]
	#[pallet::getter(fn chunks)]
	pub type Chunks<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		(Vec<u8>, Vec<u8>)
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	// #[pallet::metadata(T::AccountId = "AccountId")]
	pub enum Event<T: Config> {
		FileCreated(T::AccountId, Vec<u8>),
		FileRemoved(T::AccountId, Vec<u8>),
	//    ClaimTraslated(T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		FileAlreadyExist,
		// ClaimNotExist,
		// NotClaimOwner,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_file(
			origin: OriginFor<T>,
			filehash: Vec<u8>,
			chunks: Vec<Vec<u8>>
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Files::<T>::contains_key(&filehash), Error::<T>::FileAlreadyExist);

			Files::<T>::insert(
				&filehash,
				&chunks
			);

			Self::deposit_event(Event::FileCreated(sender, filehash));

			Ok(().into())
		}

		// #[pallet::weight(0)]
		// pub fn revoke_claim(
		// 	origin: OriginFor<T>,
		// 	claim: Vec<u8>
		// ) -> DispatchResultWithPostInfo {
		// 	let sender = ensure_signed(origin)?;
		// 	let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
		// 	ensure!(owner == sender, Error::<T>::NotClaimOwner);

		// 	Proofs::<T>::remove(&claim);

		// 	Self::deposit_event(Event::ClaimRevoked(sender, claim));
		// 	Ok(().into())
		// }

		// #[pallet::weight(0)]
		// pub fn translate_claim(
		// 	dest: OriginFor<T>,
		// 	claim: Vec<u8>
		// ) -> DispatchResultWithPostInfo {
		// 	let receiver = ensure_signed(dest)?;
		// 	let (_, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

		// 	Proofs::<T>::mutate(&claim, |d| match d {
		// 		Some(data) => *data = (receiver.clone(),frame_system::Pallet::<T>::block_number()),
		// 		_ => (),
		// 	});

		// 	Self::deposit_event(Event::ClaimTraslated(receiver, claim));
		// 	Ok(().into())
		// }
	}
}

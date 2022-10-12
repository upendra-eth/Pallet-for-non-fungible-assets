#![cfg_attr(not(feature = "std"),no_std)]

pub use pallet::*;

// use codec::FullCodec;
use codec::{Decode, Encode, MaxEncodedLen};

#[frame_support::pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;


use core::default;
use core::ptr::NonNull;

use super::*;
	use frame_support::{
		// pallet_prelude::*, 
		// traits::KeyOwnerProofSystem, 
		pallet_prelude::ValueQuery,
		traits::GetDefault};
		
	use frame_support::inherent::Vec;
use scale_info::form::MetaForm;
use sp_runtime::AccountId32;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	//    #[pallet::generate_storage_info]
	
	pub struct Pallet<T>(_);
		
	// pub struct NFTS {
	// 	id: u32,
	// 	meta_data: Vec<u8>,
	// 	owner: T::AccountId,
	// }

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + 
		IsType<<Self as frame_system::Config>::Event>;
		// type KeyOwnerProof: Parameter;
		// type ProtocolOrigin: EnsureOrigin<Self::Origin>;
	

		type MetaData: Member
			+ Parameter
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;
	}
			


	#[pallet::storage]
	#[pallet::getter(fn total)]
	pub type Total<T> = StorageValue<_, u128>;


	#[pallet::storage]
	#[pallet::getter(fn abcd)]
	pub type Nfts<T> = StorageMap<_,
	Blake2_128Concat,
	u32,
	Vec<u8>,
	OptionQuery,
	>;	


	#[pallet::storage]
	#[pallet::getter(fn abc)]
	pub(crate) type NFTs<T:Config> = StorageMap<_, 
	Blake2_128Concat, 
	u32, 
	T::AccountId,
	ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn ab)]
	pub(crate) type COLLECTIONS<T:Config> = StorageMap<_, 
	Blake2_128Concat, 
	u32, 
	T::AccountId,
	ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn xyz)]
	pub(crate) type Collections<T:Config> = StorageMap<_, 
	Blake2_128Concat, 
	u32, 
	Vec<u8>,
	OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn xy)]
	pub(crate) type Collection<T:Config> = StorageMap<_, 
	Blake2_128Concat, 
	u32, 
	u32,
	OptionQuery,
	>;

	// #[pallet::storage]
	// /// The number of units of assets held by any given account.
	// pub(super) type Account<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
	// 	_,
	// 	Blake2_128Concat,
	// 	T::AssetId,
	// 	Blake2_128Concat,
	// 	T::AccountId,
	// 	AssetBalance<T::Balance, T::Extra>,
	// 	ValueQuery,
	// 	GetDefault,
	// 	ConstU32<300_000>,
	// >; []

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NftMinted(u32, Vec<u8> , T::AccountId),
		Transferred{id: u32, MetaData: Vec<u8>, dest: T::AccountId},
		NftBurned(u32, Vec<u8>, T::AccountId),
		CollectionCreated(u32, Vec<u8>, T::AccountId),
		CollectionBurned(u32, Vec<u8>, T::AccountId),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {

		NoneValue,
		StorageOverflow,
		NftAlreadyExists,
		NftDoesNotExist,
		NotOwner,
		CollectionAlreadyExists,
		WrongMetaData,
		NftNotInCollection,
		CollectionDoesNotExist,
		CollectionNotEmpty,

	}



	#[pallet::call]
		impl<T: Config> Pallet<T> {
	 	
			#[pallet::weight(10_000 + 
				T::DbWeight::get().writes(1))]
			 pub fn create_collection(
				origin: OriginFor<T>,
				collection_id: u32, 
				meta_data: Vec<u8>) ->
				 DispatchResult {
				
				let signer = 
				ensure_signed(origin)?;
				
				ensure!(!Self::existing_collection(
					collection_id.clone()), 
				Error::<T>::CollectionAlreadyExists);
	
				
					COLLECTIONS::<T>::insert(
						collection_id, 
						signer.clone()
					);
					Collections::<T>::insert(
						collection_id, 
						meta_data.clone()
					);
				
				Self::deposit_event(Event::CollectionCreated(
					collection_id.clone(), 
					meta_data.clone(),
					signer.clone()));
				
					Ok(())
			}
		
		
			#[pallet::weight(10_000 + 
			T::DbWeight::get().writes(1))]
	 	pub fn mint(
			origin: OriginFor<T>,
			collection_id: u32, 
			nft_id: u32, 
			meta_data: Vec<u8>) ->
			 DispatchResult {
			
			let signer = 
			ensure_signed(origin)?;
			
			ensure!(!Self::existing_nft(
				nft_id.clone()), 
			Error::<T>::NftAlreadyExists);
			
			// ensure!(!Self::existing_collection(
			// 	collection_id.clone()), 
			// Error::<T>::CollectionAlreadyExists);

			ensure!(!Self::non_existing_collection(
			collection_id.clone()), 
	 		Error::<T>::CollectionDoesNotExist);

			Collection::<T>::insert(
				collection_id, 
				nft_id.clone()
			);

			
				NFTs::<T>::insert(
					nft_id, 
					signer.clone()
				);
				Nfts::<T>::insert(
					nft_id, 
					meta_data.clone()
				);
			
			Self::deposit_event(Event::NftMinted(
				nft_id.clone(), 
				meta_data.clone(),
				signer.clone()));
			
				Ok(())
		}
	
	 	#[pallet::weight(10_000 + 
			T::DbWeight::get().writes(1))]
	 	pub fn transfer(
			origin: OriginFor<T>, 
			nft_id: u32, 
			meta_data: Vec<u8>, 
				new_owner: T::AccountId,) -> 
			DispatchResult {
			let signer = 
			ensure_signed(origin)?;
			
		 ensure!(!Self::non_existing_nft(
			nft_id.clone()),
		 Error::<T>::NftDoesNotExist);
			
		 ensure!(!Self::owner_account_nft(
			nft_id.clone(), 
			signer),
		 Error::<T>::NotOwner);
		// let who = T::Lookup::lookup(who)?;
		// ensure!(Account::<T, I>::contains_key(id, &who), Error::<T, I>::BalanceZero);

			   
			   NFTs::<T>::remove(nft_id.clone());
			   Nfts::<T>::remove(nft_id.clone());



			   NFTs::<T>::insert(
				nft_id, 
				new_owner.clone()
			);


				Nfts::<T>::insert(
					nft_id, 
					meta_data.clone()
				);

			   Self::deposit_event(Event::Transferred {
				id: nft_id.clone(), 
				MetaData: meta_data.clone(),
				dest: new_owner.clone()});
			
				Ok(())
   
			}
		
			#[pallet::weight(10_000 + 
				T::DbWeight::get().writes(1))]
				 pub fn burn_collection(
					origin: OriginFor<T>, 
					collection_id: u32, 
					meta_data: Vec<u8> ) -> 
					DispatchResult {
					let signer = 
					ensure_signed(origin)?;
		
					ensure!(!Self::non_existing_collection(
						collection_id.clone()), 
					Error::<T>::CollectionDoesNotExist);
		
					ensure!(!Self::owner_account_collection(
						collection_id.clone(), signer.clone()),
					Error::<T>::NotOwner);
		
		
					ensure!(!Self::check_meta_data_collection(
						collection_id.clone(), meta_data.clone()),
					Error::<T>::WrongMetaData);
		
					ensure!(!Self::collection_not_empty(
						collection_id.clone()),
					Error::<T>::CollectionNotEmpty);
		
					Collection::<T>::remove(collection_id);
					COLLECTIONS::<T>::remove(collection_id);
					Collections::<T>::remove(collection_id);
		
					Self::deposit_event(
						Event::CollectionBurned(
						collection_id.clone(), 
						meta_data.clone(),
						signer));
					
						Ok(())
				
				
			 	}
		   
		

	   #[pallet::weight(10_000 + 
		T::DbWeight::get().writes(1))]
	 	pub fn burn_nft(
			origin: OriginFor<T>, 
			collection_id: u32,
			nft_id: u32, 
			meta_data: Vec<u8> ) -> 
			DispatchResult {
			let signer = 
			ensure_signed(origin)?;

			ensure!(!Self::non_existing_nft(
				nft_id.clone()), 
			Error::<T>::NftDoesNotExist);

			ensure!(!Self::owner_account_nft(
				nft_id.clone(), signer.clone()),
			Error::<T>::NotOwner);


			ensure!(!Self::check_meta_data_nft(
				nft_id.clone(), meta_data.clone()),
			Error::<T>::WrongMetaData);

			ensure!(!Self::nft_in_collection(
				nft_id.clone(), collection_id),
			Error::<T>::NftNotInCollection);

			NFTs::<T>::remove(nft_id);
			Nfts::<T>::remove(nft_id);
			Collection::<T>::remove(nft_id);

			Self::deposit_event(
				Event::NftBurned(
				nft_id.clone(), 
				meta_data.clone(),
				signer));
			
				Ok(())
		
		
		}
	}         

		


	

		impl<T: Config> Pallet<T> {
		pub fn existing_nft(item: u32) -> 
		bool {
			let item_nft_id = item;
			// let item_nft_meta_data = item_data;
					Nfts::<T>::get(
						item_nft_id).
				is_some()
		}

		
			pub fn existing_collection(item: u32) -> 
			bool {
				let item_collection_id = item;
				// let item_nft_meta_data = item_data;
						Collections::<T>::get(
							item_collection_id).
					is_some()
			}

			pub fn nft_in_collection(
				item_nft_id: u32,
				item_collection_id: u32) ->
			bool {
				let nft_id = item_nft_id;
				let collection_id = item_collection_id;
				let abc = Collection::<T>::get(
					nft_id) == Some(collection_id);
					return abc;
			}

			pub fn check_meta_data_nft(
				item_id: u32,
				item_meta: Vec<u8>) ->
				bool {
					let item_nft_id = item_id; 
					let item_meta_data = item_meta;
					let abc = Nfts::<T>::get(
						item_nft_id) != Some(item_meta_data);
						return abc;

				}

				pub fn check_meta_data_collection(
					item_id: u32,
					item_meta: Vec<u8>) ->
					bool {
						let item_nft_id = item_id; 
						let item_meta_data = item_meta;
						let abc = Collections::<T>::get(
							item_nft_id) != Some(item_meta_data);
							return abc;
	
					}

	
		pub fn non_existing_nft(
			item: u32) -> 
			bool {
			let item_nft_id = item;
			Nfts::<T>::get(
				item_nft_id).
				is_none()

		}

		pub fn non_existing_collection(
			item: u32) -> 
			bool {
			let item_collection_id = item;
			Collections::<T>::get(
				item_collection_id).
				is_none()

		}

		pub fn owner_account_collection
		(
			// origin: OriginFor<T>, 
			item: u32, is_owner: T::AccountId) -> 
			bool {
			// let	who =ensure_signed(origin)?; 
			let item_collection_id = item;
			 let item_collection_owner= 
			 is_owner;
			let abc = COLLECTIONS::<T>::get(
				item_collection_id) != item_collection_owner;
				return abc;
			}

		pub fn owner_account_nft
		(
			// origin: OriginFor<T>, 
			item: u32, is_owner: T::AccountId) -> 
			bool {
			// let	who =ensure_signed(origin)?; 
			let item_nft_id = item;
			 let item_nft_owner=
			  is_owner;
			let abc = NFTs::<T>::get(
				item_nft_id) != item_nft_owner;
				return abc;
			}

			pub fn collection_not_empty(
				item_id: u32) ->
				bool {
					let collection_id = item_id;
					let abc = Collection::<T>::get(
						collection_id).
						is_some();
					 return abc;
				}


		}
	
	}



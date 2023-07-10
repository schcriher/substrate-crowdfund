#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod tipos;

use frame_support::traits::{Currency, Get};
use tipos::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, Blake2_128Concat};
	use frame_system::pallet_prelude::*;

	use frame_support::{
		sp_runtime::traits::{AccountIdConversion, Zero},
		traits::ExistenceRequirement::KeepAlive,
		PalletId,
	};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type LargoMinimoNombreProyecto: Get<u32>;

		#[pallet::constant]
		type LargoMaximoNombreProyecto: Get<u32>;

		type Currency: Currency<Self::AccountId>; // Pueden no utilizarlo.

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::storage]
	pub type Proyectos<T> =
		StorageMap<_, Blake2_128Concat, BoundedString<T>, BalanceDe<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProyectoCreado { quien: T::AccountId, nombre: NombreProyecto<T> },
		ProyectoApoyado { nombre: NombreProyecto<T>, cantidad: BalanceDe<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		NombreMuyLargo,
		NombreMuyCorto,
		/// El usuario quiso apoyar un proyecto con más fondos de los que dispone.
		FondosInsuficientes,
		/// El usuario quiso apoyar un proyecto inexistente.
		ProyectoNoExiste,
		/// El usuario quiso registrar un proyecto ya existente.
		ProyectoYaExiste,
		/// El cantidad aportada debe ser mayor a cero.
		CantidadDebeSerMayorACero,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Crea un proyecto.
		pub fn crear_proyecto(origen: OriginFor<T>, nombre: String) -> DispatchResult {
			let quien = ensure_signed(origen)?;

			ensure!(
				nombre.len() >= T::LargoMinimoNombreProyecto::get() as usize,
				Error::<T>::NombreMuyCorto
			);
			ensure!(
				nombre.len() <= T::LargoMaximoNombreProyecto::get() as usize,
				Error::<T>::NombreMuyLargo
			);
			let nombre: NombreProyecto<T> = nombre.try_into().unwrap();

			ensure!(!Proyectos::<T>::contains_key(&nombre), Error::<T>::ProyectoYaExiste);

			Proyectos::<T>::set(&nombre, Zero::zero());

			// TODO
			// Debería almacenarse quien es el propietario de cada proyecto
			// para luego transferir los fondos acumulados en la cuenta del pallet
			// debiendo existir otro extrinsic para este efecto.
			//
			// Se considera que la cuenta del Pallet contiene al menos la cantidad
			// mínima para subsistir (ExistentialDeposit), dado que no hay un mínimo
			// para apoyar un proyecto, el primer apoyo fallará si es menor al
			// ExistentialDeposit. Y si estaría implementado el mecanismo de extracción
			// de los fondos, el último fondo extraído dejaría sin fondo al Pallet y
			// fallaría (con KeepAlive) o se eliminaría la cuenta.

			Self::deposit_event(Event::ProyectoCreado { quien, nombre });

			Ok(())
		}

		/// Apoya un proyecto.
		pub fn apoyar_proyecto(
			origen: OriginFor<T>,
			nombre: String,
			cantidad: BalanceDe<T>,
		) -> DispatchResult {
			let quien = ensure_signed(origen)?;

			let nombre: NombreProyecto<T> = nombre.try_into().unwrap();
			ensure!(Proyectos::<T>::contains_key(&nombre), Error::<T>::ProyectoNoExiste);

			ensure!(cantidad > Zero::zero(), Error::<T>::CantidadDebeSerMayorACero);

			let tesoro = T::PalletId::get().into_account_truncating();
			let result = T::Currency::transfer(&quien, &tesoro, cantidad, KeepAlive);
			ensure!(result.is_ok(), Error::<T>::FondosInsuficientes);

			let balance = Proyectos::<T>::get(&nombre);
			Proyectos::<T>::set(&nombre, balance + cantidad);

			Self::deposit_event(Event::ProyectoApoyado { nombre, cantidad });

			Ok(())
		}
	}
}

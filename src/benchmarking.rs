#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as TransactionPallet;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{Pallet as System, RawOrigin};
use crate::structs::*;
use sp_io::hashing::blake2_256;
use frame_support::sp_runtime::traits::Hash;

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    System::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
    service_requested {
        let token_deposited = BalanceOf::<T>::from(100_000u32);
        let provider: T::AccountId = whitelisted_caller();
        let caller = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), provider.clone(), token_deposited)
    verify {
        assert_last_event::<T>(Event::<T>::ServiceRequested {
            consumer: whitelisted_caller(),
            provider,
            token_deposited
        }.into());
    }

    service_delivered {
        let provider: T::AccountId = whitelisted_caller();
        let caller = whitelisted_caller();

		let info = DeliveredInfo::<BalanceOf::<T>, T::Hash, T::BlockNumber> {
			token_num: BalanceOf::<T>::from(25u32),
			tx_hash: T::Hashing::hash_of(&blake2_256(b"tx hash")),
			time_point: TransactionPallet::<T>::now(),
			call_hash: blake2_256(b"call hash"),
		};

    }: _(RawOrigin::Signed(caller), provider.clone(), info.clone(), info.clone())
    verify {
        assert_last_event::<T>(Event::<T>::ServiceDelivered {
            consumer: whitelisted_caller(),
            provider: whitelisted_caller(),
            refund_info: info.clone(),
            spent_info: info.clone(),
        }.into());
    }
}

#[cfg(test)]
mod tests {
    use crate::mock;
    use frame_support::sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        mock::ExternalityBuilder::build()
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::benchmarking::tests::new_test_ext(),
    crate::mock::TestRuntime,
);

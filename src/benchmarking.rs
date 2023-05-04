#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::structs::*;
use crate::Pallet as TransactionPallet;
use frame_benchmarking::v1::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::sp_runtime::traits::Hash;
use frame_system::{Pallet as System, RawOrigin};
use sp_io::hashing::blake2_256;

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    System::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
    service_requested {
        let provider : T::AccountId = account("provider", 0, 0);
        let consumer : T::AccountId = account("consumer", 0, 0);
        let token_deposited = BalanceOf::<T>::from(100_000u32);
    }: _(RawOrigin::Signed(consumer.clone()), provider.clone(), token_deposited)
    verify {
        assert_last_event::<T>(Event::<T>::ServiceRequested {
            provider,
            consumer,
            token_deposited
        }.into());
    }

    service_delivered {
        let provider : T::AccountId = account("provider", 0, 0);
        let consumer : T::AccountId = account("consumer", 0, 0);

        let info = DeliveredInfo::<BalanceOf::<T>, T::Hash, T::BlockNumber> {
            token_num: BalanceOf::<T>::from(25u32),
            tx_hash: T::Hashing::hash_of(&blake2_256(b"tx hash")),
            time_point: TransactionPallet::<T>::now(),
            call_hash: blake2_256(b"call hash"),
        };

    }: _(RawOrigin::Signed(provider.clone()), consumer.clone(), info, info)
    verify {
        assert_last_event::<T>(Event::<T>::ServiceDelivered {
            consumer,
            provider,
            refund_info: info,
            spent_info: info,
        }.into());
    }
}

#[cfg(test)]
mod tests {
    use crate::mock;
    use frame_support::sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        mock::new_test_ext()
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::benchmarking::tests::new_test_ext(),
    crate::mock::Test,
);

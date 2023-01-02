use crate as peaq_pallet_transaction;
use crate::mock::*;
use crate::structs::*;
use frame_support::assert_ok;
use sp_io::hashing::blake2_256;

fn now() -> Timepoint<u64> {
    TransactionModule::now()
}

#[test]
fn service_requested_success() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(TransactionModule::service_requested(
            Origin::signed(1),
            2,
            42
        ));
        System::assert_last_event(
            peaq_pallet_transaction::Event::ServiceRequested {
                consumer: 1,
                provider: 2,
                token_deposited: 42,
            }
            .into(),
        );
    });
}

#[test]
fn service_delivered_success() {
    new_test_ext().execute_with(|| {
        let refund_info = DeliveredInfo {
            token_num: 25,
            tx_hash: blake2_256(b"refund tx hash").into(),
            time_point: now(),
            call_hash: blake2_256(b"refund call hash"),
        };
        let spent_info = DeliveredInfo {
            token_num: 20,
            tx_hash: blake2_256(b"spent tx hash").into(),
            time_point: now(),
            call_hash: blake2_256(b"spent call hash"),
        };

        assert_ok!(TransactionModule::service_delivered(
            Origin::signed(1),
            2,
            refund_info,
            spent_info
        ));

        System::assert_last_event(
            peaq_pallet_transaction::Event::ServiceDelivered {
                provider: 1,
                consumer: 2,
                refund_info,
                spent_info,
            }
            .into(),
        );
    });
}

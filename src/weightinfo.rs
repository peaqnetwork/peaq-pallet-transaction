//! The trait definition for the weights of extrinsics.

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn service_requested() -> Weight;
    fn service_delivered() -> Weight;
}

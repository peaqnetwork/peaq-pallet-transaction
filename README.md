# Peaq Pallet Transaction

#### Introduction
The Peaq-Pallet-Transaction is a message bridge to allow the provider/consumer to receive/dispatch transactional events from/to other parties on the network. Other actions can be triggered based on the event data the consumer/provider received.

There are two scenarios here.
1. The consumer sends `service_requested` Extrinsic and the provider parsed the event data on receiving the event. The parsed data is used by the provider to check the Multi-sig wallet balance if it's sufficient to start the EV charging.
2. The provider sends the `service_delivered` Extrinsic, and further operations are performed when the target user receives the event. For example, approving a multi-sig transaction.

In the future, we'll add types of message bridge between the provider and consumer to our system.

![Flow diagram](flow_diagram.png)

#### Installation
* Import the pallet dependecies by adding below snippets to your `runtime/src/Cargo.toml` file.
```
# --snip--

[dependencies.peaq-pallet-transaction]
default-features = false
git = 'https://github.com/peaqnetwork/peaq-pallet-transaction.git'
version = '0.0.1'

# --snip--

[features]
default = ['std']
runtime-benchmarks = [
  # --snip--
  'peaq-pallet-transaction/runtime-benchmarks',
]
std = [
  'peaq-pallet-transaction/std',
  # --snip--
]
```

* Implement peaq transaction pallet on your runtime by adding below snippets to `runtime/src/lib.rs` file.
```
# --snip--

pub use peaq_pallet_transaction;

# --snip--

/// Config the transaction in pallets/transaction
impl peaq_pallet_transaction::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
}
```

* Add PeaqTransaction parameter type to the runtime construct on your `runtime/src/lib.rs` file using below snippet.
```
# --snip--
PeaqTransaction: peaq_pallet_transaction::{Pallet, Call, Storage, Event<T>},
# --snip--
```

### Usage
* After installation, build your node
* Run and connect your node to Polkadotjs App
* Check for `PeaqTransaction` under `developer - Extrinsics` tab.

### Implementation:
In this pallet, we support two Extrinsics for commmunication:

#### service_requested
The consumer uses this Extrinsic to send the message which contains provider's PK and deposited token.

#### service_delivered
The provider sends the refund/spent information (token number, transaction hash, multisig transaction's time point and call hash) and the consumer PK by this Extrinsic.


## License

[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)

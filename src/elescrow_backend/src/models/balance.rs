use ic_stable_structures::Storable;
use std::borrow::Cow;
use crate::types::transaction::{Balance};

impl Storable for Balance {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 512,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let bytes = serde_cbor::to_vec(self).expect("Failed to serialize Balance");
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).expect("Failed to deserialize Balance")
    }
}
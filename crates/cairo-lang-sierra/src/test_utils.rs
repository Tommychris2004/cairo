use bimap::BiMap;
use itertools::chain;

use crate::ids::ConcreteTypeId;
use crate::program::{ConcreteTypeLongId, GenericArg};

pub fn build_bijective_mapping() -> BiMap<ConcreteTypeId, ConcreteTypeLongId> {
    let mut elements = BiMap::new();
    elements.insert("T".into(), as_type_long_id("T", &[]));
    elements.insert("u32".into(), as_type_long_id("u32", &[]));
    elements.insert("u64".into(), as_type_long_id("u64", &[]));
    elements.insert("u128".into(), as_type_long_id("u128", &[]));
    elements.insert("felt252".into(), as_type_long_id("felt252", &[]));
    elements.insert("Tuple<>".into(), as_named_type_long_id("Struct", "Tuple", &[]));
    elements.insert(
        "U128AndFelt252".into(),
        as_named_type_long_id("Struct", "U128AndFelt252", &["u128", "felt252"]),
    );
    elements
        .insert("Option".into(), as_named_type_long_id("Enum", "Option", &["felt252", "Tuple<>"]));
    elements.insert("NonZeroFelt252".into(), as_type_long_id("NonZero", &["felt252"]));
    elements.insert("NonZeroU128".into(), as_type_long_id("NonZero", &["u128"]));
    elements.insert("ArrayFelt252".into(), as_type_long_id("Array", &["felt252"]));
    elements.insert("ArrayU128".into(), as_type_long_id("Array", &["u128"]));
    elements.insert("UninitializedFelt252".into(), as_type_long_id("Uninitialized", &["felt252"]));
    elements.insert("Uninitializedu128".into(), as_type_long_id("Uninitialized", &["u128"]));
    elements.insert("GasBuiltin".into(), as_type_long_id("GasBuiltin", &[]));
    elements.insert("RangeCheck".into(), as_type_long_id("RangeCheck", &[]));
    elements.insert("System".into(), as_type_long_id("System", &[]));
    elements.insert("StorageBaseAddress".into(), as_type_long_id("StorageBaseAddress", &[]));
    elements.insert("StorageAddress".into(), as_type_long_id("StorageAddress", &[]));
    elements.insert("ContractAddress".into(), as_type_long_id("ContractAddress", &[]));
    elements.insert("SnapshotRangeCheck".into(), as_type_long_id("Snapshot", &["RangeCheck"]));
    elements.insert("SnapshotArrayU128".into(), as_type_long_id("Snapshot", &["ArrayU128"]));
    elements.insert(
        "NonDupStruct".into(),
        as_named_type_long_id("Struct", "NonDupStruct", &["felt252", "RangeCheck"]),
    );
    elements.insert("SnapshotNonDupStruct".into(), as_type_long_id("Snapshot", &["NonDupStruct"]));
    elements.insert(
        "NonDupEnum".into(),
        as_named_type_long_id("Enum", "NonDupEnum", &["felt252", "RangeCheck"]),
    );
    elements.insert("SnapshotNonDupEnum".into(), as_type_long_id("Snapshot", &["NonDupEnum"]));
    elements
}

fn as_type_long_id(name: &str, args: &[&str]) -> ConcreteTypeLongId {
    ConcreteTypeLongId {
        generic_id: name.into(),
        generic_args: args.iter().map(|s| GenericArg::Type(ConcreteTypeId::from(*s))).collect(),
    }
}

fn as_named_type_long_id(genetic_name: &str, user_name: &str, args: &[&str]) -> ConcreteTypeLongId {
    ConcreteTypeLongId {
        generic_id: genetic_name.into(),
        generic_args: chain!(
            [GenericArg::UserType(user_name.into())],
            args.iter().map(|s| GenericArg::Type(ConcreteTypeId::from(*s)))
        )
        .collect(),
    }
}

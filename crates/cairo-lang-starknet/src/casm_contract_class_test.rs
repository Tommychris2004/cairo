use std::io::BufReader;

use cairo_lang_test_utils::compare_contents_or_fix_with_path;
use num_bigint::BigUint;
use num_traits::Num;
use test_case::test_case;

use crate::casm_contract_class::{BigIntAsHex, CasmContractClass, StarknetSierraCompilationError};
use crate::contract_class::ContractClass;
use crate::test_utils::{get_example_file_path, get_test_contract};

#[test_case("account")]
#[test_case("test_contract")]
#[test_case("minimal_contract")]
#[test_case("hello_starknet")]
#[test_case("erc20")]
fn test_casm_contract_from_contract_class(example_file_name: &str) {
    let contract_class = get_test_contract(format!("{example_file_name}.cairo").as_str());
    let casm_contract = CasmContractClass::from_contract_class(contract_class).unwrap();

    compare_contents_or_fix_with_path(
        &get_example_file_path(format!("{example_file_name}.casm.json").as_str()),
        serde_json::to_string_pretty(&casm_contract).unwrap() + "\n",
    );
}

#[test_case("test_contract")]
fn test_casm_contract_from_contract_class_failure(example_file_name: &str) {
    let f =
        std::fs::File::open(get_example_file_path(&format!("{example_file_name}.json"))).unwrap();
    let mut contract_class: ContractClass = serde_json::from_reader(BufReader::new(f)).unwrap();

    let prime = BigUint::from_str_radix(
        "800000000000011000000000000000000000000000000000000000000000001",
        16,
    )
    .unwrap();

    contract_class.sierra_program[17] = BigIntAsHex { value: prime };

    assert_eq!(
        CasmContractClass::from_contract_class(contract_class),
        Err(StarknetSierraCompilationError::ValueOutOfRange)
    );
}

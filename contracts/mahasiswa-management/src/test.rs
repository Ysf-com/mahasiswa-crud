#![cfg(test)]

extern crate std;

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::{MahasiswaManagementContract, MahasiswaManagementContractClient};

#[test]
fn test_mahasiswa_flow() {
    let env = Env::default();
    let contract_id = env.register(MahasiswaManagementContract, ());
    let client = MahasiswaManagementContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init(&admin);

    let nim = String::from_str(&env, "220001");
    let nama = String::from_str(&env, "Budi");
    let jurusan = String::from_str(&env, "Informatika");

    client.add_student(&nim, &nama, &jurusan, &2022, &true);

    let student = client.get_student(&nim);
    assert_eq!(student.nim, nim);
    assert_eq!(student.nama, nama);
}
#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test() {
    // Set up the test environment
    let env = Env::default();
    env.mock_all_auths();
    // Create a test admin address
    let admin_1 = Address::generate(&env);

    // Create a client for the contract
    let contract_id = env.register(HospitalContract, {});
    let client = HospitalContractClient::new(&env, &contract_id);

    // Call the initalize function
    let result = client.initialize(&admin_1);

    // Check that the result is the admin address
    assert_eq!(result, admin_1);

    // Verify that the storage is correctly set - using env.as_contract()
    let stored_admin: Address = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    });
    assert_eq!(stored_admin, admin_1);

    let patient_count: u64 = env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .get(&DataKey::PatientCount)
            .unwrap()
    });
    assert_eq!(patient_count, 0);

    let doctor_count: u64 = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::DoctorCount).unwrap()
    });
    assert_eq!(doctor_count, 0);

    let test_count: u64 = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::TestCount).unwrap()
    });
    assert_eq!(test_count, 0);

    // Test Patient Management Functions
    // Patient data
    let name = String::from_str(&env, "John Doe");
    let date_of_birth = 946684800; // January 1, 2000 (Unix timestamp)
    let blood_type = String::from_str(&env, "O+");
    let penicillin_str = String::from_str(&env, "penicillin");
    let peanut_str = String::from_str(&env, "peanuts");
    let mut allergies: soroban_sdk::Vec<String> = soroban_sdk::Vec::new(&env);
    allergies.push_back(penicillin_str);
    allergies.push_back(peanut_str);
    let insurance_id = String::from_str(&env, "INS123456");

    // Test registration of a new patient
    let patient_id = client.register_patient(
        &name,
        &date_of_birth,
        &blood_type,
        &allergies,
        &insurance_id,
    );

    assert_eq!(patient_id, 1);

    // Test getting a patients record
    let patient = client.get_patient(&patient_id);
    assert_eq!(patient.id, 1);
    assert_eq!(patient.name, name);

    // verify patient count was incremented
    let patient_count: u64 = env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .get(&DataKey::PatientCount)
            .unwrap()
    });
    assert_eq!(patient_count, 1);

    // verify empty test list was initialized
    let patient_tests: Vec<u64> = env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .get(&DataKey::PatientTests(patient_id))
            .unwrap()
    });
    assert_eq!(patient_tests.len(), 0);

    // Test updating a patient's information
    let updated_name = String::from_str(&env, "John Smith");
    let updated_date_of_birth = 949449600; // February 1, 2000 (Unix timestamp)
    let updated_blood_type = String::from_str(&env, "A-");
    let latex_str = String::from_str(&env, "latex");
    let mut updated_allergies: soroban_sdk::Vec<String> = soroban_sdk::Vec::new(&env);
    updated_allergies.push_back(latex_str);
    let updated_insurance_id = String::from_str(&env, "INS789012");

    let updated_patient = client.update_patient(
        &patient_id,
        &updated_name,
        &updated_date_of_birth,
        &updated_blood_type,
        &updated_allergies,
        &updated_insurance_id,
    );

    assert_eq!(updated_patient.name, updated_name);

    // Test setting patient status to inactive
    let inactive_patient = client.set_patient_active(&patient_id, &false);
    assert_eq!(inactive_patient.active, false);

    // Register another patient
    let name2 = String::from_str(&env, "Jane Doe");
    let date_of_birth2 = 978307200; // January 1, 2001 (Unix timestamp)
    let blood_type2 = String::from_str(&env, "B+");
    let mut allergies2: soroban_sdk::Vec<String> = soroban_sdk::Vec::new(&env);
    let shellfish_str = String::from_str(&env, "shellfish");
    allergies2.push_back(shellfish_str);
    let insurance_id2 = String::from_str(&env, "INS654321");

    let patient_id2 = client.register_patient(
        &name2,
        &date_of_birth2,
        &blood_type2,
        &allergies2,
        &insurance_id2,
    );
    assert_eq!(patient_id2, 2);

    // Test listing all patients
    let patients = client.list_patients();
    assert_eq!(patients.len(), 2);
}

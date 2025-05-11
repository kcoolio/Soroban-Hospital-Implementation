#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_patient_management_functions() {
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

#[test]
fn test_doctor_test_management_functions() {
    // Set up the test environment
    let env = Env::default();
    env.mock_all_auths();
    // Create a test admin address
    let admin = Address::generate(&env);

    // Create a client for the contract
    let contract_id = env.register(HospitalContract, {});
    let client = HospitalContractClient::new(&env, &contract_id);

    // Call the initalize function to initalize the contract
    client.initialize(&admin);

    // Test Doctor Management Functions

    // Doctor data
    let name = String::from_str(&env, "Dr. Jane Smith");
    let specialization = String::from_str(&env, "Cardiology");
    let license_number = String::from_str(&env, "MED12345");

    // Test registration of a new doctor
    let doctor_id = client.register_doctor(&name, &specialization, &license_number);
    assert_eq!(doctor_id, 1);

    // Test getting a doctor's record
    let doctor = client.get_doctor(&doctor_id);
    assert_eq!(doctor.id, 1);
    assert_eq!(doctor.name, name);
    assert_eq!(doctor.specialization, specialization);
    assert_eq!(doctor.license_number, license_number);
    assert_eq!(doctor.active, true);

    // Verify doctor count was incremented
    let doctor_count: u64 = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::DoctorCount).unwrap()
    });
    assert_eq!(doctor_count, 1);

    // Verify empty test list was initialized
    let doctor_tests: Vec<u64> = env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .get(&DataKey::DoctorTests(doctor_id))
            .unwrap()
    });
    assert_eq!(doctor_tests.len(), 0);

    // Test updating a doctor's information
    let updated_name = String::from_str(&env, "Dr. Jane Wilson");
    let updated_specialization = String::from_str(&env, "Electrophysiology");
    let updated_license_number = String::from_str(&env, "MED67890");

    let updated_doctor = client.update_doctor(
        &doctor_id,
        &updated_name,
        &updated_specialization,
        &updated_license_number,
    );

    // Verify the update was successful
    assert_eq!(updated_doctor.id, doctor_id);
    assert_eq!(updated_doctor.name, updated_name);
    assert_eq!(updated_doctor.specialization, updated_specialization);
    assert_eq!(updated_doctor.license_number, updated_license_number);
    assert_eq!(updated_doctor.active, true);

    // Test getting the updated doctor record
    let fetched_doctor = client.get_doctor(&doctor_id);
    assert_eq!(fetched_doctor, updated_doctor);

    // Test setting doctor status to inactive
    let inactive_doctor = client.set_doctor_active(&doctor_id, &false);
    assert_eq!(inactive_doctor.id, doctor_id);
    assert_eq!(inactive_doctor.active, false);

    // Verify the status change persisted
    let fetched_inactive_doctor = client.get_doctor(&doctor_id);
    assert_eq!(fetched_inactive_doctor.active, false);

    // Test setting doctor back to active
    let reactivated_doctor = client.set_doctor_active(&doctor_id, &true);
    assert_eq!(reactivated_doctor.active, true);

    // Register another doctor
    let name2 = String::from_str(&env, "Dr. John Lee");
    let specialization2 = String::from_str(&env, "Neurology");
    let license_number2 = String::from_str(&env, "MED54321");

    let doctor_id2 = client.register_doctor(&name2, &specialization2, &license_number2);
    assert_eq!(doctor_id2, 2);

    // Test listing all doctors
    let doctors = client.list_doctors();
    assert_eq!(doctors.len(), 2);

    // Verify the doctor count
    let final_doctor_count: u64 = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::DoctorCount).unwrap()
    });
    assert_eq!(final_doctor_count, 2);
}

#[test]
fn test_medical_test_management_functions() {
    // Set up the test environment
    let env = Env::default();
    env.mock_all_auths();
    // Create a test admin address
    let admin = Address::generate(&env);

    // Create a client for the contract
    let contract_id = env.register(HospitalContract, {});
    let client = HospitalContractClient::new(&env, &contract_id);

    // Call the initalize function to initalize the contract
    client.initialize(&admin);

    // First, register a patient and doctor for testing

    // Patient data
    let patient_name = String::from_str(&env, "John Doe");
    let date_of_birth = 946684800; // January 1, 2000
    let blood_type = String::from_str(&env, "O+");
    let allergies = soroban_sdk::Vec::new(&env);
    let insurance_id = String::from_str(&env, "INS123456");

    let patient_id = client.register_patient(
        &patient_name,
        &date_of_birth,
        &blood_type,
        &allergies,
        &insurance_id,
    );

    // Doctor data
    let doctor_name = String::from_str(&env, "Dr. Jane Smith");
    let specialization = String::from_str(&env, "Cardiology");
    let license_number = String::from_str(&env, "MED12345");

    let doctor_id = client.register_doctor(&doctor_name, &specialization, &license_number);

    // Create a medical test
    let test_type = String::from_str(&env, "Blood Test");
    let results = String::from_str(&env, "Normal red and white blood cell count");
    let notes = String::from_str(&env, "Patient should return for follow-up in 6 months");
    let test_date = 1620000000; // May 3, 2021

    let test_id = client.record_medical_test(
        &patient_id,
        &doctor_id,
        &test_type,
        &results,
        &notes,
        &test_date,
    );

    // Verify test was created with ID 1
    assert_eq!(test_id, 1);

    // Get the test and verify its data
    let test = client.get_medical_test(&test_id);
    assert_eq!(test.id, test_id);
    assert_eq!(test.patient_id, patient_id);
    assert_eq!(test.doctor_id, doctor_id);
    assert_eq!(test.test_type, test_type);
    assert_eq!(test.results, results);
    assert_eq!(test.notes, notes);
    assert_eq!(test.test_date, test_date);

    // Check test count was updated
    let test_count: u64 = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::TestCount).unwrap()
    });
    assert_eq!(test_count, 1);

    // Verify test was added to patient's test list
    let patient_tests = client.get_tests_for_patient(&patient_id);
    assert_eq!(patient_tests.len(), 1);
    assert_eq!(patient_tests.get(0).unwrap().id, test_id);

    // Verify test was added to doctor's test list
    let doctor_tests = client.get_tests_for_doctor(&doctor_id);
    assert_eq!(doctor_tests.len(), 1);
    assert_eq!(doctor_tests.get(0).unwrap().id, test_id);

    // Add a second test
    let test_type2 = String::from_str(&env, "EKG");
    let results2 = String::from_str(&env, "Normal sinus rhythm");
    let notes2 = String::from_str(&env, "No abnormalities detected");
    let test_date2 = 1625000000; // June 30, 2021

    let test_id2 = client.record_medical_test(
        &patient_id,
        &doctor_id,
        &test_type2,
        &results2,
        &notes2,
        &test_date2,
    );

    assert_eq!(test_id2, 2);

    // Get tests for patient and verify count
    let patient_tests_updated = client.get_tests_for_patient(&patient_id);
    assert_eq!(patient_tests_updated.len(), 2);

    // Get tests for doctor and verify count
    let doctor_tests_updated = client.get_tests_for_doctor(&doctor_id);
    assert_eq!(doctor_tests_updated.len(), 2);

    // Test list all medical tests
    let all_tests = client.list_all_medical_tests();
    assert_eq!(all_tests.len(), 2);

    // Register a second patient and doctor
    let patient_name2 = String::from_str(&env, "Jane Doe");
    let date_of_birth2 = 978307200; // January 1, 2001
    let blood_type2 = String::from_str(&env, "A-");
    let insurance_id2 = String::from_str(&env, "INS654321");

    let patient_id2 = client.register_patient(
        &patient_name2,
        &date_of_birth2,
        &blood_type2,
        &allergies, // reuse empty allergies list
        &insurance_id2,
    );

    let doctor_name2 = String::from_str(&env, "Dr. John Lee");
    let specialization2 = String::from_str(&env, "Orthopedics");
    let license_number2 = String::from_str(&env, "MED54321");

    let doctor_id2 = client.register_doctor(&doctor_name2, &specialization2, &license_number2);

    // Create a test with the second patient and doctor
    let test_type3 = String::from_str(&env, "X-Ray");
    let results3 = String::from_str(&env, "No fractures detected");
    let notes3 = String::from_str(&env, "Minor inflammation in knee joint");
    let test_date3 = 1630000000; // August 27, 2021

    let test_id3 = client.record_medical_test(
        &patient_id2,
        &doctor_id2,
        &test_type3,
        &results3,
        &notes3,
        &test_date3,
    );

    assert_eq!(test_id3, 3);

    // Verify patient_id2's tests
    let patient2_tests = client.get_tests_for_patient(&patient_id2);
    assert_eq!(patient2_tests.len(), 1);

    // Verify doctor_id2's tests
    let doctor2_tests = client.get_tests_for_doctor(&doctor_id2);
    assert_eq!(doctor2_tests.len(), 1);

    // Verify final test count and all medical tests list
    let final_test_count: u64 = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::TestCount).unwrap()
    });
    assert_eq!(final_test_count, 3);

    let final_all_tests = client.list_all_medical_tests();
    assert_eq!(final_all_tests.len(), 3);
}

use solana_signer::Signer;

use crate::test_helpers::{
    execute_attest, execute_initialize, generate_id, get_meditation_plan, TestHarness,
    COMMITMENT_STAKE, DAILY_FREQUENCY, DURATION_MINUTES, NUMBER_OF_DAYS,
};

#[test]
fn test_attest_succeeds() {
    let (mut svm, harness) = TestHarness::new();

    // Initialize the meditation plan
    let id = generate_id();
    let initialize_result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        id,
        NUMBER_OF_DAYS,
        DAILY_FREQUENCY,
        DURATION_MINUTES,
        COMMITMENT_STAKE,
    );
    assert!(initialize_result.is_ok(), "Initialization should succeed");
    let (meditation_plan, _meditation_bump, _vault) = initialize_result.unwrap();

    let started_at = 0; // Use 0 for testing
    let ended_at = 30 * 60;

    // Execute the attest instructions
    let attest_result = execute_attest(
        &mut svm,
        &harness.alice,
        meditation_plan,
        started_at,
        ended_at,
    );
    assert!(attest_result.is_ok(), "Attestation should succeed");

    let (plan_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    assert_eq!(plan_account.owner, harness.program_id);
    assert_eq!(
        plan.attestations.len(),
        1,
        "There should be one attestation"
    );
    assert_eq!(plan.attestations[0].attester, harness.alice.pubkey());
    assert_eq!(
        plan.attestations[0].started_at, started_at,
        "Timestamp should be 0 for test"
    );
    assert_eq!(plan.attestations[0].ended_at, ended_at);
    assert_eq!(
        plan.is_active, true,
        "Plan should be active after attestation"
    );
    assert_eq!(plan.is_completed, false, "Plan should not be completed yet");
    assert_eq!(plan.penalties, 0, "There should be no penalties yet");
    assert_eq!(plan.rewards, 0, "There should be no rewards yet");
}

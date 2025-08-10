use solana_signer::Signer;

use crate::constants::HOUR_IN_SECONDS;
use crate::test_helpers::{
    create_standard_plan, execute_attest, get_meditation_plan, set_clock, set_meditation_plan,
    TestHarness, ENDED_AT, FIFTY_USDC, STARTED_AT,
};
use crate::MeditationPlan;

#[test]
fn test_attest_succeeds() {
    let (mut svm, harness) = TestHarness::new();
    let meditation_plan = create_standard_plan(&mut svm, &harness);

    let result = execute_attest(
        &mut svm,
        &harness.alice,
        meditation_plan,
        STARTED_AT,
        ENDED_AT,
    );
    assert!(result.is_ok(), "Attestation should succeed");

    let (plan_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    assert_eq!(plan_account.owner, harness.program_id);
    assert_eq!(
        plan.attestations.len(),
        1,
        "There should be one attestation"
    );
    assert_eq!(plan.attestations[0].attester, harness.alice.pubkey());
    assert_eq!(
        plan.attestations[0].started_at, STARTED_AT,
        "Timestamp should be 0 for test"
    );
    assert_eq!(plan.attestations[0].ended_at, ENDED_AT);
    assert_eq!(
        plan.is_active, true,
        "Plan should be active after attestation"
    );
    assert_eq!(plan.is_completed, false, "Plan should not be completed yet");
    assert_eq!(plan.penalties, 0, "There should be no penalties yet");
    // 7 days @ once per day = 1/7 of the commitment stake should be rewarded
    assert_eq!(
        plan.rewards,
        FIFTY_USDC / 7,
        "The rewards for a single session should be added to the plan"
    );
}

#[test]
fn test_unauthorized_attester_fails() {
    let (mut svm, harness) = TestHarness::new();
    let meditation_plan = create_standard_plan(&mut svm, &harness);

    let result = execute_attest(
        &mut svm,
        &harness.bob,
        meditation_plan,
        STARTED_AT,
        ENDED_AT,
    );
    assert!(
        result.is_err(),
        "Attestation should fail when attester is not plan owner"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Error Code: UnauthorizedAccess"),
        "Incorrect error for unauthorized"
    );
}

#[test]
fn test_inactive_plan_fails() {
    let (mut svm, harness) = TestHarness::new();
    let meditation_plan = create_standard_plan(&mut svm, &harness);

    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    let new_plan = MeditationPlan {
        is_active: false,
        ..plan.clone()
    };
    set_meditation_plan(&mut svm, meditation_plan, new_plan);

    let result = execute_attest(
        &mut svm,
        &harness.alice,
        meditation_plan,
        STARTED_AT,
        ENDED_AT,
    );
    assert!(
        result.is_err(),
        "Attestation should fail when plan inactive"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Error Code: PlanInactive"),
        "Incorrect error for inactive plan"
    );
}

#[test]
fn test_completed_plan_fails() {
    let (mut svm, harness) = TestHarness::new();
    let meditation_plan = create_standard_plan(&mut svm, &harness);

    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    let new_plan = MeditationPlan {
        is_completed: true,
        ..plan.clone()
    };
    set_meditation_plan(&mut svm, meditation_plan, new_plan);

    let result = execute_attest(
        &mut svm,
        &harness.alice,
        meditation_plan,
        STARTED_AT,
        ENDED_AT,
    );
    assert!(
        result.is_err(),
        "Attestation should fail when plan already completed"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Error Code: PlanCompleted"),
        "Incorrect error for completed plan"
    );
}

#[test]
fn test_plan_ended_before_started_at_fails() {
    let (mut svm, harness) = TestHarness::new();
    let meditation_plan = create_standard_plan(&mut svm, &harness);
    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);

    let started_at = plan.end_at + 1; // Set start time after plan start time
    let ended_at = started_at + ENDED_AT;
    set_clock(&mut svm, ended_at + 1); // Set clock so attestation is in the past
    let result = execute_attest(
        &mut svm,
        &harness.alice,
        meditation_plan,
        started_at,
        ended_at,
    );
    assert!(
        result.is_err(),
        "Attestation should fail when start time past plan end time"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Error Code: PlanExpired"),
        "Incorrect error for expired plan"
    );
}

#[test]
fn test_duration_below_plan_duration_fails() {
    let (mut svm, harness) = TestHarness::new();
    let meditation_plan = create_standard_plan(&mut svm, &harness);

    let result = execute_attest(
        &mut svm,
        &harness.alice,
        meditation_plan,
        STARTED_AT,
        19 * 60, // Less than plan duration
    );
    assert!(
        result.is_err(),
        "Attestation should fail when duration is less than plan duration"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Error Code: AttestationTooShort"),
        "Incorrect error for attestation too short"
    );
}

#[test]
fn test_duration_above_maximum_fails() {
    let (mut svm, harness) = TestHarness::new();
    let meditation_plan = create_standard_plan(&mut svm, &harness);

    let ended_at = STARTED_AT + (8 * HOUR_IN_SECONDS) + 1; // More than 8 hours
    set_clock(&mut svm, ended_at + 1); // Set clock so attestation is in the past

    let result = execute_attest(
        &mut svm,
        &harness.alice,
        meditation_plan,
        STARTED_AT,
        ended_at,
    );
    assert!(
        result.is_err(),
        "Attestation should fail when duration is greater than 8 hours"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Error Code: AttestationTooLong"),
        "Incorrect error for attestation too long"
    );
}

#[test]
fn test_daily_frequency_exceeded_fails() {
    let (mut svm, harness) = TestHarness::new();
    let meditation_plan = create_standard_plan(&mut svm, &harness);

    let result = execute_attest(
        &mut svm,
        &harness.alice,
        meditation_plan,
        STARTED_AT,
        ENDED_AT,
    );
    assert!(result.is_ok(), "Attestation should succeed");

    let started_at = ENDED_AT + 1_000; // Same day as first attestation
    let ended_at = started_at + 30 * 60; // More than plan duration
    set_clock(&mut svm, ended_at + 1); // Set clock so attestation is in the past

    let result = execute_attest(
        &mut svm,
        &harness.alice,
        meditation_plan,
        started_at,
        ended_at,
    );
    assert!(
        result.is_err(),
        "Attestation should fail daily frequency exceeded"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Error Code: DailyFrequencyExceeded"),
        "Incorrect error for daily frequency exceeded"
    );
}

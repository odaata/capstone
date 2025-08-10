use solana_kite::get_token_account_balance;

use crate::test_helpers::{
    create_standard_plan, execute_attest, execute_complete, get_meditation_plan, set_clock,
    set_meditation_plan, TestHarness, COMMITMENT_STAKE, DURATION_MINUTES, FIFTY_USDC, HUNDY_USDC,
    NUMBER_OF_DAYS, REWARDS_PER_SESSION, STARTED_AT,
};
use crate::{MeditationPlan, DAY_IN_SECONDS};

#[test]
fn test_complete_all_sessions_succeeds() {
    let (mut svm, harness) = TestHarness::new();

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), HUNDY_USDC);

    let (meditation_plan, vault) = create_standard_plan(&mut svm, &harness);

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    let balance = get_token_account_balance(&svm, &vault);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    for i in 0..NUMBER_OF_DAYS {
        let started_at = STARTED_AT + (i as i64 * DAY_IN_SECONDS);
        let ended_at = started_at + (DURATION_MINUTES as i64 * 60);
        set_clock(&mut svm, ended_at + 1); // Set clock so attestation is in the past

        let result = execute_attest(
            &mut svm,
            &harness.alice,
            meditation_plan,
            started_at,
            ended_at,
        );
        let day = i + 1;
        assert!(result.is_ok(), "Attestation should succeed for day {}", day);
        let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
        assert_eq!(
            plan.attestations.len(),
            day as usize,
            "There should be {} attestations after day {}",
            day,
            day
        );
        let expected_rewards = REWARDS_PER_SESSION * day as u64;
        assert_eq!(
            plan.rewards, expected_rewards,
            "Rewards should accumulate correctly after day {}",
            day
        );
    }

    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    assert_eq!(plan.attestations.len(), NUMBER_OF_DAYS as usize);
    assert_eq!(plan.is_active, true);
    assert_eq!(plan.is_completed, false);
    assert_eq!(plan.penalties, 0);
    assert_eq!(plan.rewards, REWARDS_PER_SESSION * NUMBER_OF_DAYS as u64);

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    let balance = get_token_account_balance(&svm, &vault);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    let result = execute_complete(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        meditation_plan,
        vault,
    );
    assert!(result.is_ok(), "Complete should succeed");

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), HUNDY_USDC);

    let balance = get_token_account_balance(&svm, &vault);
    assert_eq!(balance.unwrap(), 0);

    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    assert_eq!(plan.attestations.len(), NUMBER_OF_DAYS as usize);
    assert_eq!(plan.is_active, false);
    assert_eq!(plan.is_completed, true);
    assert_eq!(plan.penalties, 0);
    // Rewards should be the total commitment stake - to handle rounded reward percentages
    assert_eq!(plan.rewards, COMMITMENT_STAKE);
}

#[test]
fn test_complete_with_penalties_succeeds() {
    let (mut svm, harness) = TestHarness::new();

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), HUNDY_USDC);

    let (meditation_plan, vault) = create_standard_plan(&mut svm, &harness);

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    let balance = get_token_account_balance(&svm, &vault);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    let number_of_days = 2;
    for i in 0..number_of_days {
        let started_at = STARTED_AT + (i as i64 * DAY_IN_SECONDS);
        let ended_at = started_at + (DURATION_MINUTES as i64 * 60);
        set_clock(&mut svm, ended_at + 1); // Set clock so attestation is in the past

        let result = execute_attest(
            &mut svm,
            &harness.alice,
            meditation_plan,
            started_at,
            ended_at,
        );
        let day = i + 1;
        assert!(result.is_ok(), "Attestation should succeed for day {}", day);
        let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
        assert_eq!(
            plan.attestations.len(),
            day as usize,
            "There should be {} attestations after day {}",
            day,
            day
        );
        let expected_rewards = REWARDS_PER_SESSION * day as u64;
        assert_eq!(
            plan.rewards, expected_rewards,
            "Rewards should accumulate correctly after day {}",
            day
        );
    }

    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    assert_eq!(plan.attestations.len(), number_of_days);
    assert_eq!(plan.is_active, true);
    assert_eq!(plan.is_completed, false);
    assert_eq!(plan.penalties, 0);
    assert_eq!(plan.rewards, REWARDS_PER_SESSION * number_of_days as u64);

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    let balance = get_token_account_balance(&svm, &vault);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    set_clock(&mut svm, plan.end_at + 1); // Set clock to after the plan end date
    let result = execute_complete(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        meditation_plan,
        vault,
    );
    assert!(result.is_ok(), "Complete should succeed");

    let remainder = FIFTY_USDC % 7;
    // User should receive the remainder when not evenly divisible
    let expected_rewards = ((FIFTY_USDC / 7) * number_of_days as u64) + remainder;
    let expected_penalties = (FIFTY_USDC / 7) * 5;
    assert_eq!(
        expected_rewards + expected_penalties,
        FIFTY_USDC,
        "Total should equal the original commitment stake"
    );

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), FIFTY_USDC + expected_rewards);

    let balance = get_token_account_balance(&svm, &vault);
    assert_eq!(balance.unwrap(), expected_penalties);

    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    assert_eq!(plan.attestations.len(), number_of_days as usize);
    assert_eq!(plan.is_active, false);
    assert_eq!(plan.is_completed, true);
    assert_eq!(plan.penalties, expected_penalties);
    assert_eq!(plan.rewards, expected_rewards);
}

#[test]
fn test_unauthorized_owner_fails() {
    let (mut svm, harness) = TestHarness::new();
    let (meditation_plan, vault) = create_standard_plan(&mut svm, &harness);

    let result = execute_complete(
        &mut svm,
        harness.usdc_mint,
        &harness.bob,
        harness.bob_usdc_account,
        meditation_plan,
        vault,
    );
    assert!(
        result.is_err(),
        "Completion should fail when owner is not plan owner"
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
    let (meditation_plan, vault) = create_standard_plan(&mut svm, &harness);

    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    let new_plan = MeditationPlan {
        is_active: false,
        ..plan.clone()
    };
    set_meditation_plan(&mut svm, meditation_plan, new_plan);

    let result = execute_complete(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        meditation_plan,
        vault,
    );
    assert!(result.is_err(), "Completion should fail when plan inactive");
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
    let (meditation_plan, vault) = create_standard_plan(&mut svm, &harness);

    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    let new_plan = MeditationPlan {
        is_completed: true,
        ..plan.clone()
    };
    set_meditation_plan(&mut svm, meditation_plan, new_plan);

    let result = execute_complete(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        meditation_plan,
        vault,
    );
    assert!(
        result.is_err(),
        "Completion should fail when plan already completed"
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
fn test_plan_not_ended_fails() {
    let (mut svm, harness) = TestHarness::new();
    let (meditation_plan, vault) = create_standard_plan(&mut svm, &harness);
    let (_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);

    set_clock(&mut svm, plan.end_at - 1);
    let result = execute_complete(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        meditation_plan,
        vault,
    );
    assert!(
        result.is_err(),
        "Completion should fail when plan end date not reached"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Error Code: PlanNotEnded"),
        "Incorrect error for plan not ended"
    );
}

use solana_kite::get_token_account_balance;
use solana_pubkey::Pubkey;
use solana_signer::Signer;

use crate::constants::DAY_IN_SECONDS;
use crate::test_helpers::{
    airdrop_usdc, execute_initialize, generate_id, get_meditation_plan, TestHarness,
    COMMITMENT_STAKE, DAILY_FREQUENCY, DURATION_MINUTES, FIFTY_USDC, HUNDY_USDC, NUMBER_OF_DAYS,
    USDC_TOKEN,
};

#[test]
fn test_initialize_succeeds() {
    let (mut svm, harness) = TestHarness::new();

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), HUNDY_USDC);

    let id = generate_id();
    let result = execute_initialize(
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
    assert!(result.is_ok(), "Initialize should succeed");

    let (meditation_plan, meditation_bump, vault) = result.unwrap();

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    let balance = get_token_account_balance(&svm, &vault);
    assert_eq!(balance.unwrap(), FIFTY_USDC);

    let (plan_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    assert_eq!(plan_account.owner, harness.program_id);

    assert_eq!(plan.attestations.len(), 0);
    assert_eq!(plan.bump, meditation_bump);
    assert_eq!(plan.commitment_stake, COMMITMENT_STAKE);
    assert_eq!(plan.daily_frequency, DAILY_FREQUENCY);
    assert_eq!(plan.duration_minutes, DURATION_MINUTES);
    assert_eq!(plan.id, id);
    assert_eq!(plan.is_active, true);
    assert_eq!(plan.is_completed, false);
    assert_eq!(plan.number_of_days, NUMBER_OF_DAYS);
    assert_eq!(plan.owner, harness.alice.pubkey());
    assert_eq!(plan.penalties, 0);
    assert_eq!(plan.rewards, 0);
    assert_eq!(plan.start_at, 0);
    assert_eq!(plan.end_at, NUMBER_OF_DAYS as i64 * DAY_IN_SECONDS);
}

#[test]
fn test_duplicate_id_fails() {
    let (mut svm, harness) = TestHarness::new();
    let id = generate_id();

    let result = execute_initialize(
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
    assert!(result.is_ok(), "First initialize should succeed");

    let result = execute_initialize(
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
    assert!(result.is_err(), "Second call with same id should fail");

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.bob,
        harness.bob_usdc_account,
        id,
        NUMBER_OF_DAYS,
        DAILY_FREQUENCY,
        DURATION_MINUTES,
        COMMITMENT_STAKE,
    );
    assert!(
        result.is_ok(),
        "Bob should be able to initialize with same id"
    );
}

#[test]
fn test_insufficient_usdc_fails() {
    let (mut svm, harness) = TestHarness::new();
    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), HUNDY_USDC);

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        NUMBER_OF_DAYS,
        DAILY_FREQUENCY,
        DURATION_MINUTES,
        USDC_TOKEN * 150,
    );
    assert!(result.is_err(), "USDC balance should be insufficient");
}

#[test]
fn test_non_usdc_commitment_stake_fails() {
    let (mut svm, harness) = TestHarness::new();

    let result = execute_initialize(
        &mut svm,
        Pubkey::new_unique(),
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        NUMBER_OF_DAYS,
        DAILY_FREQUENCY,
        DURATION_MINUTES,
        COMMITMENT_STAKE,
    );
    assert!(result.is_err(), "Non-USDC token should fail");
}

#[test]
fn test_commitment_stake_below_minimum_fails() {
    let (mut svm, harness) = TestHarness::new();

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        NUMBER_OF_DAYS,
        DAILY_FREQUENCY,
        DURATION_MINUTES,
        USDC_TOKEN * 9,
    );
    assert!(result.is_err(), "commitment stake below 10 should fail");
}

#[test]
fn test_commitment_stake_above_maximum_fails() {
    let (mut svm, harness) = TestHarness::new();

    airdrop_usdc(
        &mut svm,
        harness.usdc_mint,
        harness.alice.pubkey(),
        USDC_TOKEN * 600,
    );

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), USDC_TOKEN * 600);

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        NUMBER_OF_DAYS,
        DAILY_FREQUENCY,
        DURATION_MINUTES,
        USDC_TOKEN * 501,
    );
    assert!(result.is_err(), "commitment stake above 500 should fail");
}

#[test]
fn test_number_of_days_below_minimum_fails() {
    let (mut svm, harness) = TestHarness::new();

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        6,
        DAILY_FREQUENCY,
        DURATION_MINUTES,
        COMMITMENT_STAKE,
    );
    assert!(result.is_err(), "number of days below 7 should fail");
}

#[test]
fn test_number_of_days_above_maximum_fails() {
    let (mut svm, harness) = TestHarness::new();

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        31,
        DAILY_FREQUENCY,
        DURATION_MINUTES,
        COMMITMENT_STAKE,
    );
    assert!(result.is_err(), "number of days above 30 should fail");
}

#[test]
fn test_daily_frequency_below_minimum_fails() {
    let (mut svm, harness) = TestHarness::new();

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        NUMBER_OF_DAYS,
        0,
        DURATION_MINUTES,
        COMMITMENT_STAKE,
    );
    assert!(result.is_err(), "daily frequency below 1 should fail");
}

#[test]
fn test_daily_frequency_above_maximum_fails() {
    let (mut svm, harness) = TestHarness::new();

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        NUMBER_OF_DAYS,
        5,
        DURATION_MINUTES,
        COMMITMENT_STAKE,
    );
    assert!(result.is_err(), "daily frequency above 4 should fail");
}

#[test]
fn test_duration_minutes_below_minimum_fails() {
    let (mut svm, harness) = TestHarness::new();

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        NUMBER_OF_DAYS,
        DAILY_FREQUENCY,
        4,
        COMMITMENT_STAKE,
    );
    assert!(result.is_err(), "duration minutes below 5 should fail");
}

#[test]
fn test_duration_minutes_above_maximum_fails() {
    let (mut svm, harness) = TestHarness::new();

    let result = execute_initialize(
        &mut svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        NUMBER_OF_DAYS,
        DAILY_FREQUENCY,
        61,
        COMMITMENT_STAKE,
    );
    assert!(result.is_err(), "duration minutes above 60 should fail");
}

use solana_kite::get_token_account_balance;
use solana_signer::Signer;

use crate::test_helpers::{
    execute_initialize, generate_id, get_meditation_plan, TestHarness, USDC_TOKEN,
};

#[test]
fn test_initialize_succeeds() {
    let (mut svm, harness) = TestHarness::new();

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), USDC_TOKEN * 100);

    let id = generate_id();
    let number_of_days: u8 = 7;
    let daily_frequency: u8 = 1;
    let duration_minutes: u8 = 20;
    let commitment_stake: u64 = USDC_TOKEN * 50;

    let (meditation_plan, meditation_bump, _vault) = execute_initialize(
        &mut svm,
        &harness,
        id,
        &harness.alice,
        harness.alice_usdc_account,
        number_of_days,
        daily_frequency,
        duration_minutes,
        commitment_stake,
    )
    .expect("Initialize  meditation plan should succeed");

    let balance = get_token_account_balance(&svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), USDC_TOKEN * 50);

    let (plan_account, plan) = get_meditation_plan(&mut svm, &meditation_plan);
    assert_eq!(plan_account.owner, harness.program_id);

    assert_eq!(plan.attestations.len(), 0);
    assert_eq!(plan.bump, meditation_bump);
    assert_eq!(plan.commitment_stake, commitment_stake);
    assert_eq!(plan.daily_frequency, daily_frequency);
    assert_eq!(plan.duration_minutes, duration_minutes);
    assert_eq!(plan.id, id);
    assert_eq!(plan.is_active, false);
    assert_eq!(plan.is_completed, false);
    assert_eq!(plan.number_of_days, number_of_days);
    assert_eq!(plan.owner, harness.alice.pubkey());
    assert_eq!(plan.penalties, 0);
    assert_eq!(plan.rewards, 0);
    assert_eq!(plan.start_at, 0);
    assert_eq!(plan.end_at, number_of_days as i64 * 24 * 60 * 60);
}

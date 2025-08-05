use anchor_lang::AccountDeserialize;
use solana_kite::{
    get_pda_and_bump, get_token_account_balance, seeds, send_transaction_from_instructions,
};
use solana_signer::Signer;

use crate::test_helpers::{
    build_initialize_accounts, build_initialize_instruction, generate_id, TestHarness, USDC_TOKEN,
};
use crate::MeditationPlan;

#[test]
fn test_initialize_succeeds() {
    let mut harness = TestHarness::new();

    let balance = get_token_account_balance(&harness.svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), USDC_TOKEN * 100);

    let id = generate_id();
    let (meditation_plan, _meditation_bump) = get_pda_and_bump(
        &seeds!["meditation_plan", harness.alice.pubkey(), id],
        &harness.program_id,
    );
    let vault = spl_associated_token_account::get_associated_token_address(
        &meditation_plan,
        &harness.usdc_mint,
    );

    let initialize_accounts = build_initialize_accounts(
        harness.alice.pubkey(),
        harness.usdc_mint,
        harness.alice_usdc_account,
        meditation_plan,
        vault,
    );

    let number_of_days: u8 = 7;
    let daily_frequency: u8 = 1;
    let duration_minutes: u8 = 20;
    let commitment_stake: u64 = USDC_TOKEN * 50;

    let initialize_instruction = build_initialize_instruction(
        id,
        number_of_days,
        daily_frequency,
        duration_minutes,
        commitment_stake,
        initialize_accounts,
    );

    let result = send_transaction_from_instructions(
        &mut harness.svm,
        vec![initialize_instruction],
        &[&harness.alice],
        &harness.alice.pubkey(),
    );

    assert!(result.is_ok(), "Valid meditation plan should succeed");
    let balance = get_token_account_balance(&harness.svm, &harness.alice_usdc_account);
    assert_eq!(balance.unwrap(), USDC_TOKEN * 50);

    let raw_plan_account = harness.svm.get_account(&meditation_plan).unwrap();
    assert_eq!(raw_plan_account.owner, harness.program_id);

    let plan = MeditationPlan::try_deserialize(&mut raw_plan_account.data.as_slice())
        .expect("Anchor deserialize should succeed");
    assert_eq!(plan.attestations.len(), 0);
    assert!(plan.bump > 0);
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

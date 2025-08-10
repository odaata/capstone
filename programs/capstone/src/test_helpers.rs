use anchor_lang::{AccountDeserialize, AnchorSerialize, Discriminator};
use litesvm::LiteSVM;
use solana_account::Account;
use solana_clock::Clock;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_kite::{
    deploy_program, get_pda_and_bump, seeds, send_transaction_from_instructions, SolanaKiteError,
};
use solana_program_option::COption;
use solana_program_pack::Pack;
use solana_pubkey::{pubkey, Pubkey};
use solana_signer::Signer;
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::{Account as TokenAccount, AccountState as TokenAccountState};
use spl_token::ID as TOKEN_PROGRAM_ID;
use std::cell::Cell;
use std::str::FromStr;

use crate::MeditationPlan;

pub const PROGRAM_ID: &str = "Bvw5aYMCJDM1136hC5GLqmtq1LbsqSKEgC4owCQj9ZYm";

pub const USDC_MINT: &str = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";

/// Standard token unit for USDC for 6 decimals)
pub const USDC_TOKEN: u64 = 1_000_000;
pub const FIFTY_USDC: u64 = 50 * USDC_TOKEN;
pub const HUNDY_USDC: u64 = 100 * USDC_TOKEN;
pub const REWARDS_PER_SESSION: u64 = FIFTY_USDC / 7;

// Valid settings for initializing meditation plans in tests
pub const COMMITMENT_STAKE: u64 = FIFTY_USDC;
pub const DAILY_FREQUENCY: u8 = 1;
pub const DURATION_MINUTES: u8 = 20;
pub const NUMBER_OF_DAYS: u8 = 7;

pub const STARTED_AT: i64 = 0; // Use 0 for testing
pub const ENDED_AT: i64 = 30 * 60; // 30 minutes later

// Holds everything needed to test the meditation plan contract
pub struct TestHarness {
    /// The program ID
    pub program_id: Pubkey,
    /// USDC Token mint
    pub usdc_mint: Pubkey,
    /// Alice's keypair
    pub alice: Keypair,
    /// Alice's USDC token account
    pub alice_usdc_account: Pubkey,
    /// Bob's keypair
    pub bob: Keypair,
    /// Bob's USDC token account
    pub bob_usdc_account: Pubkey,
}

impl TestHarness {
    pub fn new() -> (LiteSVM, Self) {
        let mut svm = LiteSVM::new();
        let program_id = get_program_id();
        deploy_program(&mut svm, &program_id, "../../target/deploy/capstone.so").unwrap();

        let usdc_mint = create_usdc_mint(&mut svm, None);

        // Create and fund user accounts
        let alice = Keypair::new();
        svm.airdrop(&alice.pubkey(), 1_000_000_000).unwrap();

        let alice_usdc_account =
            airdrop_usdc(&mut svm, usdc_mint, alice.pubkey(), 100 * USDC_TOKEN);

        let bob = Keypair::new();
        svm.airdrop(&bob.pubkey(), 1_000_000_000).unwrap();

        let bob_usdc_account = airdrop_usdc(&mut svm, usdc_mint, bob.pubkey(), 100 * USDC_TOKEN);

        (
            svm,
            TestHarness {
                alice,
                alice_usdc_account,
                bob,
                bob_usdc_account,
                program_id,
                usdc_mint,
            },
        )
    }
}

pub fn set_clock(svm: &mut LiteSVM, unix_timestamp: i64) {
    let mut clock = svm.get_sysvar::<Clock>();
    clock.unix_timestamp = unix_timestamp;
    svm.set_sysvar::<Clock>(&clock);
}

fn create_usdc_mint(svm: &mut LiteSVM, usdc_mint_override: Option<Pubkey>) -> Pubkey {
    let usdc_mint = usdc_mint_override.unwrap_or_else(|| Pubkey::from_str(USDC_MINT).unwrap());

    // Initialize the USDC Mint based on the account downloaded from devnet
    // see @link https://solana.stackexchange.com/questions/23006/how-to-generate-usdt-mint-in-litesvm-anchor-tests
    let usdc_mint_account_info = Account {
        lamports: 156551902655,
        data: vec![
            1, 0, 0, 0, 235, 133, 68, 207, 145, 24, 39, 122, 150, 50, 233, 153, 80, 102, 101, 27,
            191, 177, 64, 139, 133, 115, 235, 192, 46, 83, 239, 171, 90, 191, 14, 11, 27, 183, 254,
            156, 226, 97, 54, 202, 6, 1, 1, 0, 0, 0, 168, 6, 51, 255, 6, 125, 136, 223, 165, 212,
            53, 123, 244, 78, 38, 206, 204, 207, 8, 102, 104, 129, 3, 198, 186, 96, 159, 164, 76,
            24, 210, 8,
        ],
        executable: false,
        owner: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        // Actual data was too big of a number - not sure if this matters
        rent_epoch: 1_000_000_000,
    };
    svm.set_account(usdc_mint, usdc_mint_account_info).unwrap();
    usdc_mint
}

pub fn create_fake_usdc_mint(svm: &mut LiteSVM) -> Pubkey {
    create_usdc_mint(svm, Some(Pubkey::new_unique()))
}

pub fn airdrop_usdc(
    svm: &mut LiteSVM,
    usdc_mint: Pubkey,
    recipient: Pubkey,
    amount: u64,
) -> Pubkey {
    let usdc_account = get_associated_token_address(&recipient, &usdc_mint);
    let token_acc = TokenAccount {
        mint: usdc_mint,
        owner: recipient,
        amount,
        delegate: COption::None,
        state: TokenAccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };
    let mut token_acc_bytes = [0u8; TokenAccount::LEN];
    TokenAccount::pack(token_acc, &mut token_acc_bytes).unwrap();
    svm.set_account(
        usdc_account,
        Account {
            lamports: 1_000_000_000,
            data: token_acc_bytes.to_vec(),
            owner: TOKEN_PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
    usdc_account
}

pub fn get_program_id() -> Pubkey {
    Pubkey::from_str(PROGRAM_ID).unwrap()
}

thread_local! {
    static ID_COUNTER: Cell<u64> = Cell::new(1);
}

/// Generates a unique ID for testing meditation plans
///
/// This function returns incrementing offer IDs starting from 1, ensuring
/// each test gets unique IDs to avoid conflicts between test cases.
pub fn generate_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let id = counter.get();
        counter.set(id + 1);
        id
    })
}

pub fn get_initialize_discriminator() -> Vec<u8> {
    let discriminator_input = b"global:initialize";
    anchor_lang::solana_program::hash::hash(discriminator_input).to_bytes()[..8].to_vec()
}

pub fn get_attest_discriminator() -> Vec<u8> {
    let discriminator_input = b"global:attest";
    anchor_lang::solana_program::hash::hash(discriminator_input).to_bytes()[..8].to_vec()
}

pub fn get_complete_discriminator() -> Vec<u8> {
    let discriminator_input = b"global:complete";
    anchor_lang::solana_program::hash::hash(discriminator_input).to_bytes()[..8].to_vec()
}

pub fn get_meditation_plan(
    svm: &mut LiteSVM,
    meditation_plan: &Pubkey,
) -> (Account, MeditationPlan) {
    let plan_account = svm.get_account(&meditation_plan).unwrap();

    let plan = MeditationPlan::try_deserialize(&mut plan_account.data.as_slice())
        .expect("Anchor deserialize should succeed");
    (plan_account, plan)
}

pub fn set_meditation_plan(
    svm: &mut LiteSVM,
    meditation_plan: Pubkey,
    plan: MeditationPlan,
) -> (Account, MeditationPlan) {
    let plan_account = svm.get_account(&meditation_plan).unwrap();

    let mut data = MeditationPlan::DISCRIMINATOR.to_vec();
    data.extend(plan.try_to_vec().expect("Anchor serialize should succeed"));
    // plan.try_serialize(&mut data)
    svm.set_account(
        meditation_plan,
        Account {
            lamports: plan_account.lamports,
            data,
            owner: plan_account.owner,
            executable: plan_account.executable,
            rent_epoch: plan_account.rent_epoch,
        },
    )
    .unwrap();
    (plan_account, plan)
}

pub struct InitializeAccounts {
    pub associated_token_program: Pubkey,
    pub meditation_plan: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub owner_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub vault: Pubkey,
}

/// Helper function to create MakeOfferAccounts with standard program IDs
///
/// This function eliminates the repetitive initialization of the three standard
/// program IDs (associated_token_program, token_program, system_program) that
/// are always the same constants across all tests. Instead of copy-pasting
/// these three lines in every test, this helper focuses on the variable fields.
fn build_initialize_accounts(
    owner: Pubkey,
    mint: Pubkey,
    owner_ata: Pubkey,
    meditation_plan: Pubkey,
    vault: Pubkey,
) -> InitializeAccounts {
    InitializeAccounts {
        associated_token_program: spl_associated_token_account::ID,
        token_program: spl_token::ID,
        system_program: anchor_lang::system_program::ID,
        owner,
        mint,
        owner_ata,
        meditation_plan,
        vault,
    }
}

fn build_initialize_instruction(
    id: u64,
    number_of_days: u8,
    daily_frequency: u8,
    duration_minutes: u8,
    commitment_stake: u64,
    accounts: InitializeAccounts,
) -> Instruction {
    let mut instruction_data = get_initialize_discriminator();
    instruction_data.extend_from_slice(&id.to_le_bytes());
    instruction_data.extend_from_slice(&number_of_days.to_le_bytes());
    instruction_data.extend_from_slice(&daily_frequency.to_le_bytes());
    instruction_data.extend_from_slice(&duration_minutes.to_le_bytes());
    instruction_data.extend_from_slice(&commitment_stake.to_le_bytes());

    let account_metas = vec![
        AccountMeta::new(accounts.owner, true),
        AccountMeta::new(accounts.meditation_plan, false),
        AccountMeta::new_readonly(accounts.mint, false),
        AccountMeta::new(accounts.owner_ata, false),
        AccountMeta::new(accounts.vault, false),
        AccountMeta::new_readonly(accounts.associated_token_program, false),
        AccountMeta::new_readonly(accounts.token_program, false),
        AccountMeta::new_readonly(accounts.system_program, false),
    ];

    Instruction {
        program_id: get_program_id(),
        accounts: account_metas,
        data: instruction_data,
    }
}

/// Initializes a meditation plan and sends USDC to vault
pub fn execute_initialize(
    svm: &mut LiteSVM,
    usdc_mint: Pubkey,
    owner: &Keypair,
    owner_ata: Pubkey,
    id: u64,
    number_of_days: u8,
    daily_frequency: u8,
    duration_minutes: u8,
    commitment_stake: u64,
) -> Result<(Pubkey, u8, Pubkey), SolanaKiteError> {
    // Create PDAs
    let (meditation_plan, meditation_bump) = get_pda_and_bump(
        &seeds!["meditation_plan", owner.pubkey(), id],
        &get_program_id(),
    );
    let vault =
        spl_associated_token_account::get_associated_token_address(&meditation_plan, &usdc_mint);

    // Build accounts
    let initialize_accounts =
        build_initialize_accounts(owner.pubkey(), usdc_mint, owner_ata, meditation_plan, vault);

    // Build and execute instruction
    let initialize_instruction = build_initialize_instruction(
        id,
        number_of_days,
        daily_frequency,
        duration_minutes,
        commitment_stake,
        initialize_accounts,
    );

    send_transaction_from_instructions(
        svm,
        vec![initialize_instruction],
        &[owner],
        &owner.pubkey(),
    )?;

    Ok((meditation_plan, meditation_bump, vault))
}

pub fn create_standard_plan(svm: &mut LiteSVM, harness: &TestHarness) -> (Pubkey, Pubkey) {
    let (meditation_plan, _meditation_bump, vault) = execute_initialize(
        svm,
        harness.usdc_mint,
        &harness.alice,
        harness.alice_usdc_account,
        generate_id(),
        NUMBER_OF_DAYS,
        DAILY_FREQUENCY,
        DURATION_MINUTES,
        COMMITMENT_STAKE,
    )
    .expect("Initialization should succeed");

    // Set the clock to be after the ended_at time so a standard attestation is valid by default
    set_clock(svm, ENDED_AT + 1);

    (meditation_plan, vault)
}

// Attest helpers
pub struct AttestAccounts {
    pub attester: Pubkey,
    pub meditation_plan: Pubkey,
    pub system_program: Pubkey,
}

fn build_attest_accounts(attester: Pubkey, meditation_plan: Pubkey) -> AttestAccounts {
    AttestAccounts {
        system_program: anchor_lang::system_program::ID,
        attester,
        meditation_plan,
    }
}

fn build_attest_instruction(
    started_at: i64,
    ended_at: i64,
    accounts: AttestAccounts,
) -> Instruction {
    let mut instruction_data = get_attest_discriminator();
    instruction_data.extend_from_slice(&started_at.to_le_bytes());
    instruction_data.extend_from_slice(&ended_at.to_le_bytes());

    let account_metas = vec![
        AccountMeta::new(accounts.attester, true),
        AccountMeta::new(accounts.meditation_plan, false),
        AccountMeta::new_readonly(accounts.system_program, false),
    ];

    Instruction {
        program_id: get_program_id(),
        accounts: account_metas,
        data: instruction_data,
    }
}

pub fn execute_attest(
    svm: &mut LiteSVM,
    attester: &Keypair,
    meditation_plan: Pubkey,
    started_at: i64,
    ended_at: i64,
) -> Result<(), SolanaKiteError> {
    let accounts = build_attest_accounts(attester.pubkey(), meditation_plan);
    let instruction = build_attest_instruction(started_at, ended_at, accounts);
    send_transaction_from_instructions(svm, vec![instruction], &[attester], &attester.pubkey())
}

// Complete helpers
pub struct CompleteAccounts {
    pub associated_token_program: Pubkey,
    pub meditation_plan: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub owner_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub vault: Pubkey,
}

fn build_complete_accounts(
    owner: Pubkey,
    mint: Pubkey,
    owner_ata: Pubkey,
    meditation_plan: Pubkey,
    vault: Pubkey,
) -> CompleteAccounts {
    CompleteAccounts {
        associated_token_program: spl_associated_token_account::ID,
        token_program: spl_token::ID,
        system_program: anchor_lang::system_program::ID,
        owner,
        mint,
        owner_ata,
        meditation_plan,
        vault,
    }
}

fn build_complete_instruction(accounts: CompleteAccounts) -> Instruction {
    let instruction_data = get_complete_discriminator();

    let account_metas = vec![
        AccountMeta::new(accounts.owner, true),
        AccountMeta::new(accounts.meditation_plan, false),
        AccountMeta::new_readonly(accounts.mint, false),
        AccountMeta::new(accounts.owner_ata, false),
        AccountMeta::new(accounts.vault, false),
        AccountMeta::new_readonly(accounts.associated_token_program, false),
        AccountMeta::new_readonly(accounts.token_program, false),
        AccountMeta::new_readonly(accounts.system_program, false),
    ];

    Instruction {
        program_id: get_program_id(),
        accounts: account_metas,
        data: instruction_data,
    }
}

pub fn execute_complete(
    svm: &mut LiteSVM,
    usdc_mint: Pubkey,
    owner: &Keypair,
    owner_ata: Pubkey,
    meditation_plan: Pubkey,
    vault: Pubkey,
) -> Result<(), SolanaKiteError> {
    let accounts =
        build_complete_accounts(owner.pubkey(), usdc_mint, owner_ata, meditation_plan, vault);
    let instruction = build_complete_instruction(accounts);
    send_transaction_from_instructions(svm, vec![instruction], &[owner], &owner.pubkey())
}

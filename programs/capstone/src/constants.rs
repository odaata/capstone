use anchor_lang::prelude::*;

#[constant]
pub const ANCHOR_DISCRIMINATOR: u8 = 8;

// Choose USDC mint based on feature flags
#[constant]
#[cfg(feature = "mainnet")]
pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

#[constant]
#[cfg(feature = "devnet")]
pub const USDC_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

// Default to localnet/testing mint if no feature is specified
#[constant]
#[cfg(not(any(feature = "mainnet", feature = "devnet")))]
pub const USDC_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

# Turbin3 Capstone Project

## Overview

SOL Journey is a Solana-based mobile dApp that provides a spiritual toolkit for users to level up their spiritual
practices, reflect on their progress, and connect with other spiritual travelers. The app leverages the Anchor framework
for secure, on-chain attestation of meditation sessions and journaling, while maintaining user privacy through
wallet-based authentication.

This was built for the [Turbin3 Q3 Builders Cohort](https://turbin3.org).

## Features (MVP)

- **Meditation Plan Initialization:** Users can initialize a meditation plan by setting length, daily frequency, and
  USDC commitment stake.
- **Meditation Session Attestations:** Users record meditation sessions with start and end times. For each successful
  meditation session, users earn rewards back from their commitment stake.
- **Meditation Plan Completion:** Users complete their meditation plan when the rewards and penalties are finalized and
  USDC is returned to the meditator.

## Technology Stack

- **Solana Blockchain**
- **Anchor Framework (Rust)**
- **LiteSVM for testing**

## Installation

- Install Rust and Anchor and any dependencies required for your environment.
    - See the [Anchor Installation Guide](https://www.anchor-lang.com/docs/installation)

```bash
git clone git@github.com:odaata/capstone.git
cd capstone
anchor build
anchor test
```

## Usage

1. Connect your Solana wallet.
2. `Initialize` a meditation plan by specifying the length, daily frequency, and USDC commitment stake.
3. `Attest` meditation sessions every day by submitting start and end times.
4. `Complete` the meditation plan to finalize rewards and penalties, and receive USDC back.

## Development

- Rust programs are located in `/programs`.
- Anchor tests and scripts are in `programs/capstone/src/tests`.
- Typescript tests are not working yet with `LiteSVM`, so only Rust tests are available.

## Future Roadmap

- **On-chain Meditation Attestation:** Users record meditation sessions on Solana, optionally connecting biometric
  devices for proof-of-meditation.
- **Private Journaling:** Daily reflections are stored securely and privately, accessible only via the user's wallet.
- **Soul-bound NFTs:** Users earn non-transferable NFTs representing spiritual milestones and reputation.
- **Reputation System:** Progress and insights can be shared with the community, building a reputation based on verified
  activity.
- **DeFi Integration:** Users can utilize their assets in DeFi protocols while pursuing spiritual goals.

## References

* [Assignments](./assignments)
* [Documentation](./docs)

## License

[MIT](LICENSE)

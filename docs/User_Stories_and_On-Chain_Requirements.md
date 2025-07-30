# User Stories & On-Chain Requirements

## Top Users for PoC

- Individuals Learning to Meditate

Rationale: This group represents the core problem the project aims to solve. While the other user types define market
segments, this group defines the need. The POC must prove that the blend of financial incentives and spiritual practice
actually helps these individuals build and maintain a meditation habit. Their feedback is vital to confirm that the dApp
is not just a novel financial tool, but an effective wellness toolkit, which is the heart of the value proposition.

## Core Function Mapping

- **Onboarding and Goal Setting:**
    - **Create a Plan:** Define their meditation goals, such as frequency (e.g., 5 times a week) and duration (e.g., 10 minutes per session).
    - **Simplified Setup:** Go through a quick, non-technical tutorial on how the app helps them stick to their plan.
    - **Stake Crypto:** Stake crypto to start the plan from the first step

- **Core Practice Loop:**
    - **Start a Session:** Initiate a meditation session.
    - **Practice Meditation:** Use the in-app timer to track meditation session.
    - **Confirm Completion:** Attest that they have completed the session. This is the key action that interacts with their commitment stake.
    - **Private Journaling:** Record thoughts and reflections in a private, encrypted journal after each session to track their inner journey.

- **Tracking and Motivation:**
    - **View Progress:** Check a personal dashboard to see their streak, total meditation time and rewards earned.
    - **Community Feed:** See an anonymous global feed of others who are currently meditating or have recently finished, creating a sense of shared, collective effort

## Core PoC Requirements

### 1. Technical Requirements for the "Core Practice Loop"
This is the most critical interaction, as it combines the wellness activity with the on-chain "Commit-to-Earn" mechanic.
- **Solana Program (Smart Contract):**
    - **State Management:** Needs a program to create and manage a `SoloPool` account for each user. This account will store the user's plan details (e.g., , , ) and track their progress (`status`, ). `stake_amount``daily_goal_minutes``challenge_duration``completion_rate`
    - **Staking Function:** An instruction () that allows a user to define their meditation plan and transfer their stake (e.g., SOL) into a Program-Owned Account. `create_solo_pool`
    - **Attestation Function:** A core instruction () that a user calls to confirm a completed meditation session. This function must contain the logic to verify that the user is part of an active challenge and to update their progress. `submit_attestation`
    - **Reward/Claim Logic:** A basic instruction () to calculate and release a portion of the staked funds back to the user upon a successful attestation. For a POC, this can be a simple, proportional release. `claim_solo_rewards`

- **Frontend Application (Next.js/React):**
    - **Wallet Integration:** Must connect to a Solana wallet (e.g., Phantom, Solflare) using libraries like `@solana/wallet-adapter`. This is non-negotiable, as it's required to sign all on-chain transactions.
    - **UI for Plan Creation:** A simple form where users can input their desired stake amount and meditation goal (duration/frequency). This will be used to call the instruction. `create_solo_pool`
    - **Meditation Timer:** A basic, time-based component in the UI. When the timer completes, it should enable the "Confirm Completion" button.
    - **Interaction with Program:** The frontend must be able to serialize instruction data and send transactions to the Solana program to call the function when the user confirms a session. `submit_attestation`

- **Data and Privacy:**
    - **Client-Side Encryption for Journal:** To implement the private journal, the application must use the user's wallet to generate a key to encrypt journal entries _before_ they are stored. The wallet signature can be used for this, ensuring only the user can decrypt their own entries.
    - **Local or Decentralized Storage:** For the POC, journal entries can be stored simply in the browser's `localStorage`. For a more robust solution, a decentralized storage option like Shadow Drive could be used, with access gated by the user's wallet.

### 2. Technical Requirements for "Tracking and Motivation"
This interaction focuses on providing feedback to the user to keep them engaged.
- **Frontend Application (Next.js/React):**
    - **Read On-Chain State:** The UI needs to query the user's `SoloPool` account from the Solana blockchain. It will use the user's public key to find their specific account.
    - **Personal Dashboard:** A component that parses the on-chain data (, , etc.) and displays it in a user-friendly way (e.g., "You've earned X rewards," "Your streak is Y days"). `stake_amount``rewards_claimed`
    - **Global Feed (Simplified):** For a POC, this doesn't need to be complex. It can be a simple "Recent Activity" list that reads the last few transactions from the blockchain and displays the (anonymous) public key of the user who completed a session. This demonstrates the community aspect without requiring a centralized database. `submit_attestation`

## User Stories

- User Story: "User creates a new meditation plan."
  - Account to hold the meditation plan including duration, frequency, rewards and penalty for missing
    - How many weeks will the plan run? (1-4)
    - How many times within a day they commit to meditate? (1-4)
    - How long is each meditation in minutes? (5 >= l <= 60)
  - Vault to hold commitment stake
    - Limited to under $500 for PoC
  - DeFi rewards come from Lulo, so sell toke for Lulo token and hold in vault
  - Penalty for the PoC is to donate to charity - init charity vault if needed

- User Story: "User submits meditation attestation"
  - Validations:
    - Session duration >= commitment duration
    - Have not completed all sessions for the current 24-hour period
  - Update meditation account to release rewards for each session and update the total released
    - Simple percentage calculation for the PoC

- User Story: "User claims rewards"
  - Validations:
    - For PoC, rewards can only be claimed when the meditation plan is complete
  - Convert stake from Lulo back into original token
  - Send to user's wallet
  - Send penalties for missed sessions to community charity vault
  - Query for attestation transactions to compile final totals for meditaiton plan
    - Total rewards
    - Total penalties
    - Total interest earned

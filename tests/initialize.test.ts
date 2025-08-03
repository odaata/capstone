import { BN } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

import { loadSvm, setupUSDC, USDC_MINT } from "./helpers";

const { program, provider, svm } = loadSvm();

describe("capstone initialize", () => {
  let commitmentStake: BN;
  let dailyFrequency: number;
  let durationMinutes: number;
  let id: BN;
  let numberOfDays: number;

  const subject = async () => {
    const tx = await program.methods
      .initialize(
        id,
        numberOfDays,
        dailyFrequency,
        durationMinutes,
        commitmentStake,
      )
      .accounts({
        mint: USDC_MINT,
        owner: provider.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  };

  beforeAll(async () => {
    await setupUSDC(svm, provider.publicKey);
  });

  beforeEach(() => {
    id = new BN(5);
    commitmentStake = new BN(250);
    dailyFrequency = 2;
    durationMinutes = 20;
    numberOfDays = 30;
  });

  it("initializes meditation plan and transfers USDC", async () => {
    await subject();

    const [planStatePDA, planStateBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("meditation_plan"),
        provider.publicKey.toBuffer(),
        id.toBuffer(),
      ],
      program.programId,
    );

    const planState = await program.account.meditationPlan.fetch(planStatePDA);
    expect(planState).toStrictEqual({
      attestations: [],
      bump: planStateBump,
      commitmentStake,
      dailyFrequency,
      durationMinutes,
      endAt: Date.now() + numberOfDays * 24 * 60 * 60,
      id,
      isActive: false,
      isCompleted: false,
      numberOfDays,
      owner: provider.publicKey,
      penalties: 0,
      rewards: 0,
      start_at: Date.now(),
    });
  });
});

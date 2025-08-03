import { BN } from "@coral-xyz/anchor";
import { getAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

import type { CapstoneProgram } from "./helpers";
import { airdropUsdc, loadSvm, USDC_MINT } from "./helpers";
import { LiteSVM } from "litesvm";
import { LiteSVMProvider } from "anchor-litesvm";

describe("capstone initialize", () => {
  let commitmentStake: BN;
  let dailyFrequency: number;
  let durationMinutes: number;
  let id: BN;
  let numberOfDays: number;
  let program: CapstoneProgram;
  let provider: LiteSVMProvider;
  let svm: LiteSVM;
  let usdcAta: PublicKey;

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

  beforeEach(() => {
    jest.resetModules();
    const result = loadSvm();
    program = result.program;
    provider = result.provider;
    svm = result.svm;

    id = new BN(Date.now());
    commitmentStake = new BN(250);
    dailyFrequency = 2;
    durationMinutes = 20;
    numberOfDays = 30;
  });

  it("initializes meditation plan and transfers USDC", async () => {
    usdcAta = airdropUsdc(svm, provider.publicKey, 500n);

    let usdcAccountInfo = await getAccount(provider.connection, usdcAta);
    expect(usdcAccountInfo.amount).toBe(500n);

    await subject();

    usdcAccountInfo = await getAccount(provider.connection, usdcAta);
    expect(usdcAccountInfo.amount).toBe(250n);

    const [planStatePDA, planStateBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("meditation_plan"),
        provider.publicKey.toBuffer(),
        id.toArrayLike(Buffer, "le", 8),
      ],
      program.programId,
    );

    const planState = await program.account.meditationPlan.fetch(planStatePDA);
    expect(planState).toMatchObject({
      attestations: [],
      bump: planStateBump,
      dailyFrequency,
      durationMinutes,
      isActive: false,
      isCompleted: false,
      numberOfDays,
      owner: provider.publicKey,
    });
    expect(planState.commitmentStake.toNumber()).toBe(
      commitmentStake.toNumber(),
    );
    expect(planState.id.toNumber()).toBe(id.toNumber());

    expect(planState.penalties.toNumber()).toBe(0);
    expect(planState.rewards.toNumber()).toBe(0);

    // LiteSVM clock returns 0 as the unix timestamp
    expect(planState.startAt.toNumber()).toBe(0);
    expect(planState.endAt.toNumber()).toBe(numberOfDays * (24 * 60 * 60));
  });

  describe("errors", () => {
    beforeEach(() => {
      usdcAta = airdropUsdc(svm, provider.publicKey, 500n);
    });

    describe.each([
      ["too low", 6],
      ["too high", 31],
    ])("when number of days %s", (_label, days) => {
      beforeEach(() => {
        numberOfDays = days;
      });

      it("throws InvalidNumberOfDays error", async () => {
        await expect(subject()).rejects.toThrow(
          "Number of days must be between 7 and 30",
        );
      });
    });
  });
});

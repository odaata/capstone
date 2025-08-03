import { Program } from "@coral-xyz/anchor";
import {
  ACCOUNT_SIZE,
  AccountLayout,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import type { LiteSVM } from "litesvm";
import { fromWorkspace, LiteSVMProvider } from "anchor-litesvm";

import IDL from "../target/idl/capstone.json";
import { Capstone } from "../target/types/capstone";
import usdcAccount from "./usdcAccount.json";

export type CapstoneProgram = Program<Capstone>;

export const USDC_MINT = new PublicKey(
  IDL.constants.find((c) => c.name === "USDC_MINT")?.value ?? "",
);

export const setupUSDC = (svm: LiteSVM) => {
  const usdcAccountInfo = {
    data: Buffer.from(usdcAccount.data.data),
    executable: false,
    lamports: usdcAccount.lamports,
    owner: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    rentEpoch: 1_000_000_000,
  };
  svm.setAccount(USDC_MINT, usdcAccountInfo);
};

export const airdropUsdc = (svm: LiteSVM, owner: PublicKey, amount = 500n) => {
  const usdcAta = getAssociatedTokenAddressSync(USDC_MINT, owner, true);
  const tokenAccData = Buffer.alloc(ACCOUNT_SIZE);
  AccountLayout.encode(
    {
      amount,
      closeAuthority: PublicKey.default,
      closeAuthorityOption: 0,
      delegate: PublicKey.default,
      delegateOption: 0,
      delegatedAmount: 0n,
      isNative: 0n,
      isNativeOption: 0,
      mint: USDC_MINT,
      owner,
      state: 1,
    },
    tokenAccData,
  );

  svm.setAccount(usdcAta, {
    lamports: 1_000_000_000,
    data: tokenAccData,
    owner: TOKEN_PROGRAM_ID,
    executable: false,
  });
  return usdcAta;
};

export const loadSvm = () => {
  const svm = fromWorkspace(".");
  svm.withSplPrograms();
  setupUSDC(svm);
  const provider = new LiteSVMProvider(svm);
  const program = new Program<Capstone>(IDL, provider);
  return { program, provider, svm };
};

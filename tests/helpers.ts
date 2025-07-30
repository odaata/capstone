import { Program } from "@coral-xyz/anchor";
import { fromWorkspace, LiteSVMProvider } from "anchor-litesvm";

import AMM_IDL from "../target/idl/capstone.json";
import { Capstone } from "../target/types/capstone";

export type CapstoneProgram = Program<Capstone>;

export const loadSvm = () => {
  const svm = fromWorkspace(".");
  const provider = new LiteSVMProvider(svm);
  const program = new Program<Capstone>(AMM_IDL, provider);
  return { program, provider, svm };
};

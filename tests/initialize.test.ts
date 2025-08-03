import { loadSvm } from "./helpers";

const { program, provider } = loadSvm();

describe("capstone initialize", () => {
  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({ initializer: provider.publicKey })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});

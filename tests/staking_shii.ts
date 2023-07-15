import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakingShii } from "../target/types/staking_shii";

describe("staking_shii", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StakingShii as Program<StakingShii>;
  const provider = anchor.getProvider();
  const wallet = anchor.web3.workspace.StakingShii.provider.wallet as anchor.web3.Keypair;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakingShii } from "../target/types/staking_shii";

describe("staking_shii", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StakingShii as Program<StakingShii>;
  const provider = anchor.getProvider();
  const wallet = anchor.web3.workspace.StakingShii.provider.wallet as anchor.web3.Keypair;

  let stakeInfoAccount = anchor.web3.PublicKey
  let userInfoAccount = anchor.web3.PublicKey
  let StakeInfoTokenAccount = anchor.web3.PublicKey
  let mint = anchor.web3.PublicKey
  let UserInfoTokenAccount = anchor.web3.PublicKey


  before(async () => {
   
    userInfoAccount = anchor.web3.PublicKey.createProgramAddressSync(
      [Buffer.from("USER"), provider.publicKey.toBuffer()],
      program.programId
    )

    stakeInfoAccount = anchor.web3.PublicKey.createProgramAddressSync(
      [Buffer.from("STAKE"), provider.publicKey.toBuffer(), mint.toBuffer()],
      program.programId
    )

  })

  it("Stake", async () => {
   
  });

  it("Redeem", async () => {
   
  });

  it("Unstake", async () => {
   
  });
});

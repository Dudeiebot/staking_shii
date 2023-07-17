import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakingShii } from "../target/types/staking_shii";
import * as token from "@solana/spl-token";

describe("staking_shii", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StakingShii as Program<StakingShii>;
  const provider = anchor.getProvider();
  const wallet = anchor.workspace.StakingShii.provider.wallet as anchor.web3.Keypair;

  let mintAccount: anchor.web3.PublicKey
  let userTokenAccount: anchor.web3.PublicKey


  let stakeInfoAccount: anchor.web3.PublicKey
  let userInfoAccount: anchor.web3.PublicKey
  let stakeInfoTokenAccount: anchor.web3.PublicKey


  before(async () => {

    mintAccount: await token.CreateMint( //We created a mint account to mint and check for)
      provider.connection,
      wallet,
      wallet.publicKey,
      wallet.publicKey,
      0
    );
    const tokenAccount = await token.getOrCreatedAssociatedTokenAccount(
      provider.connection,
      wallet,
      mintAccount,
      wallet.publicKey,
    );
    userTokenAccount = tokenAccount.address;
    const mintTx = await token.mintTo(
      provider.connection,
      wallet,
      mintAccount,
      userTokenAccount,
      wallet,
      1
    );

    console.log("minted! ", mintTx)

   
    userInfoAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("USER"), wallet.publicKey.toBuffer()],
      program.programId
    )[0]

    stakeInfoAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("STAKE"), wallet.publicKey.toBuffer(), mintAccount.toBuffer()],
      program.programId
    )[0]

    stakeInfoTokenAccount = token.getAssociatedTokenAddressSync(
      mintAccount,
      stakeInfoAccount,
      true
    );

  })

  it("Stake", async () => {

    const tokenAccountBefore = await token.getAccount(
      provider.connection,
      userTokenAccount,
    );

    console.log("Before staking balance: ", tokenAccountBefore.amount)

    const tx = program.methods.stake().accounts({
      userInfo: userInfoAccount,
      stakeInfo: stakeInfoAccount,
      mint: mintAccount,
      stakeTokenAccount: stakeInfoTokenAccount,
      userTokenAccount: userTokenAccount,
      owner: wallet.publicKey,
      systemProgram:  anchor.web3.SystemProgram.programId,
      tokenProgram: token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: token.ASSOCIATED_TOKEN_PROGRAM,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY
    }).rpc()

    const tokenAccountAfter = await token.getAccount(
      provider.connection,
      userTokenAccount
    );
    const stakeTokenAccountAfter = await token.getAccount(
      provider.connection,
      stakeInfoTokenAccount
    );
    const stakeInfo = await program.account.stakeInfo.fetch(
      stakeInfoAccount
    )
    console.log(stakeInfo);

    console.log("After Staking Balance (user) = ", tokenAccountAfter.amount)
    console.log("After Staking Balance (staking) = ", stakeTokenAccountAfter.amount)
   
  });

  it("Redeem", async () => {
   
  });

  it("Unstake", async () => {
   
  });
});

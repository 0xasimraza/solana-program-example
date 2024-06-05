import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GreeterExample } from "../target/types/greeter_example";

describe("greeter-example", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GreeterExample as Program<GreeterExample>;

  let greetingAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize("Hello World")
      .accounts({
        greetingAccount: greetingAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([greetingAccount])
      .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.greetingAccount.fetch(
      greetingAccount.publicKey
    );
    console.log("account: ", account);
  });

  it("Is set_greeting!", async () => {
    // Add your test here.
    const newGreeting = "New Greeting!";
    await program.methods
      .setGreeting(newGreeting)
      .accounts({
        greetingAccount: greetingAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Fetch the account and check the new greeting.
    const account = await program.account.greetingAccount.fetch(
      greetingAccount.publicKey
    );

    console.log("account: ", account);
    // assert.equal(account.greeting, newGreeting);
  });

  it("Is deposit sol!", async () => {
    // Add your test here.

    const depositAmount = new anchor.BN(10_000_000_000); // 10 SOL
    await program.methods
      .depositSol(depositAmount)
      .accounts({
        greetingAccount: greetingAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch the account and check the new greeting.
    const account = await program.account.greetingAccount.fetch(
      greetingAccount.publicKey
    );

    console.log("account: ", account.balance.toString());
    // assert.equal(account.greeting, newGreeting);
  });

  it("Is withdraw sol!", async () => {
    // Add your test here.

    const withdrawAmount = new anchor.BN(1_000_000_000); // 1 SOL
    await program.methods
      .withdrawSol(withdrawAmount)
      .accounts({
        greetingAccount: greetingAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Fetch the account and check the new greeting.
    const account = await program.account.greetingAccount.fetch(
      greetingAccount.publicKey
    );

    console.log("account: ", account.balance.toString());
    // assert.equal(account.greeting, newGreeting);
  });
});

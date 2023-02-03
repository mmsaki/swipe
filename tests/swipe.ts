import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Swipe } from "../target/types/swipe";

describe("swipe", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Swipe as Program<Swipe>;

  it("Can create a user account!", async () => {
    // Add your test here.
    const [pda, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("user")),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    )
    const tx = await program.methods.createUser("my_username", "my_uri").accounts({
      owner: provider.wallet.publicKey,
      user: pda,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Updates a user account!", async () => { });

  it("Creates a post account!", async () => { });

  it("Updates a post account!", async () => { });

  it("Deletes a post account!", async () => { });

  it("Creates a like account!", async () => { });

  it("Unmatched a liked account!", async () => { });
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day4RequireRevertCustomErrors } from "../target/types/day_4_require_revert_custom_errors";

describe("day_4_require_revert_custom_errors", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day4RequireRevertCustomErrors as Program<Day4RequireRevertCustomErrors>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RoogContract } from "../target/types/roog_contract";

describe("roog-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RoogContract as Program<RoogContract>;
  const secret = require('./min')

  it("Create token mint!", async () => {

  });
});

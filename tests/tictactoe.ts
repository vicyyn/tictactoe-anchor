import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tictactoe } from "../target/types/tictactoe";

describe("tictactoe", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  let game = anchor.web3.Keypair.generate()
  let player_o = anchor.web3.Keypair.generate()

  const program = anchor.workspace.Tictactoe as Program<Tictactoe>;

  it("Initialize Game", async () => {
    // Add your test here.
    const tx = await program.rpc.initializeGame({
	accounts: {
	    payer: program.provider.wallet.publicKey,
	    game: game.publicKey,
	    systemProgram: anchor.web3.SystemProgram.programId,
	},
	instructions: [await program.account.game.createInstruction(game)],
    });
    console.log("Your transaction signature", tx);
  });

  it("Player o joins", async () => {
    // Add your test here.
    const tx = await program.rpc.playerJoins({
	accounts: {
	    player: player_o.publicKey,
	    game: game.publicKey,
	},
    });

    console.log("Your transaction signature", tx);
  });

  it("Player x plays", async () => {
    // Add your test here.
    const tx = await program.rpc.playerMoves(anchor.BN(0),{
	accounts: {
	    player: program.provider.wallet.publicKey,
	    game: game.publicKey,
	},
    });

    console.log("Your transaction signature", tx);
  });
});

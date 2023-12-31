import { PublicKey } from "@solana/web3.js";

export const GLOBAL_STATE_SEED = "GLOBAL_STATE_SEED";
export const VAULT_SEED = "VAULT_SEED";
export const USER_STATE_SEED = "USER_STATE_SEED";

export const TOKEN_ADDRESS = new PublicKey("8N3ZkCwRe36Cj1PqXaMw2h92yzSy18L1z6sptQMiQGrr");

export const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

// todo: for test, it is now one hour
// export const DAY_IN_MS = 3600 * 1000;
export const DAY_IN_MS = 3600 * 24 * 1000;
export const DAY_IN_SECS = 3600 * 24;
export const HOUR_IN_SECS = 3600;
export const TOKEN_DECIMAL = 1000000000;
// minimum amount to deposit
// should mul 10**decimals here to get real minimum
export const DEPOSIT_MINIMUM_AMOUNT = 100;
// tier starts from 0
export const DEFAULT_MAX_TIER = 2;

export const NETWORK = "mainnet-beta";

export const PROGRAM_ID = new PublicKey(
  "FZg4UhJbmeCJCpj3GCx6f1LZeFqt7fUZ2jEFJWx78kwS"
);

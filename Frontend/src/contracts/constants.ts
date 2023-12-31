import { PublicKey } from "@solana/web3.js";

export const GLOBAL_STATE_SEED = "GLOBAL_STATE_SEED";
export const VAULT_SEED = "VAULT_SEED";
export const USER_STATE_SEED = "USER_STATE_SEED";

// todo: for test, it is now one hour
// export const DAY_IN_MS = 3600 * 1000;
export const DAY_IN_MS = 3600 * 24 * 1000;
export const DAY_IN_SECS = 3600 * 24;
export const HOUR_IN_SECS = 3600;
// minimum amount to deposit
// should mul 10**decimals here to get real minimum
export const DEPOSIT_MINIMUM_AMOUNT = 100;
// tier starts from 0
export const DEFAULT_MAX_TIER = 2;

export const NETWORK = "mainnet-beta";

export const PROGRAM_ID = new PublicKey(
  "FZg4UhJbmeCJCpj3GCx6f1LZeFqt7fUZ2jEFJWx78kwS"
);
export const TREASURY = new PublicKey(
  "SRatkWAdtom4oHyvdsHwRPLhKWWABkHQ1tFuJ587HhQ"
);

import { invoke } from "@tauri-apps/api/core";
import { appDataDir, join } from "@tauri-apps/api/path";
import { Stronghold, type Client } from "@tauri-apps/plugin-stronghold";

const CLIENT = "lume";
const VAULT_FILE = "vault.hold";

export type ApiKeyName = "anthropic" | "deepgram" | "voyage";

const STORE_KEYS: Record<ApiKeyName, string> = {
  anthropic: "anthropic_key",
  deepgram: "deepgram_key",
  voyage: "voyage_key",
};

const NAMES = Object.keys(STORE_KEYS) as ApiKeyName[];

let vaultPromise: Promise<{ stronghold: Stronghold; client: Client }> | null = null;

async function openVault(): Promise<{ stronghold: Stronghold; client: Client }> {
  const password = await invoke<string>("vault_password");
  const vaultPath = await join(await appDataDir(), VAULT_FILE);
  const stronghold = await Stronghold.load(vaultPath, password);

  let client: Client;
  try {
    client = await stronghold.loadClient(CLIENT);
  } catch {
    client = await stronghold.createClient(CLIENT);
  }
  return { stronghold, client };
}

/**
 * Open the vault once per session. A single cached promise prevents a second
 * concurrent `Stronghold.load` of the same snapshot (e.g. React's dev
 * double-mount), which otherwise races and reports "no data present"; it also
 * avoids re-prompting the Keychain. The vault is unlocked with the password the
 * Rust side keeps in the macOS Keychain; the file is created on first save.
 */
function getVault(): Promise<{ stronghold: Stronghold; client: Client }> {
  if (!vaultPromise) {
    vaultPromise = openVault().catch((err: unknown) => {
      vaultPromise = null; // allow a later retry after a failure
      throw err;
    });
  }
  return vaultPromise;
}

export async function loadKeys(): Promise<Partial<Record<ApiKeyName, string>>> {
  const { client } = await getVault();
  const store = client.getStore();
  const decoder = new TextDecoder();
  const result: Partial<Record<ApiKeyName, string>> = {};

  for (const name of NAMES) {
    const data = await store.get(STORE_KEYS[name]);
    if (data && data.length > 0) {
      result[name] = decoder.decode(new Uint8Array(data));
    }
  }
  return result;
}

export async function saveKeys(keys: Record<ApiKeyName, string>): Promise<void> {
  const { stronghold, client } = await getVault();
  const store = client.getStore();
  const encoder = new TextEncoder();

  for (const name of NAMES) {
    await store.insert(STORE_KEYS[name], Array.from(encoder.encode(keys[name])));
  }
  await stronghold.save();
}

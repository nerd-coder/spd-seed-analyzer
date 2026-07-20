import init, {
  analyze_seed,
  parse_seed,
  spd_commit,
  spd_version,
} from "@/wasm/spd_wasm";

export type SeedInfo = {
  input: string;
  numeric: number;
  code: string | null;
  formatted: string;
};

export type ItemEntry = {
  name: string;
  category: string;
  source?: string | null;
};

export type FloorReport = {
  depth: number;
  items: ItemEntry[];
  quests: string[];
};

export type SeedReport = {
  seed: SeedInfo;
  spd_version: string;
  spd_commit: string;
  floors_requested: number;
  floors: FloorReport[];
  status: string;
  message?: string | null;
};

let ready: Promise<void> | null = null;

export function ensureWasm(): Promise<void> {
  if (!ready) {
    ready = init().then(() => undefined);
  }
  return ready;
}

export async function parseSeed(input: string): Promise<SeedInfo> {
  await ensureWasm();
  return parse_seed(input) as SeedInfo;
}

export async function analyzeSeed(
  input: string,
  floors: number,
): Promise<SeedReport> {
  await ensureWasm();
  return analyze_seed(input, floors) as SeedReport;
}

export async function getSpdMeta(): Promise<{ version: string; commit: string }> {
  await ensureWasm();
  return { version: spd_version(), commit: spd_commit() };
}

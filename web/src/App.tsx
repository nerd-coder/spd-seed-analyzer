import { useEffect, useState, type FormEvent } from "react";
import { Dices, Loader2, Search } from "lucide-react";

import { FloorMapCanvas } from "@/components/FloorMapCanvas";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import {
  analyzeSeed,
  getSpdMeta,
  type IdentityEntry,
  type SeedReport,
} from "@/lib/spd-wasm";

const ADVANCED_KEY = "spd-analyzer-advanced-mode";

function IdentityTable({
  title,
  entries,
  appearanceLabel,
}: {
  title: string;
  entries: IdentityEntry[];
  appearanceLabel: string;
}) {
  return (
    <div className="space-y-2">
      <h3 className="text-sm font-medium">{title}</h3>
      <div className="overflow-x-auto rounded-md border">
        <table className="w-full text-sm">
          <thead className="bg-muted/50 text-muted-foreground">
            <tr>
              <th className="px-3 py-2 text-left font-medium">Item</th>
              <th className="px-3 py-2 text-left font-medium">
                {appearanceLabel}
              </th>
            </tr>
          </thead>
          <tbody>
            {entries.map((e) => (
              <tr key={e.item} className="border-t">
                <td className="px-3 py-1.5">{e.name}</td>
                <td className="px-3 py-1.5 font-mono text-xs capitalize">
                  {e.appearance.toLowerCase()}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default function App() {
  const [seed, setSeed] = useState("GFX-PZH-DCH");
  const [floors, setFloors] = useState(24);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [report, setReport] = useState<SeedReport | null>(null);
  const [meta, setMeta] = useState<{ version: string; commit: string } | null>(
    null,
  );
  const [advancedMode, setAdvancedMode] = useState(() => {
    try {
      return localStorage.getItem(ADVANCED_KEY) === "1";
    } catch {
      return false;
    }
  });

  useEffect(() => {
    getSpdMeta()
      .then(setMeta)
      .catch((e: unknown) => {
        setError(e instanceof Error ? e.message : String(e));
      });
  }, []);

  function toggleAdvanced(next: boolean) {
    setAdvancedMode(next);
    try {
      localStorage.setItem(ADVANCED_KEY, next ? "1" : "0");
    } catch {
      /* ignore */
    }
  }

  async function onAnalyze(e: FormEvent) {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      const result = await analyzeSeed(seed.trim(), floors);
      setReport(result);
    } catch (err) {
      setReport(null);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="mx-auto flex min-h-svh w-full max-w-3xl flex-col gap-6 px-4 py-10">
      <header className="space-y-2">
        <div className="flex flex-wrap items-center gap-2">
          <Dices className="size-7 text-primary" />
          <h1 className="text-2xl font-semibold tracking-tight">
            SPD Seed Analyzer
          </h1>
          {meta && (
            <Badge variant="secondary" className="font-mono text-xs">
              SPD {meta.version}@{meta.commit}
            </Badge>
          )}
        </div>
        <p className="text-muted-foreground text-sm">
          Enter a Shattered Pixel Dungeon seed to inspect generation data.
          Calculations run in Rust via WebAssembly.
        </p>
      </header>

      <Card>
        <CardHeader>
          <CardTitle>Seed</CardTitle>
          <CardDescription>
            Accepts codes like <code className="font-mono">ABC-DEF-GHI</code>,
            numeric seeds, or free-text fun seeds.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={onAnalyze} className="flex flex-col gap-4">
            <div className="grid gap-2">
              <Label htmlFor="seed">Seed</Label>
              <Input
                id="seed"
                value={seed}
                onChange={(e) => setSeed(e.target.value)}
                placeholder="XXX-XXX-XXX"
                autoComplete="off"
                spellCheck={false}
                className="font-mono uppercase"
              />
            </div>
            <div className="grid max-w-[10rem] gap-2">
              <Label htmlFor="floors">Floors</Label>
              <Input
                id="floors"
                type="number"
                min={1}
                max={26}
                value={floors}
                onChange={(e) => setFloors(Number(e.target.value) || 1)}
              />
            </div>

            <div className="flex items-start justify-between gap-4 rounded-lg border p-3">
              <div className="space-y-1">
                <Label htmlFor="advanced" className="text-sm font-medium">
                  Advanced mode
                </Label>
                <p className="text-muted-foreground text-xs leading-relaxed">
                  Shows full floor maps (spoilers). Can heavily affect how you
                  experience a seeded run — leave off for item lists only.
                </p>
              </div>
              <Switch
                id="advanced"
                checked={advancedMode}
                onCheckedChange={toggleAdvanced}
              />
            </div>

            {advancedMode && (
              <Alert variant="destructive">
                <AlertTitle>Spoiler warning</AlertTitle>
                <AlertDescription>
                  Floor maps reveal layout, entrances, exits, and room shapes
                  before you play. Use only if you want that information.
                </AlertDescription>
              </Alert>
            )}

            <div>
              <Button type="submit" disabled={loading || !seed.trim()}>
                {loading ? (
                  <Loader2 className="animate-spin" />
                ) : (
                  <Search />
                )}
                Analyze
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>

      {error && (
        <Alert variant="destructive">
          <AlertTitle>Error</AlertTitle>
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {report && (
        <>
          <Card>
            <CardHeader>
              <CardTitle className="font-mono">
                {report.seed.code ?? report.seed.formatted}
              </CardTitle>
              <CardDescription className="space-y-1">
                <span className="block">
                  Numeric:{" "}
                  <span className="font-mono text-foreground">
                    {report.seed.numeric}
                  </span>
                </span>
                <span className="block">
                  Status: <Badge variant="outline">{report.status}</Badge>
                </span>
              </CardDescription>
            </CardHeader>
            {report.message && (
              <CardContent>
                <Alert>
                  <AlertTitle>Progress</AlertTitle>
                  <AlertDescription>{report.message}</AlertDescription>
                </Alert>
              </CardContent>
            )}
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Identities</CardTitle>
              <CardDescription>
                Unidentified appearances for this seed (from run init RNG).
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-6">
              <IdentityTable
                title="Potions"
                entries={report.identities.potions}
                appearanceLabel="Color"
              />
              <IdentityTable
                title="Scrolls"
                entries={report.identities.scrolls}
                appearanceLabel="Rune"
              />
              <IdentityTable
                title="Rings"
                entries={report.identities.rings}
                appearanceLabel="Gem"
              />
            </CardContent>
          </Card>

          {report.floors.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle>Floors</CardTitle>
                <CardDescription>
                  Partial levelgen: layout builder + main floor drops.
                  {advancedMode
                    ? " Maps use original region tilesheets when available."
                    : " Enable Advanced mode to view floor maps."}
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-5">
                {report.floors.map((floor) => (
                  <div
                    key={floor.depth}
                    className="space-y-2 border-b pb-4 last:border-0 last:pb-0"
                  >
                    <div className="flex flex-wrap items-center gap-2">
                      <h3 className="font-medium">Floor {floor.depth}</h3>
                      {floor.feeling && floor.feeling !== "none" && (
                        <Badge variant="secondary" className="capitalize">
                          {floor.feeling}
                        </Badge>
                      )}
                      {floor.builder && (
                        <Badge variant="outline" className="font-mono text-xs">
                          {floor.builder}
                        </Badge>
                      )}
                      {floor.map && advancedMode && (
                        <Badge variant="outline" className="font-mono text-xs">
                          {floor.map.width}×{floor.map.height} ·{" "}
                          {floor.map.tileset}
                        </Badge>
                      )}
                    </div>

                    {advancedMode && floor.map && (
                      <div className="overflow-x-auto">
                        <FloorMapCanvas map={floor.map} scale={2} />
                      </div>
                    )}

                    {floor.rooms && floor.rooms.length > 0 && (
                      <div className="space-y-1">
                        <p className="text-muted-foreground text-xs font-medium uppercase tracking-wide">
                          Rooms
                        </p>
                        <p className="text-sm leading-relaxed">
                          {floor.rooms
                            .map((r) => r.replace(/Room$/, ""))
                            .join(" · ")}
                        </p>
                      </div>
                    )}
                    {floor.items.length === 0 ? (
                      <p className="text-muted-foreground text-sm">
                        No items listed.
                      </p>
                    ) : (
                      <ul className="list-disc space-y-0.5 pl-5 text-sm">
                        {floor.items.map((item, i) => (
                          <li key={`${floor.depth}-${i}`}>
                            <span>{item.name}</span>
                            {item.source && (
                              <span className="text-muted-foreground">
                                {" "}
                                ({item.source})
                              </span>
                            )}
                          </li>
                        ))}
                      </ul>
                    )}
                  </div>
                ))}
              </CardContent>
            </Card>
          )}
        </>
      )}
    </div>
  );
}

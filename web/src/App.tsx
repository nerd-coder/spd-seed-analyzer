import { useEffect, useState, type FormEvent } from "react";
import { Dices, Loader2, Search } from "lucide-react";

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
import {
  analyzeSeed,
  getSpdMeta,
  type SeedReport,
} from "@/lib/spd-wasm";

export default function App() {
  const [seed, setSeed] = useState("JLY-ZYR-HET");
  const [floors, setFloors] = useState(24);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [report, setReport] = useState<SeedReport | null>(null);
  const [meta, setMeta] = useState<{ version: string; commit: string } | null>(
    null,
  );

  useEffect(() => {
    getSpdMeta()
      .then(setMeta)
      .catch((e: unknown) => {
        setError(e instanceof Error ? e.message : String(e));
      });
  }, []);

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
                placeholder="JLY-ZYR-HET"
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
          <CardContent className="space-y-4">
            {report.message && (
              <Alert>
                <AlertTitle>Work in progress</AlertTitle>
                <AlertDescription>{report.message}</AlertDescription>
              </Alert>
            )}

            {report.floors.length === 0 ? (
              <p className="text-muted-foreground text-sm">
                No floor items yet. Seed parsing and RNG foundations are in
                place; per-floor generation is next.
              </p>
            ) : (
              report.floors.map((floor) => (
                <div key={floor.depth} className="space-y-2">
                  <h3 className="font-medium">Floor {floor.depth}</h3>
                  <ul className="text-sm list-disc pl-5">
                    {floor.items.map((item, i) => (
                      <li key={`${floor.depth}-${i}`}>{item.name}</li>
                    ))}
                  </ul>
                </div>
              ))
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}

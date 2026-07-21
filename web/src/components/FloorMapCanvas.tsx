import { useEffect, useRef, useState } from "react";

import { drawFloorMap, loadTileset, TILE_PX } from "@/lib/tiles";
import type { FloorMap } from "@/lib/spd-wasm";

type Props = {
  map: FloorMap;
  /** Pixel scale per game tile (1 = 16px, 2 = 32px, …). */
  scale?: number;
  className?: string;
};

export function FloorMapCanvas({ map, scale = 2, className }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setError(null);
    const canvas = canvasRef.current;
    if (!canvas) return;

    const w = map.width;
    const h = map.height;
    canvas.width = w * TILE_PX * scale;
    canvas.height = h * TILE_PX * scale;

    loadTileset(map.tileset)
      .then((img) => {
        if (cancelled) return;
        const ctx = canvas.getContext("2d");
        if (!ctx) return;
        drawFloorMap(ctx, img, w, h, map.tiles, scale);
      })
      .catch((e: unknown) => {
        if (!cancelled) {
          setError(e instanceof Error ? e.message : String(e));
        }
      });

    return () => {
      cancelled = true;
    };
  }, [map, scale]);

  return (
    <div className={className}>
      {error ? (
        <p className="text-destructive text-xs">
          {error} (place tilesheets under{" "}
          <code className="font-mono">public/assets/environment/</code>)
        </p>
      ) : (
        <canvas
          ref={canvasRef}
          className="max-w-full rounded border bg-black/80 image-pixelated"
          style={{ imageRendering: "pixelated" }}
        />
      )}
    </div>
  );
}

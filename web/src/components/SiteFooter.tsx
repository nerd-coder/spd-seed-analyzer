export function SiteFooter() {
  return (
    <footer className="border-border text-muted-foreground shrink-0 border-t px-4 py-3 text-center text-[11px] leading-relaxed">
      <p>
        © {new Date().getFullYear()}{' '}
        <span className="text-foreground/80 font-medium">
          SPD Seed Analyzer
        </span>
        {' · '}
        <a
          href="https://www.gnu.org/licenses/gpl-3.0.html"
          target="_blank"
          rel="noreferrer"
          className="underline-offset-2 hover:text-foreground hover:underline"
        >
          GPL-3.0
        </a>
      </p>
      <p className="mt-0.5">
        Based on{' '}
        <a
          href="https://github.com/00-Evan/shattered-pixel-dungeon"
          target="_blank"
          rel="noreferrer"
          className="underline-offset-2 hover:text-foreground hover:underline"
        >
          Shattered Pixel Dungeon
        </a>{' '}
        by 00-Evan
      </p>
    </footer>
  )
}

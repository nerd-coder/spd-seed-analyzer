import type { ReactNode } from 'react'
import { cn } from '@/lib/utils'

export const SECTIONS = [
  { id: 'color', title: 'Color' },
  { id: 'type', title: 'Type' },
  { id: 'shape', title: 'Shape and depth' },
  { id: 'components', title: 'Components' },
  { id: 'pieces', title: 'Game pieces' },
  { id: 'patterns', title: 'Patterns' },
  { id: 'voice', title: 'Voice' },
] as const

export type SectionId = (typeof SECTIONS)[number]['id']

export function Section({
  id,
  intro,
  children,
}: {
  id: SectionId
  intro: ReactNode
  children: ReactNode
}) {
  const title = SECTIONS.find((section) => section.id === id)?.title
  return (
    <section
      id={id}
      aria-labelledby={`${id}-title`}
      className="flex scroll-mt-20 flex-col gap-10 border-t pt-10"
    >
      <header className="flex max-w-[64ch] flex-col gap-3">
        <h2 id={`${id}-title`} className="font-pixel text-px-2">
          {title}
        </h2>
        <p className="text-lg text-muted-foreground">{intro}</p>
      </header>
      {children}
    </section>
  )
}

export function Subsection({
  title,
  note,
  children,
  className,
}: {
  title: string
  note?: ReactNode
  children: ReactNode
  className?: string
}) {
  return (
    <div className={cn('flex flex-col gap-4', className)}>
      <div className="flex max-w-[64ch] flex-col gap-1">
        <h3 className="font-pixel text-px-1">{title}</h3>
        {note ? <p className="text-muted-foreground">{note}</p> : null}
      </div>
      {children}
    </div>
  )
}

export function SectionNav() {
  return (
    <nav aria-label="Sections" className="hidden lg:block">
      <ul className="sticky top-24 flex flex-col gap-1 text-sm">
        {SECTIONS.map((section) => (
          <li key={section.id}>
            <a
              href={`#${section.id}`}
              className="block rounded-sm px-2 py-1.5 font-semibold text-muted-foreground hover:bg-muted hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
            >
              {section.title}
            </a>
          </li>
        ))}
      </ul>
    </nav>
  )
}

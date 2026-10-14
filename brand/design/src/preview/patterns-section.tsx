import {
  BinocularsIcon,
  InfoIcon,
  MagnifyingGlassIcon,
  PlantIcon,
  PlusIcon,
  XIcon,
} from '@phosphor-icons/react'
import { formatLevel, tierClass } from '@/components/spd/item-slot'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  Field,
  FieldContent,
  FieldDescription,
  FieldLabel,
} from '@/components/ui/field'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
} from '@/components/ui/input-group'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import { Switch } from '@/components/ui/switch'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { regionOf } from '@/lib/regions'
import { cn } from '@/lib/utils'
import { Section, Subsection } from '@/preview/section'

const MATCHES = [
  { seed: 'KRT-MAB-QWE', item: 'Ring of Wealth', level: 2, depth: 6 },
  { seed: 'PLO-ZXV-HUN', item: 'Ring of Wealth', level: 3, depth: 13 },
  { seed: 'DUW-ENC-OAR', item: 'Ring of Wealth', level: 2, depth: 2 },
] as const

function SeedSidebar() {
  return (
    <aside className="flex w-full max-w-80 flex-col gap-5 border bg-sidebar p-4 text-sidebar-foreground">
      <Tabs defaultValue="analyze">
        <TabsList className="w-full">
          <TabsTrigger value="analyze">
            <MagnifyingGlassIcon data-icon="inline-start" />
            Analyze
          </TabsTrigger>
          <TabsTrigger value="find">
            <BinocularsIcon data-icon="inline-start" />
            Find
          </TabsTrigger>
        </TabsList>
      </Tabs>
      <Field>
        <FieldLabel htmlFor="sidebar-seed">Seed</FieldLabel>
        <InputGroup>
          <InputGroupAddon align="inline-start" aria-hidden>
            <PlantIcon />
          </InputGroupAddon>
          <InputGroupInput
            id="sidebar-seed"
            defaultValue="ABC-DEF-GHI"
            className="font-mono"
            spellCheck={false}
          />
        </InputGroup>
        <Button className="w-full">Analyze seed</Button>
        <FieldDescription>
          Up to 10 seeds stay open. The oldest closes first.
        </FieldDescription>
      </Field>
      <Alert variant="info">
        <InfoIcon />
        <AlertTitle>Supports SPD v4.0.2</AlertTitle>
        <AlertDescription>
          Other versions generate other floors.
        </AlertDescription>
      </Alert>
    </aside>
  )
}

function FinderCard() {
  return (
    <Card className="min-w-0 flex-1">
      <CardHeader>
        <CardTitle>Find seeds</CardTitle>
        <CardDescription>
          Every constraint must hold on the same seed.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <div className="flex flex-wrap items-end gap-2">
          <NativeSelect
            aria-label="Item"
            defaultValue="wealth"
            className="min-w-40 flex-1"
          >
            <NativeSelectOption value="wealth">
              Ring of Wealth
            </NativeSelectOption>
            <NativeSelectOption value="frost">Wand of Frost</NativeSelectOption>
          </NativeSelect>
          <NativeSelect aria-label="Upgrade" defaultValue="2">
            <NativeSelectOption value="1">+1 or higher</NativeSelectOption>
            <NativeSelectOption value="2">+2 or higher</NativeSelectOption>
            <NativeSelectOption value="3">+3 or higher</NativeSelectOption>
          </NativeSelect>
          <NativeSelect aria-label="Floors" defaultValue="1-15">
            <NativeSelectOption value="1-5">Floors 1–5</NativeSelectOption>
            <NativeSelectOption value="1-15">Floors 1–15</NativeSelectOption>
          </NativeSelect>
          <Button variant="ghost" size="icon" aria-label="Remove constraint">
            <XIcon />
          </Button>
        </div>
        <Button variant="secondary" size="sm" className="w-fit">
          <PlusIcon data-icon="inline-start" />
          Add constraint
        </Button>
        <Field orientation="horizontal">
          <Switch id="finder-deep" />
          <FieldContent>
            <FieldLabel htmlFor="finder-deep">Deep search</FieldLabel>
            <FieldDescription>
              Also match Imp and Wandmaker rewards. Slower.
            </FieldDescription>
          </FieldContent>
        </Field>
        <table className="w-full text-sm">
          <caption className="sr-only">Matching seeds</caption>
          <thead className="text-left text-muted-foreground">
            <tr className="border-b">
              <th className="py-2 font-semibold">Seed</th>
              <th className="py-2 font-semibold">Match</th>
              <th className="py-2 font-semibold">Floor</th>
            </tr>
          </thead>
          <tbody>
            {MATCHES.map((match) => {
              const region = regionOf(match.depth)
              return (
                <tr key={match.seed} className="border-b last:border-b-0">
                  <td className="py-2 font-mono">{match.seed}</td>
                  <td className="py-2">
                    {match.item}{' '}
                    <span
                      className={cn('font-semibold', tierClass(match.level))}
                    >
                      {formatLevel(match.level)}
                    </span>
                  </td>
                  <td className="py-2">
                    <Badge variant={region.id}>Floor {match.depth}</Badge>
                  </td>
                </tr>
              )
            })}
          </tbody>
        </table>
      </CardContent>
      <CardFooter className="justify-between">
        <span className="text-sm text-muted-foreground tabular-nums">
          3 matches in 100,000 seeds
        </span>
        <Button>
          <BinocularsIcon data-icon="inline-start" />
          Search seeds
        </Button>
      </CardFooter>
    </Card>
  )
}

export function PatternsSection() {
  return (
    <Section
      id="patterns"
      intro="How the parts combine in the analyzer's two modes. The report itself is at the top of the page."
    >
      <Subsection
        title="Analyze and find"
        note="The sidebar holds the seed and the mode. The finder is a window because it stands apart from any one seed; its results name the floor in that floor's region color."
      >
        <div className="flex flex-col items-start gap-6 lg:flex-row">
          <SeedSidebar />
          <FinderCard />
        </div>
      </Subsection>
    </Section>
  )
}

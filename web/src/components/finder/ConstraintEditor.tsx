import { PlusIcon, TrashIcon } from '@phosphor-icons/react'
import { ItemIcon } from '@/components/ItemIcon'
import { Button } from '@/components/ui/button'
import { Field, FieldGroup, FieldLegend, FieldSet } from '@/components/ui/field'
import { InputGroup, InputGroupAddon } from '@/components/ui/input-group'
import {
  NativeSelect,
  NativeSelectOptGroup,
  NativeSelectOption,
} from '@/components/ui/native-select'
import {
  FINDER_ITEM_GROUPS,
  finderItemLabel,
  isFinderItemUpgradeable,
} from './finder-items'
import { type FinderConstraint, MAX_CONSTRAINTS } from './finder-types'

type ConstraintEditorProps = {
  constraints: FinderConstraint[]
  running: boolean
  onAdd: () => void
  onRemove: (id: number) => void
  onUpdate: (id: number, patch: Partial<Omit<FinderConstraint, 'id'>>) => void
}

const UPGRADE_LEVELS = [1, 2, 3, 4] as const

export function ConstraintEditor({
  constraints,
  running,
  onAdd,
  onRemove,
  onUpdate,
}: ConstraintEditorProps) {
  return (
    <FieldSet data-disabled={running ? true : undefined}>
      <FieldLegend variant="label">Item constraints</FieldLegend>
      <FieldGroup className="gap-2">
        {constraints.map((constraint, index) => {
          const itemLabel = finderItemLabel(constraint.className)
          const upgradeable = isFinderItemUpgradeable(constraint.className)
          const isFirst = index === 0
          return (
            <Field
              key={constraint.id}
              data-disabled={running ? true : undefined}
            >
              <div className="flex items-center gap-1">
                <InputGroup className="min-w-0 flex-1">
                  <InputGroupAddon align="inline-start" className="py-0">
                    <ItemIcon
                      classNameItem={constraint.className}
                      size={16}
                      sourceWidth={
                        constraint.className.startsWith('RingOf')
                          ? 8
                          : undefined
                      }
                      sourceHeight={
                        constraint.className.startsWith('RingOf')
                          ? 10
                          : undefined
                      }
                      scaleSource={false}
                      title={itemLabel}
                    />
                  </InputGroupAddon>
                  <NativeSelect
                    value={constraint.className}
                    disabled={running}
                    aria-label={`Item ${index + 1} type`}
                    onChange={(event) => {
                      const className = event.target.value
                      onUpdate(constraint.id, {
                        className,
                        minLevel: isFinderItemUpgradeable(className)
                          ? constraint.minLevel
                          : null,
                      })
                    }}
                    className="min-w-0 flex-1 [&_[data-slot=native-select]]:border-0 [&_[data-slot=native-select]]:bg-transparent [&_[data-slot=native-select]]:focus-visible:ring-0"
                  >
                    {FINDER_ITEM_GROUPS.map((group) => (
                      <NativeSelectOptGroup
                        key={group.label}
                        label={group.label}
                      >
                        {group.items.map((item) => (
                          <NativeSelectOption
                            key={item.className}
                            value={item.className}
                          >
                            {item.label}
                          </NativeSelectOption>
                        ))}
                      </NativeSelectOptGroup>
                    ))}
                  </NativeSelect>
                  {upgradeable ? (
                    <NativeSelect
                      value={
                        constraint.minLevel === null
                          ? 'any'
                          : String(constraint.minLevel)
                      }
                      disabled={running}
                      aria-label={`Item ${index + 1} upgrade level`}
                      onChange={(event) =>
                        onUpdate(constraint.id, {
                          minLevel:
                            event.target.value === 'any'
                              ? null
                              : Number(event.target.value),
                        })
                      }
                      className="w-20 shrink-0 border-l [&_[data-slot=native-select]]:border-0 [&_[data-slot=native-select]]:bg-transparent [&_[data-slot=native-select]]:focus-visible:ring-0"
                    >
                      <NativeSelectOption value="any">Any</NativeSelectOption>
                      {UPGRADE_LEVELS.map((level) => (
                        <NativeSelectOption key={level} value={level}>
                          ≥ +{level}
                        </NativeSelectOption>
                      ))}
                    </NativeSelect>
                  ) : null}
                </InputGroup>
                <Button
                  type="button"
                  size="icon-sm"
                  variant="ghost"
                  className={
                    isFirst
                      ? undefined
                      : 'text-destructive hover:bg-destructive/10 hover:text-destructive'
                  }
                  disabled={
                    running ||
                    (isFirst && constraints.length >= MAX_CONSTRAINTS)
                  }
                  onClick={isFirst ? onAdd : () => onRemove(constraint.id)}
                  aria-label={isFirst ? 'Add item' : `Remove item ${index + 1}`}
                >
                  {isFirst ? <PlusIcon /> : <TrashIcon />}
                </Button>
              </div>
            </Field>
          )
        })}
      </FieldGroup>
    </FieldSet>
  )
}

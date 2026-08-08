import { PlusIcon, TrashIcon } from '@phosphor-icons/react'
import { ItemIcon } from '@/components/ItemIcon'
import { Button } from '@/components/ui/button'
import { Field, FieldGroup, FieldLegend, FieldSet } from '@/components/ui/field'
import { InputGroup } from '@/components/ui/input-group'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import {
  FINDER_GROUP_ORDER,
  type FinderItemGroup,
  fromCoreCategory,
  isFinderItemGroupUpgradeable,
  isFinderItemUpgradeable,
  itemsForGroup,
  toCoreCategory,
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
          const uiGroup = fromCoreCategory(constraint.itemGroup)
          const itemsInGroup = itemsForGroup(uiGroup)

          // Use item level upgradeable logic if an item is selected, otherwise fallback to group logic
          const upgradeable = constraint.className
            ? isFinderItemUpgradeable(constraint.className)
            : isFinderItemGroupUpgradeable(uiGroup)

          return (
            <Field
              key={constraint.id}
              data-disabled={running ? true : undefined}
            >
              <div className="flex items-center gap-1">
                <InputGroup className="min-w-0 flex-1">
                  <div className="relative flex w-10 shrink-0 items-center justify-center border-r py-0">
                    <ItemIcon
                      classNameItem={constraint.className ?? undefined}
                      category={
                        constraint.className
                          ? undefined
                          : toCoreCategory(uiGroup)
                      }
                      size={16}
                      sourceWidth={
                        constraint.className?.startsWith('RingOf')
                          ? 8
                          : undefined
                      }
                      sourceHeight={
                        constraint.className?.startsWith('RingOf')
                          ? 10
                          : undefined
                      }
                      scaleSource={false}
                      title={
                        constraint.className
                          ? itemsInGroup.find(
                              (i) => i.className === constraint.className
                            )?.label
                          : uiGroup
                      }
                    />
                    <NativeSelect
                      value={uiGroup}
                      disabled={running}
                      aria-label={`Item ${index + 1} category`}
                      onChange={(event) => {
                        const newUiGroup = event.target.value as FinderItemGroup
                        const coreCategory = toCoreCategory(newUiGroup)

                        onUpdate(constraint.id, {
                          itemGroup: coreCategory,
                          className: null,
                          minLevel: isFinderItemGroupUpgradeable(newUiGroup)
                            ? constraint.minLevel
                            : null,
                        })
                      }}
                      className="absolute inset-0 h-full w-full opacity-0 cursor-pointer [&_[data-slot=native-select]]:h-full [&_[data-slot=native-select]]:border-0 [&_[data-slot=native-select]]:bg-transparent"
                    >
                      {FINDER_GROUP_ORDER.map((groupLabel) => (
                        <NativeSelectOption key={groupLabel} value={groupLabel}>
                          {groupLabel}
                        </NativeSelectOption>
                      ))}
                    </NativeSelect>
                  </div>

                  <NativeSelect
                    value={constraint.className ?? 'any'}
                    disabled={running}
                    aria-label={`Item ${index + 1} name`}
                    onChange={(event) => {
                      const val = event.target.value
                      const newClassName = val === 'any' ? null : val

                      onUpdate(constraint.id, {
                        className: newClassName,
                        minLevel: (
                          newClassName
                            ? isFinderItemUpgradeable(newClassName)
                            : isFinderItemGroupUpgradeable(uiGroup)
                        )
                          ? constraint.minLevel
                          : null,
                      })
                    }}
                    className="min-w-0 flex-1 [&_[data-slot=native-select]]:border-0 [&_[data-slot=native-select]]:bg-transparent [&_[data-slot=native-select]]:focus-visible:ring-0"
                  >
                    <NativeSelectOption value="any">Any</NativeSelectOption>
                    {itemsInGroup.map((item) => (
                      <NativeSelectOption
                        key={item.className}
                        value={item.className}
                      >
                        {item.label}
                      </NativeSelectOption>
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
                  className="text-destructive hover:bg-destructive/10 hover:text-destructive shrink-0"
                  disabled={running || constraints.length <= 1}
                  onClick={() => onRemove(constraint.id)}
                  aria-label={`Remove item ${index + 1}`}
                >
                  <TrashIcon />
                </Button>
              </div>
            </Field>
          )
        })}

        <div className="pt-2">
          <Button
            type="button"
            variant="outline"
            size="sm"
            disabled={running || constraints.length >= MAX_CONSTRAINTS}
            onClick={onAdd}
            className="w-full flex items-center justify-center gap-1"
          >
            <PlusIcon /> Add item
          </Button>
        </div>
      </FieldGroup>
    </FieldSet>
  )
}

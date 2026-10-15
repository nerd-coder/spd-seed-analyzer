import { PlusIcon, TrashIcon } from '@phosphor-icons/react'
import type { ReactNode } from 'react'
import { ItemBox } from '@/components/ItemBox'
import { Button } from '@/components/ui/button'
import { Field, FieldGroup, FieldLegend, FieldSet } from '@/components/ui/field'
import { InputGroup } from '@/components/ui/input-group'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import { itemClassName, itemIdFromClassName } from '@/generated/game-ids'
import { itemSprite } from '@/lib/item-kind'
import { upgradeLevelClass } from '@/lib/upgrade'
import {
  FINDER_GROUP_ORDER,
  type FinderItemGroup,
  finderUpgradeLevels,
  fromCoreCategory,
  isFinderItemGroupUpgradeable,
  isFinderItemUpgradeable,
  itemsForGroup,
  toCoreCategory,
} from './finder-items'
import {
  type FinderConstraint,
  MAX_CONSTRAINTS,
  MAX_FLOORS,
} from './finder-types'

type ConstraintEditorProps = {
  constraints: FinderConstraint[]
  running: boolean
  onAdd: () => void
  onRemove: (id: number) => void
  onUpdate: (id: number, patch: Partial<Omit<FinderConstraint, 'id'>>) => void
}

const DEPTHS = Array.from({ length: MAX_FLOORS }, (_, index) => index + 1)
const OVERLAY_SELECT_CLASS =
  'absolute inset-0 h-full w-full cursor-pointer opacity-0 [&_[data-slot=native-select]]:h-full [&_[data-slot=native-select]]:border-0 [&_[data-slot=native-select]]:bg-transparent [&_[data-slot=native-select-icon]]:hidden'

type CompactOverlaySelectProps = {
  id?: string
  value: string
  display: ReactNode
  disabled?: boolean
  'aria-label': string
  onChange: (value: string) => void
  children: ReactNode
}

function keptMinLevel(group: FinderItemGroup, minLevel: number | null) {
  if (minLevel === null) return null
  return finderUpgradeLevels(group).includes(minLevel) ? minLevel : null
}

function CompactOverlaySelect({
  id,
  value,
  display,
  disabled,
  'aria-label': ariaLabel,
  onChange,
  children,
}: CompactOverlaySelectProps) {
  return (
    <div className="relative flex w-10 shrink-0 self-stretch items-center justify-center border-l py-0 has-[select:disabled]:opacity-50">
      <span className="pointer-events-none text-xs tabular-nums whitespace-nowrap">
        {display}
      </span>
      <NativeSelect
        id={id}
        value={value}
        disabled={disabled}
        aria-label={ariaLabel}
        onChange={(event) => onChange(event.target.value)}
        className={OVERLAY_SELECT_CLASS}
      >
        {children}
      </NativeSelect>
    </div>
  )
}

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
          const className =
            constraint.item != null ? itemClassName(constraint.item) : null

          // Use item level upgradeable logic if an item is selected, otherwise fallback to group logic
          const upgradeable = className
            ? isFinderItemUpgradeable(className)
            : isFinderItemGroupUpgradeable(uiGroup)

          return (
            <Field
              key={constraint.id}
              data-disabled={running ? true : undefined}
            >
              <div className="flex items-center gap-1">
                <InputGroup className="min-w-0 flex-1">
                  <div className="relative flex w-10 shrink-0 self-stretch items-center justify-center border-r py-0">
                    <ItemBox
                      scale={2}
                      panel={false}
                      border={false}
                      bg={false}
                      sprites={itemSprite({
                        className,
                        category: className
                          ? undefined
                          : toCoreCategory(uiGroup),
                      })}
                      title={
                        className
                          ? itemsInGroup.find((i) => i.className === className)
                              ?.label
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
                          item: null,
                          minLevel: keptMinLevel(
                            newUiGroup,
                            constraint.minLevel
                          ),
                        })
                      }}
                      className={OVERLAY_SELECT_CLASS}
                    >
                      {FINDER_GROUP_ORDER.map((groupLabel) => (
                        <NativeSelectOption key={groupLabel} value={groupLabel}>
                          {groupLabel}
                        </NativeSelectOption>
                      ))}
                    </NativeSelect>
                  </div>

                  <NativeSelect
                    value={className ?? 'any'}
                    disabled={running}
                    aria-label={`Item ${index + 1} name`}
                    onChange={(event) => {
                      const val = event.target.value
                      const newClassName = val === 'any' ? null : val

                      const upgradeableName = newClassName
                        ? isFinderItemUpgradeable(newClassName)
                        : isFinderItemGroupUpgradeable(uiGroup)
                      onUpdate(constraint.id, {
                        item: itemIdFromClassName(newClassName) ?? null,
                        minLevel: upgradeableName
                          ? keptMinLevel(uiGroup, constraint.minLevel)
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
                    <CompactOverlaySelect
                      value={
                        constraint.minLevel === null
                          ? 'any'
                          : String(constraint.minLevel)
                      }
                      display={
                        constraint.minLevel === null ? (
                          'Any'
                        ) : (
                          <span
                            className={upgradeLevelClass(constraint.minLevel)}
                          >
                            +{constraint.minLevel}
                          </span>
                        )
                      }
                      disabled={running}
                      aria-label={`Item ${index + 1} upgrade level`}
                      onChange={(value) =>
                        onUpdate(constraint.id, {
                          minLevel: value === 'any' ? null : Number(value),
                        })
                      }
                    >
                      <NativeSelectOption value="any">Any</NativeSelectOption>
                      {finderUpgradeLevels(uiGroup).map((level) => (
                        <NativeSelectOption key={level} value={level}>
                          ≥ +{level}
                        </NativeSelectOption>
                      ))}
                    </CompactOverlaySelect>
                  ) : null}

                  <CompactOverlaySelect
                    value={String(constraint.maxDepth)}
                    display={`↑${constraint.maxDepth}`}
                    disabled={running}
                    aria-label={`Item ${index + 1} depth`}
                    onChange={(value) =>
                      onUpdate(constraint.id, { maxDepth: Number(value) })
                    }
                  >
                    {DEPTHS.map((depth) => (
                      <NativeSelectOption key={depth} value={depth}>
                        ↑{depth}
                      </NativeSelectOption>
                    ))}
                  </CompactOverlaySelect>
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

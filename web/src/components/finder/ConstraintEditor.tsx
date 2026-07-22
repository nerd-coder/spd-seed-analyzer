import { PlusIcon, TrashIcon } from '@phosphor-icons/react'
import { Fragment } from 'react'
import { ItemIcon } from '@/components/ItemIcon'
import { Button } from '@/components/ui/button'
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSeparator,
  FieldSet,
  FieldTitle,
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import {
  NativeSelect,
  NativeSelectOptGroup,
  NativeSelectOption,
} from '@/components/ui/native-select'
import { FINDER_ITEM_GROUPS } from './finder-items'
import {
  type FinderConstraint,
  isIntegerInRange,
  MAX_CONSTRAINTS,
} from './finder-types'

type ConstraintEditorProps = {
  constraints: FinderConstraint[]
  floors: number
  running: boolean
  attempted: boolean
  onAdd: () => void
  onRemove: (id: number) => void
  onUpdate: (id: number, patch: Partial<Omit<FinderConstraint, 'id'>>) => void
}

export function ConstraintEditor({
  constraints,
  floors,
  running,
  attempted,
  onAdd,
  onRemove,
  onUpdate,
}: ConstraintEditorProps) {
  return (
    <FieldSet data-disabled={running ? true : undefined}>
      <FieldLegend variant="label">Item constraints</FieldLegend>
      <FieldDescription>
        Choose exact item types; internal game class names stay hidden.
      </FieldDescription>
      <FieldGroup className="gap-4">
        {constraints.map((constraint, index) => {
          const minInvalid = !isIntegerInRange(constraint.minDepth, 1, floors)
          const maxInvalid =
            minInvalid ||
            !isIntegerInRange(
              constraint.maxDepth,
              Number(constraint.minDepth),
              floors
            )
          return (
            <Fragment key={constraint.id}>
              {index > 0 ? <FieldSeparator /> : null}
              <FieldGroup className="gap-2">
                <Field orientation="horizontal">
                  <FieldTitle>Item {index + 1}</FieldTitle>
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon-sm"
                    disabled={running || constraints.length === 1}
                    onClick={() => onRemove(constraint.id)}
                    aria-label={`Remove item ${index + 1}`}
                    className="ml-auto"
                  >
                    <TrashIcon />
                  </Button>
                </Field>
                <div className="grid gap-2 sm:grid-cols-[minmax(0,1fr)_6rem_6rem]">
                  <Field data-disabled={running ? true : undefined}>
                    <FieldLabel htmlFor={`finder-item-${constraint.id}`}>
                      Item type
                    </FieldLabel>
                    <div className="flex items-center gap-2">
                      <ItemIcon
                        classNameItem={constraint.className}
                        size={24}
                      />
                      <NativeSelect
                        id={`finder-item-${constraint.id}`}
                        value={constraint.className}
                        disabled={running}
                        onChange={(event) =>
                          onUpdate(constraint.id, {
                            className: event.target.value,
                          })
                        }
                        className="min-w-0 flex-1"
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
                    </div>
                  </Field>
                  <Field
                    data-invalid={attempted && minInvalid ? true : undefined}
                    data-disabled={running ? true : undefined}
                  >
                    <FieldLabel htmlFor={`finder-min-${constraint.id}`}>
                      Min floor
                    </FieldLabel>
                    <Input
                      id={`finder-min-${constraint.id}`}
                      type="number"
                      min={1}
                      max={floors}
                      step={1}
                      value={constraint.minDepth}
                      disabled={running}
                      aria-invalid={attempted && minInvalid}
                      onChange={(event) =>
                        onUpdate(constraint.id, {
                          minDepth:
                            event.currentTarget.value === ''
                              ? ''
                              : event.currentTarget.valueAsNumber,
                        })
                      }
                    />
                  </Field>
                  <Field
                    data-invalid={attempted && maxInvalid ? true : undefined}
                    data-disabled={running ? true : undefined}
                  >
                    <FieldLabel htmlFor={`finder-max-${constraint.id}`}>
                      Max floor
                    </FieldLabel>
                    <Input
                      id={`finder-max-${constraint.id}`}
                      type="number"
                      min={constraint.minDepth === '' ? 1 : constraint.minDepth}
                      max={floors}
                      step={1}
                      value={constraint.maxDepth}
                      disabled={running}
                      aria-invalid={attempted && maxInvalid}
                      onChange={(event) =>
                        onUpdate(constraint.id, {
                          maxDepth:
                            event.currentTarget.value === ''
                              ? ''
                              : event.currentTarget.valueAsNumber,
                        })
                      }
                    />
                  </Field>
                </div>
                {attempted && (minInvalid || maxInvalid) ? (
                  <FieldError>
                    Use an inclusive range between floor 1 and {floors}.
                  </FieldError>
                ) : null}
              </FieldGroup>
            </Fragment>
          )
        })}
      </FieldGroup>
      <Field orientation="horizontal">
        <Button
          type="button"
          variant="outline"
          disabled={running || constraints.length >= MAX_CONSTRAINTS}
          onClick={onAdd}
        >
          <PlusIcon data-icon="inline-start" />
          Add item
        </Button>
      </Field>
    </FieldSet>
  )
}

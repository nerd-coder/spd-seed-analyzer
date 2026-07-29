import { ItemIcon } from '@/components/ItemIcon'
import { Badge } from '@/components/ui/badge'
import {
  Combobox,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxInput,
  ComboboxItem,
  ComboboxList,
} from '@/components/ui/combobox'
import { InputGroupAddon } from '@/components/ui/input-group'
import {
  Item,
  ItemContent,
  ItemDescription,
  ItemMedia,
  ItemTitle,
} from '@/components/ui/item'
import type { TrinketKind, TrinketSelectionReport } from '@/lib/spd-wasm'
import {
  TRINKET_OPTIONS,
  type TrinketOption,
  trinketAcquisitionLabel,
  trinketOption,
} from './trinket-options'

export function TrinketCombobox({
  id,
  value,
  selection,
  disabled,
  label,
  onChange,
}: {
  id: string
  value: TrinketKind
  selection: TrinketSelectionReport
  disabled: boolean
  label: string
  onChange: (value: TrinketKind) => void
}) {
  const selected = trinketOption(value)

  return (
    <Combobox
      items={TRINKET_OPTIONS}
      value={selected}
      disabled={disabled}
      itemToStringValue={(option: TrinketOption) => option.label}
      onValueChange={(option: TrinketOption | null) => {
        if (option) onChange(option.value)
      }}
    >
      <ComboboxInput
        id={id}
        aria-label={label}
        placeholder="Search trinkets"
        disabled={disabled}
        className="min-w-0 flex-1"
      >
        <InputGroupAddon align="inline-start">
          <ItemIcon
            classNameItem={selected.className}
            category="trinket"
            size={18}
            title={selected.label}
          />
        </InputGroupAddon>
      </ComboboxInput>
      <ComboboxContent className="sm:w-96">
        <ComboboxEmpty>No trinket found.</ComboboxEmpty>
        <ComboboxList>
          {(option: TrinketOption) => (
            <ComboboxItem key={option.value} value={option}>
              <Item size="xs" className="p-0">
                <ItemMedia>
                  <ItemIcon
                    classNameItem={option.className}
                    category="trinket"
                    size={20}
                    title={option.label}
                  />
                </ItemMedia>
                <ItemContent>
                  <ItemTitle>
                    {option.label}
                    <Badge variant="outline">
                      {trinketAcquisitionLabel(option, selection)}
                    </Badge>
                  </ItemTitle>
                  <ItemDescription>{option.description}</ItemDescription>
                </ItemContent>
              </Item>
            </ComboboxItem>
          )}
        </ComboboxList>
      </ComboboxContent>
    </Combobox>
  )
}

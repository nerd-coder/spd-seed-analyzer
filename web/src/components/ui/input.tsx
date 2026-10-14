import type * as React from 'react'

import { cn } from '@/lib/utils'

/* Inputs are recessed: page-colored well, shadow on the top edge. */
function Input({ className, type, ...props }: React.ComponentProps<'input'>) {
  return (
    <input
      type={type}
      data-slot="input"
      className={cn(
        'h-9 w-full min-w-0 rounded-sm border border-input bg-background px-3 py-1 text-sm inset-shadow-recessed transition-colors outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-semibold file:text-foreground placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/40 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-2 aria-invalid:ring-destructive/25',
        className
      )}
      {...props}
    />
  )
}

export { Input }

import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { WorkspaceEmptyPlaceholder } from '@/components/WorkspaceEmptyPlaceholder'

export function EmptyAnalysisPlaceholder() {
  return (
    <WorkspaceEmptyPlaceholder
      icon={MagnifyingGlassIcon}
      title="No seeds analyzed yet"
      description="Enter a seed in the sidebar to start an analysis."
    />
  )
}

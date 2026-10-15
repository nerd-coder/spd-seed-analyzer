import { useEffect } from 'react'
import { firstImageFile } from '@/lib/seed-ocr/read-image'
import {
  dropScreenshot,
  hideScreenshotDropTarget,
  scanScreenshot,
  showScreenshotDropTarget,
} from '@/stores/app'

/**
 * `dragover` repeats every few hundred ms while a drag is over the page, so
 * this long without one means the drag left even if a `dragleave` was lost.
 */
const DRAG_WATCHDOG_MS = 1000

const carriesFiles = (event: DragEvent) =>
  event.dataTransfer?.types.includes('Files') ?? false

/** Accept screenshots pasted or dragged anywhere on the page while `active`. */
export function useScreenshotPasteDrop(active: boolean) {
  useEffect(() => {
    if (!active) return
    // Every element entered fires `dragenter` before the one left fires `dragleave`.
    let depth = 0
    let watchdog: ReturnType<typeof setTimeout> | undefined

    function endDrag() {
      depth = 0
      clearTimeout(watchdog)
      hideScreenshotDropTarget()
    }
    function armWatchdog() {
      clearTimeout(watchdog)
      watchdog = setTimeout(endDrag, DRAG_WATCHDOG_MS)
    }

    function onDragEnter(event: DragEvent) {
      if (!carriesFiles(event)) return
      depth++
      showScreenshotDropTarget()
      armWatchdog()
    }
    function onDragOver(event: DragEvent) {
      if (!carriesFiles(event) || !event.dataTransfer) return
      event.preventDefault()
      event.dataTransfer.dropEffect = 'copy'
      armWatchdog()
    }
    function onDragLeave(event: DragEvent) {
      if (!carriesFiles(event)) return
      depth = Math.max(0, depth - 1)
      if (depth === 0) endDrag()
    }
    function onDrop(event: DragEvent) {
      if (!carriesFiles(event) || !event.dataTransfer) return
      event.preventDefault()
      depth = 0
      clearTimeout(watchdog)
      void dropScreenshot(event.dataTransfer.files)
    }
    function onPaste(event: ClipboardEvent) {
      const image = firstImageFile(event.clipboardData?.files)
      if (!image) return
      event.preventDefault()
      void scanScreenshot(image)
    }

    window.addEventListener('dragenter', onDragEnter)
    window.addEventListener('dragover', onDragOver)
    window.addEventListener('dragleave', onDragLeave)
    window.addEventListener('drop', onDrop)
    window.addEventListener('paste', onPaste)
    return () => {
      window.removeEventListener('dragenter', onDragEnter)
      window.removeEventListener('dragover', onDragOver)
      window.removeEventListener('dragleave', onDragLeave)
      window.removeEventListener('drop', onDrop)
      window.removeEventListener('paste', onPaste)
      clearTimeout(watchdog)
      hideScreenshotDropTarget()
    }
  }, [active])
}

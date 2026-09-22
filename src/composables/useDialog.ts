import { ref, watch, onBeforeUnmount, Ref, nextTick } from 'vue'

export function useDialog(
  isOpen: Readonly<Ref<boolean>>,
  onChange?: (v: boolean) => void
) {
  const dialog = ref<HTMLDialogElement | null>(null)

  const sync = async (open: boolean) => {
    await nextTick()
    if (!dialog.value) return
    if (open && !dialog.value.open) dialog.value.showModal()
    if (!open && dialog.value.open) dialog.value.close()
  }

  const onClose = () => onChange?.(false)

  watch(isOpen, sync, { immediate: true })
  onBeforeUnmount(() => dialog.value?.open && dialog.value.close())

  return { dialog, onClose }
}

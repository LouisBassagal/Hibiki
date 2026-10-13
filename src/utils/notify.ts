import { toast } from 'vue-sonner'

const bringToFront = () => {
  if (!document.querySelector('dialog[open]')) return
  const el = document.querySelector<HTMLElement>('.toast-layer')
  el?.hidePopover()
  el?.showPopover()
}

export const notify = {
  success: (message: string) => {
    bringToFront()
    toast.success(message)
  },
  error: (message: string) => {
    bringToFront()
    toast.error(message)
  },
  info: (message: string) => {
    bringToFront()
    toast.info(message)
  },
}

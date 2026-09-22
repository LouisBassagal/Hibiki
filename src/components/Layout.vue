<template>
  <main
    class="relative w-full h-full grow overflow-x-hidden flex flex-col bg-background pt-safe-t pb-safe-b"
  >
    <Transition name="reveal" mode="out-in">
      <slot />
    </Transition>
    <FloatingMenu />
  </main>
</template>

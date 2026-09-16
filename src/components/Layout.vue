<template>
  <main
    class="relative w-full h-full grow overflow-x-hidden flex flex-col bg-background"
  >
    <div class="h-10 shrink-0"></div>
    <Transition name="reveal" mode="in-out">
      <slot />
    </Transition>
    <FloatingMenu />
  </main>
</template>

<script setup>
import { Handle, Position } from '@vue-flow/core'

const props = defineProps({
  data: {
    type: Object,
    required: true,
  },
})

const palette = {
  start: { color: 'success', icon: 'tabler-player-play' },
  task: { color: 'primary', icon: 'tabler-cpu' },
  approval: { color: 'warning', icon: 'tabler-user-check' },
  end: { color: 'error', icon: 'tabler-flag' },
}

const style = computed(() => palette[props.data.nodeType] ?? palette.task)
const subtitle = computed(() => props.data.nodeType === 'task' ? (props.data.executor ?? 'noop') : '')
</script>

<template>
  <VCard
    :color="style.color"
    variant="tonal"
    min-width="170"
    class="workflow-canvas-node"
  >
    <VCardText class="d-flex align-center gap-2 pa-2">
      <VIcon
        :icon="style.icon"
        size="18"
      />
      <div>
        <div class="text-body-2 font-weight-medium">
          {{ data.name || '(unnamed)' }}
        </div>
        <div
          v-if="subtitle"
          class="text-caption text-medium-emphasis text-capitalize"
        >
          {{ subtitle }}
        </div>
      </div>
    </VCardText>
    <Handle
      v-if="data.nodeType !== 'start'"
      type="target"
      :position="Position.Left"
    />
    <Handle
      v-if="data.nodeType !== 'end'"
      type="source"
      :position="Position.Right"
    />
  </VCard>
</template>

<style scoped>
.workflow-canvas-node {
  cursor: grab;
}
</style>

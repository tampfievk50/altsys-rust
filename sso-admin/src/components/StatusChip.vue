<script setup>
const props = defineProps({
  status: {
    type: [String, Boolean],
    required: true,
  },
})

// Covers every status vocabulary used across the 8 services: booleans
// (is_active/is_enabled) and the Running/Succeeded/Failed-style strings used
// by workflow executions, sdlc step executions and automation rule firings.
const colorMap = {
  true: 'success',
  false: 'secondary',
  active: 'success',
  inactive: 'secondary',
  enabled: 'success',
  disabled: 'secondary',
  running: 'info',
  pending: 'warning',
  succeeded: 'success',
  completed: 'success',
  failed: 'error',
  skipped: 'secondary',
  cancelled: 'secondary',
}

const label = computed(() => {
  if (typeof props.status === 'boolean')
    return props.status ? 'Active' : 'Inactive'

  return props.status
})

const color = computed(() => {
  const key = String(props.status).toLowerCase()

  return colorMap[key] ?? 'primary'
})
</script>

<template>
  <VChip
    :color="color"
    size="small"
    label
    class="text-capitalize"
  >
    {{ label }}
  </VChip>
</template>

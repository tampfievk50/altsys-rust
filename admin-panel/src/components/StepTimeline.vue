<script setup>
/**
 * Renders a list of timestamped status events as a vertical timeline.
 * Used for Workflow node executions, SDLC step executions, and Automation
 * rule firings — each caller maps its own API shape into this normalized
 * `{ title, status, attempt, output, error, timestamp }` shape first.
 */
defineProps({
  items: {
    type: Array,
    required: true,
  },
})

const dotColor = status => {
  const key = String(status).toLowerCase()
  if (['succeeded', 'completed', 'success'].includes(key))
    return 'success'
  if (key === 'failed')
    return 'error'
  if (['running', 'pending'].includes(key))
    return 'info'

  return 'secondary'
}

// `output` is usually a JSON string (tool results, agent output wrappers,
// workflow context) — pretty-print it so nested fields are actually scannable
// instead of one dense line. Falls back to the raw value for plain-text output.
const formatOutput = raw => {
  if (typeof raw !== 'string')
    return raw

  try {
    return JSON.stringify(JSON.parse(raw), null, 2)
  }
  catch {
    return raw
  }
}
</script>

<template>
  <VTimeline
    density="compact"
    align="start"
    truncate-line="both"
  >
    <VTimelineItem
      v-for="(item, index) in items"
      :key="index"
      :dot-color="dotColor(item.status)"
      size="small"
    >
      <div class="d-flex justify-space-between align-center flex-wrap gap-2 mb-1">
        <h6 class="text-h6">
          {{ item.title }}
          <span
            v-if="item.attempt > 1"
            class="text-body-2 text-medium-emphasis"
          >(attempt {{ item.attempt }})</span>
        </h6>
        <div class="d-flex align-center gap-2">
          <StatusChip :status="item.status" />
          <span
            v-if="item.timestamp"
            class="text-caption text-medium-emphasis"
          >{{ new Date(item.timestamp).toLocaleString() }}</span>
        </div>
      </div>
      <pre
        v-if="item.output"
        class="text-body-2 bg-var-theme-background pa-2 rounded"
        style="white-space: pre-wrap; word-break: break-word;"
      >{{ formatOutput(item.output) }}</pre>
      <VAlert
        v-if="item.error"
        type="error"
        variant="tonal"
        density="compact"
        class="mt-1"
      >
        {{ item.error }}
      </VAlert>
    </VTimelineItem>
  </VTimeline>
</template>

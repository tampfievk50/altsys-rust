<script setup>
/**
 * A monospace textarea for the JSON-config fields scattered across the
 * backends (Tool.config, WorkflowDefinition graphs, AutomationRule.action,
 * SdlcRun context, ...). Validates on blur so a malformed edit is caught
 * before submit rather than rejected opaquely by the server.
 */
const props = defineProps({
  modelValue: {
    type: String,
    default: '',
  },
  label: {
    type: String,
    default: 'JSON',
  },
  rows: {
    type: [Number, String],
    default: 6,
  },
  required: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['update:modelValue'])

const parseError = ref('')

const validate = () => {
  const raw = props.modelValue?.trim()
  if (!raw) {
    parseError.value = props.required ? 'JSON is required' : ''

    return
  }
  try {
    JSON.parse(raw)
    parseError.value = ''
  }
  catch (err) {
    parseError.value = `Invalid JSON: ${err.message}`
  }
}

defineExpose({ validate, isValid: () => { validate(); return !parseError.value } })
</script>

<template>
  <VTextarea
    :model-value="modelValue"
    :label="label"
    :rows="rows"
    class="font-monospace"
    :error-messages="parseError"
    auto-grow
    @update:model-value="emit('update:modelValue', $event)"
    @blur="validate"
  />
</template>

<style scoped>
.font-monospace :deep(textarea) {
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  font-size: 0.8125rem;
}
</style>

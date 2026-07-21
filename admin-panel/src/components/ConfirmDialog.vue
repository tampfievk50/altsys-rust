<script setup>
const props = defineProps({
  modelValue: {
    type: Boolean,
    required: true,
  },
  title: {
    type: String,
    default: 'Are you sure?',
  },
  text: {
    type: String,
    default: 'This action cannot be undone.',
  },
  confirmColor: {
    type: String,
    default: 'error',
  },
  confirmLabel: {
    type: String,
    default: 'Delete',
  },
})

const emit = defineEmits(['update:modelValue', 'confirm'])
</script>

<template>
  <VDialog
    :model-value="modelValue"
    max-width="420"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <VCard :title="title">
      <VCardText>{{ text }}</VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="emit('update:modelValue', false)"
        >
          Cancel
        </VBtn>
        <VBtn
          :color="confirmColor"
          @click="emit('confirm'); emit('update:modelValue', false)"
        >
          {{ confirmLabel }}
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>
</template>

<script setup>
const props = defineProps({
  modelValue: {
    type: [String, Array, null],
    default: null,
  },
  fetchOptions: {
    type: Function,
    required: true,
  },
  itemTitle: {
    type: [String, Function],
    default: 'name',
  },
  itemValue: {
    type: String,
    default: 'id',
  },
  label: {
    type: String,
    default: '',
  },
  multiple: {
    type: Boolean,
    default: false,
  },
  rules: {
    type: Array,
    default: () => [],
  },
  clearable: {
    type: Boolean,
    default: true,
  },

  // Shown in the dropdown when the fetch succeeds but returns zero options,
  // so an empty list reads as "nothing created yet" instead of "broken".
  emptyText: {
    type: String,
    default: 'No options available.',
  },
})

const emit = defineEmits(['update:modelValue'])

// An autocomplete select whose options come from another service's list
// endpoint, for forms that reference another resource by id (e.g. an Agent
// form picking a Platform Model, or an SDLC run wizard picking Tools).
// `fetchOptions` is called once on mount; pass a fresh function (not a
// cached array) if the caller needs it re-fetched per open.
const options = ref([])
const loading = ref(false)
const loadError = ref('')

const load = async () => {
  loading.value = true
  loadError.value = ''
  try {
    options.value = await props.fetchOptions()
  }
  catch (err) {
    loadError.value = err.message || 'Failed to load options'
  }
  finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <VAutocomplete
    :model-value="modelValue"
    :items="options"
    :item-title="itemTitle"
    :item-value="itemValue"
    :label="label"
    :multiple="multiple"
    :rules="rules"
    :clearable="clearable"
    :loading="loading"
    :error-messages="loadError"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <template #no-data>
      <p class="text-body-2 text-medium-emphasis px-4 py-2 mb-0">
        {{ loading ? 'Loading…' : emptyText }}
      </p>
    </template>
  </VAutocomplete>
</template>

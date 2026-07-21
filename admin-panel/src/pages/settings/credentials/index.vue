<script setup>
definePage({
  meta: { navActiveLink: 'settings-credentials' },
})

const userData = useCookie('userData')
const tenants = ref([])
const selectedTenant = ref(userData.value?.tenantId ?? null)

const loadTenants = async () => {
  tenants.value = await ssoApi('/api/v1/tenants')
  if (!selectedTenant.value && tenants.value.length)
    selectedTenant.value = tenants.value[0].id
}

// Per-provider shape of what's needed to connect. `secretLabel` names
// whatever goes in the one always-present `secret` column the backend
// stores; `fields` are provider-specific extras that get folded into the
// opaque `metadata` JSON string alongside anything typed into the advanced
// metadata box (for keys not modeled here yet).
const providerConfig = {
  anthropic: {
    label: 'Anthropic',
    secretLabel: 'API Key',
    secretPlaceholder: 'sk-ant-...',
    fields: [],
  },
  openai: {
    label: 'OpenAI',
    secretLabel: 'API Key',
    secretPlaceholder: 'sk-...',
    fields: [
      { key: 'organization_id', label: 'Organization ID (optional)' },
    ],
  },
  gemini: {
    label: 'Gemini',
    secretLabel: 'API Key',
    secretPlaceholder: 'AIza...',
    fields: [
      { key: 'project_id', label: 'Project ID (optional)' },
    ],
  },
  'azure_openai': {
    label: 'Azure OpenAI',
    secretLabel: 'API Key',
    fields: [
      { key: 'endpoint', label: 'Endpoint URL', placeholder: 'https://your-resource.openai.azure.com', required: true },
      { key: 'deployment', label: 'Deployment Name', required: true },
      { key: 'api_version', label: 'API Version (optional)', placeholder: '2024-02-01' },
    ],
  },
  mistral: {
    label: 'Mistral',
    secretLabel: 'API Key',
    fields: [],
  },
  cohere: {
    label: 'Cohere',
    secretLabel: 'API Key',
    fields: [],
  },
  ollama: {
    label: 'Ollama',
    secretLabel: 'API Key / Token (any value if your instance has none)',
    fields: [
      { key: 'base_url', label: 'Base URL', placeholder: 'http://localhost:11434', required: true },
    ],
  },
  github: {
    label: 'GitHub',
    secretLabel: 'Personal Access Token',
    secretPlaceholder: 'ghp_...',
    fields: [],
  },
  jira: {
    label: 'Jira',
    secretLabel: 'API Token',
    fields: [
      { key: 'email', label: 'Account Email', type: 'email', required: true },
    ],
  },

  // Covers anything self-hosted or not yet in this list (e.g. an
  // OpenAI-compatible gateway).
  custom: {
    label: 'Custom',
    secretLabel: 'Secret',
    fields: [
      { key: 'base_url', label: 'Base URL (optional)' },
    ],
  },
}

const providerItems = Object.entries(providerConfig).map(([value, cfg]) => ({ value, title: cfg.label }))

const crud = useCrud({
  list: () => selectedTenant.value ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/credentials`) : Promise.resolve([]),
  create: payload => sdlcPlatformApi('/api/v1/credentials', { method: 'POST', body: payload }),
  update: (id, payload) => sdlcPlatformApi(`/api/v1/credentials/${id}`, { method: 'PUT', body: payload }),
  remove: id => sdlcPlatformApi(`/api/v1/credentials/${id}`, { method: 'DELETE' }),
})

watch(selectedTenant, crud.fetchAll)

const headers = [
  { title: 'Name', key: 'name' },
  { title: 'Provider', key: 'provider' },
  { title: 'Secret', key: 'secret_hint' },
  { title: 'Status', key: 'is_active' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()
const refAdvancedMetadata = ref()

const buildEmptyExtra = provider => Object.fromEntries((providerConfig[provider]?.fields ?? []).map(f => [f.key, '']))

const emptyForm = () => ({
  name: '',
  provider: 'anthropic',
  secret: '',
  extra: buildEmptyExtra('anthropic'),
  advancedMetadata: '',
  isActive: true,
})

const form = ref(emptyForm())

const currentProviderConfig = computed(() => providerConfig[form.value.provider] ?? providerConfig.custom)
const currentProviderFields = computed(() => currentProviderConfig.value.fields)

// Reset the structured extras whenever the provider changes while creating
// (the provider select is disabled once editing, so this never fires then).
watch(() => form.value.provider, provider => {
  if (!editingId.value)
    form.value.extra = buildEmptyExtra(provider)
})

const openCreate = () => {
  editingId.value = null
  form.value = emptyForm()
  formError.value = ''
  dialogOpen.value = true
}

// Splits a credential's stored metadata into the structured fields this
// provider knows about vs. anything left over, so editing a credential
// doesn't silently drop keys the form doesn't have an input for.
const splitMetadata = (provider, metadataRaw) => {
  let parsed = {}

  try {
    parsed = JSON.parse(metadataRaw ?? '{}')
  }
  catch {
    parsed = {}
  }

  const fieldKeys = new Set((providerConfig[provider]?.fields ?? []).map(f => f.key))
  const extra = buildEmptyExtra(provider)
  const rest = {}

  for (const [key, value] of Object.entries(parsed)) {
    if (fieldKeys.has(key))
      extra[key] = value
    else
      rest[key] = value
  }

  return { extra, rest }
}

const openEdit = credential => {
  editingId.value = credential.id

  const { extra, rest } = splitMetadata(credential.provider, credential.metadata)

  form.value = {
    name: credential.name,
    provider: credential.provider,
    secret: '',
    extra,
    advancedMetadata: Object.keys(rest).length ? JSON.stringify(rest, null, 2) : '',
    isActive: credential.is_active,
  }
  formError.value = ''
  dialogOpen.value = true
}

const buildMetadata = () => {
  let advanced = {}

  if (form.value.advancedMetadata?.trim()) {
    try {
      advanced = JSON.parse(form.value.advancedMetadata)
    }
    catch {
      advanced = {}
    }
  }

  const structured = Object.fromEntries(Object.entries(form.value.extra).filter(([, v]) => v !== '' && v != null))
  const merged = { ...advanced, ...structured }

  return Object.keys(merged).length ? JSON.stringify(merged) : null
}

const submit = async () => {
  const { valid } = await refForm.value.validate()
  if (!valid || !(refAdvancedMetadata.value?.isValid() ?? true))
    return

  formError.value = ''
  try {
    if (editingId.value) {
      await crud.update(editingId.value, {
        name: form.value.name,
        secret: form.value.secret || null,
        metadata: buildMetadata(),
        is_active: form.value.isActive,
      })
    }
    else {
      await crud.create({
        tenant_id: selectedTenant.value,
        name: form.value.name,
        provider: form.value.provider,
        secret: form.value.secret,
        metadata: buildMetadata(),
      })
    }

    dialogOpen.value = false
  }
  catch (err) {
    formError.value = err.message || 'Save failed'
  }
}

const confirmOpen = ref(false)
const pendingDeleteId = ref(null)

const askDelete = id => {
  pendingDeleteId.value = id
  confirmOpen.value = true
}

const doDelete = () => crud.remove(pendingDeleteId.value)

// ===== Reveal secret =====
const revealDialogOpen = ref(false)
const revealCredential = ref(null)
const revealSecret = ref('')
const revealError = ref('')
const revealLoading = ref(false)

const openReveal = async credential => {
  revealCredential.value = credential
  revealSecret.value = ''
  revealError.value = ''
  revealDialogOpen.value = true
  revealLoading.value = true
  try {
    const result = await sdlcPlatformApi(`/api/v1/credentials/${credential.id}/secret`)

    revealSecret.value = result.secret
  }
  catch (err) {
    revealError.value = err.message || 'Failed to reveal secret'
  }
  finally {
    revealLoading.value = false
  }
}

const copyRevealSecret = () => navigator.clipboard?.writeText(revealSecret.value)

onMounted(async () => {
  await loadTenants()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Provider Credentials">
    <template #append>
      <div class="d-flex align-center gap-4">
        <VSelect
          v-model="selectedTenant"
          :items="tenants"
          item-title="name"
          item-value="id"
          label="Tenant"
          density="compact"
          style="min-inline-size: 220px;"
          hide-details
        />
        <VBtn
          prepend-icon="tabler-plus"
          :disabled="!selectedTenant"
          @click="openCreate"
        >
          New Credential
        </VBtn>
      </div>
    </template>

    <p class="text-body-2 text-medium-emphasis mx-4 mb-2">
      A credential holds what's needed to connect to a provider. Register a model (for AI providers) or a repository/config (for GitHub/Jira) elsewhere in Settings to point at one.
    </p>

    <VAlert
      v-if="crud.error.value"
      type="error"
      variant="tonal"
      class="mx-4"
    >
      {{ crud.error.value }}
    </VAlert>

    <VDataTable
      :headers="headers"
      :items="crud.items.value"
      :loading="crud.loading.value"
      item-value="id"
    >
      <template #item.provider="{ item }">
        <VChip
          size="small"
          label
        >
          {{ providerConfig[item.provider]?.label ?? item.provider }}
        </VChip>
      </template>
      <template #item.secret_hint="{ item }">
        <span class="text-caption font-monospace">{{ item.secret_hint ?? '••••••••' }}</span>
      </template>
      <template #item.is_active="{ item }">
        <StatusChip :status="item.is_active" />
      </template>
      <template #item.actions="{ item }">
        <IconBtn @click="openReveal(item)">
          <VIcon icon="tabler-eye" />
        </IconBtn>
        <IconBtn @click="openEdit(item)">
          <VIcon icon="tabler-pencil" />
        </IconBtn>
        <IconBtn @click="askDelete(item.id)">
          <VIcon icon="tabler-trash" />
        </IconBtn>
      </template>
    </VDataTable>
  </VCard>

  <VDialog
    v-model="dialogOpen"
    max-width="560"
  >
    <VCard :title="editingId ? 'Edit Credential' : 'New Credential'">
      <VCardText>
        <VAlert
          v-if="formError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ formError }}
        </VAlert>
        <VForm
          ref="refForm"
          @submit.prevent="submit"
        >
          <VRow>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.name"
                label="Name"
                placeholder="Anthropic — production"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppSelect
                v-model="form.provider"
                :items="providerItems"
                item-title="title"
                item-value="value"
                label="Provider"
                :disabled="!!editingId"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol cols="12">
              <AppTextField
                v-model="form.secret"
                :label="currentProviderConfig.secretLabel"
                type="password"
                :rules="editingId ? [] : [requiredValidator]"
                :placeholder="editingId ? 'Leave blank to keep current secret' : currentProviderConfig.secretPlaceholder"
              />
            </VCol>
            <VCol
              v-for="field in currentProviderFields"
              :key="field.key"
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.extra[field.key]"
                :label="field.label"
                :placeholder="field.placeholder"
                :type="field.type ?? 'text'"
                :rules="field.required ? [requiredValidator] : []"
              />
            </VCol>
            <VCol cols="12">
              <JsonField
                ref="refAdvancedMetadata"
                v-model="form.advancedMetadata"
                label="Additional Metadata (JSON, optional)"
                :rows="3"
              />
            </VCol>
            <VCol
              v-if="editingId"
              cols="12"
            >
              <VSwitch
                v-model="form.isActive"
                label="Active"
              />
            </VCol>
          </VRow>
        </VForm>
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="dialogOpen = false"
        >
          Cancel
        </VBtn>
        <VBtn @click="submit">
          Save
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>

  <VDialog
    v-model="revealDialogOpen"
    max-width="480"
  >
    <VCard :title="`Secret — ${revealCredential?.name}`">
      <VCardText>
        <VAlert
          v-if="revealError"
          type="error"
          variant="tonal"
        >
          {{ revealError }}
        </VAlert>
        <VProgressLinear
          v-if="revealLoading"
          indeterminate
          class="mb-4"
        />
        <AppTextField
          v-else
          :model-value="revealSecret"
          label="Secret"
          readonly
          append-inner-icon="tabler-copy"
          @click:append-inner="copyRevealSecret"
        />
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="revealDialogOpen = false"
        >
          Close
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>

  <ConfirmDialog
    v-model="confirmOpen"
    title="Delete credential?"
    text="Any model or tool registered against it will stop working."
    @confirm="doDelete"
  />
</template>

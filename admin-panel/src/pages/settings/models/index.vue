<script setup>
definePage({
  meta: { navActiveLink: 'settings-models' },
})

const userData = useCookie('userData')
const tenants = ref([])
const selectedTenant = ref(userData.value?.tenantId ?? null)

const loadTenants = async () => {
  tenants.value = await ssoApi('/api/v1/tenants')
  if (!selectedTenant.value && tenants.value.length)
    selectedTenant.value = tenants.value[0].id
}

const providers = ['anthropic', 'openai', 'gemini', 'azure_openai', 'mistral', 'cohere', 'ollama', 'custom']
const capabilities = ['chat', 'completion', 'embedding', 'vision', 'reasoning']

// Local id -> name/provider cache so the table can show a credential's name
// instead of a bare UUID without re-fetching per row.
const credentialsById = ref({})

const loadCredentials = async () => {
  if (!selectedTenant.value)
    return

  const credentials = await sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/credentials`)

  credentialsById.value = Object.fromEntries(credentials.map(c => [c.id, c]))
}

const fetchCredentials = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/credentials`)

const crud = useCrud({
  list: () => selectedTenant.value ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/models`) : Promise.resolve([]),
  create: payload => sdlcPlatformApi('/api/v1/models', { method: 'POST', body: payload }),
  update: (id, payload) => sdlcPlatformApi(`/api/v1/models/${id}`, { method: 'PUT', body: payload }),
  remove: id => sdlcPlatformApi(`/api/v1/models/${id}`, { method: 'DELETE' }),
})

watch(selectedTenant, async () => {
  await loadCredentials()
  await crud.fetchAll()
})

const headers = [
  { title: 'Provider', key: 'provider' },
  { title: 'Model', key: 'model_name' },
  { title: 'Capability', key: 'capability' },
  { title: 'Credential', key: 'credential_id' },
  { title: 'Scope', key: 'tenant_id' },
  { title: 'Status', key: 'is_active' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()

const emptyForm = () => ({
  isGlobal: false,
  provider: 'anthropic',
  modelName: '',
  capability: 'chat',
  credentialId: null,
  endpointUrl: '',
  isActive: true,
})

const form = ref(emptyForm())

const openCreate = () => {
  editingId.value = null
  form.value = emptyForm()
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = model => {
  editingId.value = model.id
  form.value = {
    isGlobal: !model.tenant_id,
    provider: model.provider,
    modelName: model.model_name,
    capability: model.capability,
    credentialId: model.credential_id,
    endpointUrl: model.endpoint_url ?? '',
    isActive: model.is_active,
  }
  formError.value = ''
  dialogOpen.value = true
}

const submit = async () => {
  const { valid } = await refForm.value.validate()
  if (!valid)
    return

  formError.value = ''
  try {
    if (editingId.value) {
      await crud.update(editingId.value, {
        provider: form.value.provider,
        model_name: form.value.modelName,
        capability: form.value.capability,
        credential_id: form.value.credentialId,
        endpoint_url: form.value.endpointUrl || null,
        is_active: form.value.isActive,
      })
    }
    else {
      await crud.create({
        tenant_id: form.value.isGlobal ? null : selectedTenant.value,
        provider: form.value.provider,
        model_name: form.value.modelName,
        capability: form.value.capability,
        credential_id: form.value.credentialId,
        endpoint_url: form.value.endpointUrl || null,
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

onMounted(async () => {
  await loadTenants()
  await loadCredentials()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Model Registry">
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
          New Model
        </VBtn>
      </div>
    </template>

    <p class="text-body-2 text-medium-emphasis mx-4 mb-2">
      Registers a specific model exposed by a provider credential (e.g. claude-sonnet-5 via an Anthropic credential).
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
          class="text-capitalize"
        >
          {{ item.provider }}
        </VChip>
      </template>
      <template #item.credential_id="{ item }">
        {{ item.credential_id ? (credentialsById[item.credential_id]?.name ?? item.credential_id.slice(0, 8)) : '—' }}
      </template>
      <template #item.tenant_id="{ item }">
        {{ item.tenant_id ? 'Tenant' : 'Global' }}
      </template>
      <template #item.is_active="{ item }">
        <StatusChip :status="item.is_active" />
      </template>
      <template #item.actions="{ item }">
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
    <VCard :title="editingId ? 'Edit Model' : 'New Model'">
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
              <AppSelect
                v-model="form.provider"
                :items="providers"
                label="Provider"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.modelName"
                label="Model Name"
                placeholder="claude-sonnet-5"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppSelect
                v-model="form.capability"
                :items="capabilities"
                label="Capability"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="form.credentialId"
                label="Credential"
                :fetch-options="fetchCredentials"
                :item-title="c => `${c.name} (${c.provider})`"
                empty-text="No credentials for this tenant yet — create one in Settings → Credentials."
              />
            </VCol>
            <VCol cols="12">
              <AppTextField
                v-model="form.endpointUrl"
                label="Endpoint URL (optional override)"
              />
            </VCol>
            <VCol
              cols="12"
              class="d-flex align-center"
            >
              <VSwitch
                v-if="!editingId"
                v-model="form.isGlobal"
                label="Global model (not tenant-scoped)"
              />
              <VSwitch
                v-else
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

  <ConfirmDialog
    v-model="confirmOpen"
    title="Delete model?"
    text="This cannot be undone."
    @confirm="doDelete"
  />
</template>

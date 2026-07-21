<script setup>
definePage({
  meta: { navActiveLink: 'settings-jira' },
})

// Jira connections are stored as Tool rows with tool_type "jira" — see the
// GitHub settings page for why this maps named fields onto the opaque
// `config` JSON string instead of showing raw JSON.
const TOOL_TYPE = 'jira'

const userData = useCookie('userData')
const tenants = ref([])
const selectedTenant = ref(userData.value?.tenantId ?? null)

const loadTenants = async () => {
  tenants.value = await ssoApi('/api/v1/tenants')
  if (!selectedTenant.value && tenants.value.length)
    selectedTenant.value = tenants.value[0].id
}

const credentialsById = ref({})

const loadCredentials = async () => {
  if (!selectedTenant.value)
    return

  const credentials = await sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/credentials`)

  credentialsById.value = Object.fromEntries(credentials.map(c => [c.id, c]))
}

const fetchCredentials = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/credentials`)
  .then(credentials => credentials.filter(c => c.provider === TOOL_TYPE))

const parseConfig = raw => {
  try {
    return JSON.parse(raw ?? '{}')
  }
  catch {
    return {}
  }
}

const crud = useCrud({
  list: () => selectedTenant.value
    ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/tools`).then(tools => tools.filter(t => t.tool_type === TOOL_TYPE))
    : Promise.resolve([]),
  create: payload => sdlcPlatformApi('/api/v1/tools', { method: 'POST', body: payload }),
  update: (id, payload) => sdlcPlatformApi(`/api/v1/tools/${id}`, { method: 'PUT', body: payload }),
  remove: id => sdlcPlatformApi(`/api/v1/tools/${id}`, { method: 'DELETE' }),
})

watch(selectedTenant, async () => {
  await loadCredentials()
  await crud.fetchAll()
})

const headers = [
  { title: 'Name', key: 'name' },
  { title: 'Base URL', key: 'base_url' },
  { title: 'Project Key', key: 'project_key' },
  { title: 'Account Email', key: 'email' },
  { title: 'Credential', key: 'credential_id' },
  { title: 'Status', key: 'is_enabled' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const rows = computed(() => crud.items.value.map(tool => {
  const config = parseConfig(tool.config)

  return {
    ...tool,
    base_url: config.base_url ?? '—',
    project_key: config.project_key ?? '—',
    email: config.email ?? '—',
    credential_id: config.credential_id ?? null,
  }
}))

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()

const emptyForm = () => ({ name: '', description: '', baseUrl: '', projectKey: '', email: '', credentialId: null, webhookSecret: '', isEnabled: true })
const form = ref(emptyForm())

const generateWebhookSecret = () => {
  form.value.webhookSecret = crypto.randomUUID().replace(/-/g, '')
}

const openCreate = () => {
  editingId.value = null
  form.value = emptyForm()
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = row => {
  editingId.value = row.id

  const config = parseConfig(row.config)

  form.value = {
    name: row.name,
    description: row.description ?? '',
    baseUrl: config.base_url ?? '',
    projectKey: config.project_key ?? '',
    email: config.email ?? '',
    credentialId: config.credential_id ?? null,
    webhookSecret: config.webhook_secret ?? '',
    isEnabled: row.is_enabled,
  }
  formError.value = ''
  dialogOpen.value = true
}

const buildConfig = () => JSON.stringify({
  base_url: form.value.baseUrl,
  project_key: form.value.projectKey,
  email: form.value.email || null,
  credential_id: form.value.credentialId,
  webhook_secret: form.value.webhookSecret || null,
})

const submit = async () => {
  const { valid } = await refForm.value.validate()
  if (!valid)
    return

  formError.value = ''
  try {
    if (editingId.value) {
      await crud.update(editingId.value, {
        name: form.value.name,
        description: form.value.description || null,
        config: buildConfig(),
        is_enabled: form.value.isEnabled,
      })
    }
    else {
      await crud.create({
        tenant_id: selectedTenant.value,
        name: form.value.name,
        tool_type: TOOL_TYPE,
        description: form.value.description || null,
        config: buildConfig(),
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
  <VCard title="Jira Configuration">
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
          New Jira Connection
        </VBtn>
      </div>
    </template>

    <p class="text-body-2 text-medium-emphasis mx-4 mb-2">
      Connects the pipeline to a Jira project for reading tickets and posting updates. The credential should hold a Jira API token for the account email below.
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
      :items="rows"
      :loading="crud.loading.value"
      item-value="id"
    >
      <template #item.credential_id="{ item }">
        {{ item.credential_id ? (credentialsById[item.credential_id]?.name ?? item.credential_id.slice(0, 8)) : '—' }}
      </template>
      <template #item.is_enabled="{ item }">
        <StatusChip :status="item.is_enabled" />
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
    <VCard :title="editingId ? 'Edit Jira Connection' : 'New Jira Connection'">
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
                placeholder="Main Jira project"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.projectKey"
                label="Project Key"
                placeholder="PROJ"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.baseUrl"
                label="Base URL"
                placeholder="https://yourorg.atlassian.net"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.email"
                label="Account Email"
                type="email"
                placeholder="bot@yourorg.com"
              />
            </VCol>
            <VCol cols="12">
              <RemoteSelect
                v-model="form.credentialId"
                label="Credential (API token)"
                :fetch-options="fetchCredentials"
                :item-title="c => `${c.name} (${c.provider})`"
                :rules="[requiredValidator]"
                empty-text="No Jira credentials for this tenant yet — create one in Settings → Credentials with provider Jira."
              />
            </VCol>
            <VCol cols="12">
              <div class="d-flex align-center gap-2">
                <AppTextField
                  v-model="form.webhookSecret"
                  label="Webhook Secret (optional)"
                  placeholder="Generate one to enable inbound Jira webhooks"
                  class="flex-grow-1"
                />
                <VBtn
                  variant="tonal"
                  color="secondary"
                  @click="generateWebhookSecret"
                >
                  Generate
                </VBtn>
              </div>
              <p class="text-body-2 text-medium-emphasis mt-1 mb-0">
                Required for a Project linked to this connection to receive Jira Automation webhooks. Each linked project's edit dialog shows the exact URL to paste into Jira.
              </p>
            </VCol>
            <VCol cols="12">
              <AppTextarea
                v-model="form.description"
                label="Description"
                rows="2"
              />
            </VCol>
            <VCol
              v-if="editingId"
              cols="12"
            >
              <VSwitch
                v-model="form.isEnabled"
                label="Enabled"
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
    title="Delete this Jira connection?"
    text="Any SDLC run configured to use it will stop working."
    @confirm="doDelete"
  />
</template>

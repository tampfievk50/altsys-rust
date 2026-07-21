<script setup>
definePage({
  meta: { navActiveLink: 'project-management' },
})

const router = useRouter()
const userData = useCookie('userData')
const tenants = ref([])
const selectedTenant = ref(userData.value?.tenantId ?? null)

const loadTenants = async () => {
  tenants.value = await ssoApi('/api/v1/tenants')
  if (!selectedTenant.value && tenants.value.length)
    selectedTenant.value = tenants.value[0].id
}

const crud = useCrud({
  list: () => selectedTenant.value ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/projects`) : Promise.resolve([]),
  create: payload => sdlcPlatformApi('/api/v1/projects', { method: 'POST', body: payload }),
  update: (id, payload) => sdlcPlatformApi(`/api/v1/projects/${id}`, { method: 'PUT', body: payload }),
  remove: id => sdlcPlatformApi(`/api/v1/projects/${id}`, { method: 'DELETE' }),
})

const toolsById = ref({})

const loadTools = async () => {
  if (!selectedTenant.value)
    return

  const tools = await sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/tools`)

  toolsById.value = Object.fromEntries(tools.map(t => [t.id, t]))
}

const fetchGithubTools = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/tools`)
  .then(tools => tools.filter(t => t.tool_type === 'github'))

const fetchJiraTools = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/tools`)
  .then(tools => tools.filter(t => t.tool_type === 'jira'))

const parseToolConfig = raw => {
  try {
    return JSON.parse(raw ?? '{}')
  }
  catch {
    return {}
  }
}

// Only resolvable once editing an existing project (the URL needs its ID) with
// a Jira config selected whose Settings → Jira page has a webhook secret set.
const jiraWebhookUrl = computed(() => {
  if (!editingId.value || !form.value.jiraToolId)
    return ''

  const secret = parseToolConfig(toolsById.value[form.value.jiraToolId]?.config).webhook_secret

  return secret ? `${import.meta.env.VITE_SDLC_PLATFORM_API_URL}/api/v1/webhooks/jira/${editingId.value}/${secret}` : ''
})

watch(selectedTenant, async () => {
  await loadTools()
  await crud.fetchAll()
})

const headers = [
  { title: 'Name', key: 'name' },
  { title: 'Slug', key: 'slug' },
  { title: 'GitHub Config', key: 'github_tool_id' },
  { title: 'Jira Config', key: 'jira_tool_id' },
  { title: 'Status', key: 'is_active' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()
const refWorkflowConfig = ref()

const emptyForm = () => ({
  name: '',
  slug: '',
  githubToolId: null,
  defaultBranch: '',
  jiraToolId: null,
  buildCommand: '',
  testCommand: '',
  codingStandards: '',
  workflowConfig: '',
  isActive: true,
})

const form = ref(emptyForm())

const openCreate = () => {
  editingId.value = null
  form.value = emptyForm()
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = project => {
  editingId.value = project.id
  form.value = {
    name: project.name,
    slug: project.slug,
    githubToolId: project.github_tool_id,
    defaultBranch: project.default_branch ?? '',
    jiraToolId: project.jira_tool_id ?? null,
    buildCommand: project.build_command ?? '',
    testCommand: project.test_command ?? '',
    codingStandards: project.coding_standards ?? '',
    workflowConfig: project.workflow_config ?? '',
    isActive: project.is_active,
  }
  formError.value = ''
  dialogOpen.value = true
}

const submit = async () => {
  const { valid } = await refForm.value.validate()
  if (!valid || !(refWorkflowConfig.value?.isValid() ?? true))
    return

  formError.value = ''
  try {
    if (editingId.value) {
      await crud.update(editingId.value, {
        name: form.value.name,
        github_tool_id: form.value.githubToolId,
        default_branch: form.value.defaultBranch || null,
        jira_tool_id: form.value.jiraToolId,
        build_command: form.value.buildCommand || null,
        test_command: form.value.testCommand || null,
        coding_standards: form.value.codingStandards || null,
        workflow_config: form.value.workflowConfig || null,
        is_active: form.value.isActive,
      })
    }
    else {
      await crud.create({
        tenant_id: selectedTenant.value,
        name: form.value.name,
        slug: form.value.slug,
        github_tool_id: form.value.githubToolId,
        default_branch: form.value.defaultBranch || null,
        jira_tool_id: form.value.jiraToolId,
        build_command: form.value.buildCommand || null,
        test_command: form.value.testCommand || null,
        coding_standards: form.value.codingStandards || null,
        workflow_config: form.value.workflowConfig || null,
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
  await loadTools()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Projects">
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
          New Project
        </VBtn>
      </div>
    </template>

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
      class="clickable-rows"
      @click:row="(_event, { item }) => router.push(`/project-management/${item.id}/tasks`)"
    >
      <template #item.name="{ item }">
        <span class="font-weight-medium text-primary">{{ item.name }}</span>
      </template>
      <template #item.is_active="{ item }">
        <StatusChip :status="item.is_active" />
      </template>
      <template #item.github_tool_id="{ item }">
        {{ toolsById[item.github_tool_id]?.name ?? '—' }}
      </template>
      <template #item.jira_tool_id="{ item }">
        {{ item.jira_tool_id ? (toolsById[item.jira_tool_id]?.name ?? '—') : '—' }}
      </template>
      <template #item.actions="{ item }">
        <IconBtn @click.stop="openEdit(item)">
          <VIcon icon="tabler-pencil" />
        </IconBtn>
        <IconBtn @click.stop="askDelete(item.id)">
          <VIcon icon="tabler-trash" />
        </IconBtn>
      </template>
    </VDataTable>
  </VCard>

  <VDialog
    v-model="dialogOpen"
    max-width="640"
  >
    <VCard :title="editingId ? 'Edit Project' : 'New Project'">
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
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              v-if="!editingId"
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.slug"
                label="Slug"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="form.githubToolId"
                label="GitHub Config"
                :fetch-options="fetchGithubTools"
                :rules="[requiredValidator]"
                empty-text="No GitHub config for this tenant yet — create one in Settings → GitHub."
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.defaultBranch"
                label="Default Branch"
                placeholder="main"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="form.jiraToolId"
                label="Jira Config (optional)"
                :fetch-options="fetchJiraTools"
                empty-text="No Jira config for this tenant yet — create one in Settings → Jira."
              />
            </VCol>
            <VCol
              v-if="form.jiraToolId"
              cols="12"
            >
              <AppTextField
                v-if="jiraWebhookUrl"
                :model-value="jiraWebhookUrl"
                label="Jira Webhook URL"
                readonly
                append-inner-icon="tabler-copy"
                @click:append-inner="navigator.clipboard.writeText(jiraWebhookUrl)"
              />
              <p
                v-else
                class="text-body-2 text-medium-emphasis mb-0"
              >
                {{ editingId ? 'Set a webhook secret on this Jira config (Settings → Jira) to get a webhook URL to paste into Jira Automation.' : 'Save this project first, then reopen it here to get the webhook URL to paste into Jira Automation.' }}
              </p>
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.buildCommand"
                label="Build Command"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.testCommand"
                label="Test Command"
              />
            </VCol>
            <VCol
              v-if="editingId"
              cols="12"
              md="6"
              class="d-flex align-center"
            >
              <VSwitch
                v-model="form.isActive"
                label="Active"
              />
            </VCol>
            <VCol cols="12">
              <AppTextarea
                v-model="form.codingStandards"
                label="Coding Standards"
                rows="3"
              />
            </VCol>
            <VCol cols="12">
              <JsonField
                ref="refWorkflowConfig"
                v-model="form.workflowConfig"
                label="Workflow Config (JSON)"
                :rows="4"
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
    title="Delete project?"
    text="This cannot be undone."
    @confirm="doDelete"
  />
</template>

<style scoped>
.clickable-rows :deep(tbody tr) {
  cursor: pointer;
}
</style>

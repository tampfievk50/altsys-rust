<script setup>
definePage({
  meta: { navActiveLink: 'sdlc' },
})

const userData = useCookie('userData')
const tenants = ref([])
const selectedTenant = ref(userData.value?.tenantId ?? null)

const loadTenants = async () => {
  tenants.value = await ssoApi('/api/v1/tenants')
  if (!selectedTenant.value && tenants.value.length)
    selectedTenant.value = tenants.value[0].id
}

const crud = useCrud({
  list: () => selectedTenant.value ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/sdlc-runs`) : Promise.resolve([]),
})

watch(selectedTenant, crud.fetchAll)

const headers = [
  { title: 'Ticket', key: 'ticket_key' },
  { title: 'Status', key: 'status' },
  { title: 'Current Step', key: 'current_step' },
  { title: 'Branch', key: 'branch_name' },
  { title: 'Pull Request', key: 'pull_request_url' },
  { title: 'Started', key: 'started_at' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const agentsByType = type =>
  sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/agents`).then(agents => agents.filter(a => a.agent_type === type))

const fetchTools = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/tools`)
const fetchProjects = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/projects`)

// ===== Start run =====
const startDialogOpen = ref(false)
const startError = ref('')
const startLoading = ref(false)
const refForm = ref()

const emptyStartForm = () => ({
  projectId: null,
  ticketKey: '',
  ticketSummary: '',
  plannerAgentId: null,
  architectAgentId: null,
  developerAgentId: null,
  reviewerAgentId: null,
  documentationAgentId: null,
  gitToolId: null,
  buildToolId: null,
  filesystemToolId: null,
  githubToolId: null,
  jiraToolId: null,
})

const startForm = ref(emptyStartForm())

const openStart = () => {
  startForm.value = emptyStartForm()
  startError.value = ''
  startDialogOpen.value = true
}

const submitStart = async () => {
  const { valid } = await refForm.value.validate()
  if (!valid)
    return

  startLoading.value = true
  startError.value = ''
  try {
    await sdlcPlatformApi('/api/v1/sdlc-runs', {
      method: 'POST',
      body: {
        tenant_id: selectedTenant.value,
        project_id: startForm.value.projectId,
        ticket_key: startForm.value.ticketKey,
        ticket_summary: startForm.value.ticketSummary || null,
        planner_agent_id: startForm.value.plannerAgentId,
        architect_agent_id: startForm.value.architectAgentId,
        developer_agent_id: startForm.value.developerAgentId,
        reviewer_agent_id: startForm.value.reviewerAgentId,
        documentation_agent_id: startForm.value.documentationAgentId,
        git_tool_id: startForm.value.gitToolId,
        build_tool_id: startForm.value.buildToolId,
        filesystem_tool_id: startForm.value.filesystemToolId,
        github_tool_id: startForm.value.githubToolId || null,
        jira_tool_id: startForm.value.jiraToolId || null,
      },
    })
    startDialogOpen.value = false
    await crud.fetchAll()
  }
  catch (err) {
    startError.value = err.message || 'Failed to start run'
  }
  finally {
    startLoading.value = false
  }
}

// ===== Step detail =====
const detailDialogOpen = ref(false)
const detailRun = ref(null)
const detailSteps = ref([])
const detailLoading = ref(false)
const detailError = ref('')

const loadDetail = async run => {
  detailRun.value = run
  detailDialogOpen.value = true
  detailLoading.value = true
  detailError.value = ''
  try {
    detailSteps.value = await sdlcPlatformApi(`/api/v1/sdlc-runs/${run.id}/steps`)
  }
  catch (err) {
    detailError.value = err.message || 'Failed to load step executions'
  }
  finally {
    detailLoading.value = false
  }
}

const timelineItems = computed(() => detailSteps.value.map(s => ({
  title: s.step,
  status: s.status,
  attempt: s.attempt,
  output: s.output,
  error: s.error,
  timestamp: s.completed_at ?? s.started_at,
})))

onMounted(async () => {
  await loadTenants()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Autonomous SDLC Runs">
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
          prepend-icon="tabler-player-play"
          :disabled="!selectedTenant"
          @click="openStart"
        >
          Start Run
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
    >
      <template #item.status="{ item }">
        <StatusChip :status="item.status" />
      </template>
      <template #item.current_step="{ item }">
        {{ item.current_step ?? '—' }}
      </template>
      <template #item.branch_name="{ item }">
        {{ item.branch_name ?? '—' }}
      </template>
      <template #item.pull_request_url="{ item }">
        <a
          v-if="item.pull_request_url"
          :href="item.pull_request_url"
          target="_blank"
          rel="noopener noreferrer"
        >{{ item.pull_request_url }}</a>
        <span v-else>—</span>
      </template>
      <template #item.started_at="{ item }">
        {{ item.started_at ? new Date(item.started_at).toLocaleString() : '—' }}
      </template>
      <template #item.actions="{ item }">
        <IconBtn @click="loadDetail(item)">
          <VIcon icon="tabler-list-details" />
        </IconBtn>
      </template>
    </VDataTable>
  </VCard>

  <VDialog
    v-model="startDialogOpen"
    max-width="680"
  >
    <VCard title="Start SDLC Run">
      <VCardText>
        <VAlert
          v-if="startError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ startError }}
        </VAlert>
        <VForm
          ref="refForm"
          @submit.prevent="submitStart"
        >
          <VRow>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.projectId"
                label="Project"
                :fetch-options="fetchProjects"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="startForm.ticketKey"
                label="Ticket Key"
                placeholder="PROJ-123"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol cols="12">
              <AppTextarea
                v-model="startForm.ticketSummary"
                label="Ticket Summary (optional if a Jira tool is set)"
                rows="2"
              />
            </VCol>

            <VCol cols="12">
              <h6 class="text-h6 mb-2">
                Agents
              </h6>
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.plannerAgentId"
                label="Planner Agent"
                :fetch-options="() => agentsByType('planner')"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.architectAgentId"
                label="Architect Agent"
                :fetch-options="() => agentsByType('architect')"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.developerAgentId"
                label="Developer Agent"
                :fetch-options="() => agentsByType('developer')"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.reviewerAgentId"
                label="Reviewer Agent"
                :fetch-options="() => agentsByType('reviewer')"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.documentationAgentId"
                label="Documentation Agent"
                :fetch-options="() => agentsByType('documentation')"
                :rules="[requiredValidator]"
              />
            </VCol>

            <VCol cols="12">
              <h6 class="text-h6 mb-2">
                Tools
              </h6>
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.gitToolId"
                label="Git Tool"
                :fetch-options="fetchTools"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.buildToolId"
                label="Build Tool"
                :fetch-options="fetchTools"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.filesystemToolId"
                label="Filesystem Tool"
                :fetch-options="fetchTools"
                :rules="[requiredValidator]"
                empty-text="No Filesystem tool yet — create one (tool_type: filesystem) with the same working_directory as the Git Tool."
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.githubToolId"
                label="GitHub Tool (optional)"
                :fetch-options="fetchTools"
                :rules="[]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="startForm.jiraToolId"
                label="Jira Tool (optional)"
                :fetch-options="fetchTools"
                :rules="[]"
              />
            </VCol>
          </VRow>
        </VForm>
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="startDialogOpen = false"
        >
          Cancel
        </VBtn>
        <VBtn
          :loading="startLoading"
          @click="submitStart"
        >
          Start
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>

  <VDialog
    v-model="detailDialogOpen"
    max-width="720"
  >
    <VCard :title="`Run — ${detailRun?.ticket_key}`">
      <VCardText>
        <VAlert
          v-if="detailError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ detailError }}
        </VAlert>
        <VProgressLinear
          v-if="detailLoading"
          indeterminate
          class="mb-4"
        />
        <p
          v-else-if="!timelineItems.length"
          class="text-body-2 text-medium-emphasis"
        >
          No step executions yet.
        </p>
        <StepTimeline
          v-else
          :items="timelineItems"
        />
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="detailDialogOpen = false"
        >
          Close
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>
</template>

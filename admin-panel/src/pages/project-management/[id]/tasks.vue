<script setup>
definePage({
  meta: { navActiveLink: 'project-management' },
})

const route = useRoute()
const router = useRouter()

const project = ref(null)
const loading = ref(false)
const loadError = ref('')
const events = ref([])
const overrides = ref([])
const sdlcRuns = ref([])

const parsePayload = raw => {
  try {
    return JSON.parse(raw)
  }
  catch {
    return null
  }
}

const load = async () => {
  loading.value = true
  loadError.value = ''
  try {
    project.value = await sdlcPlatformApi(`/api/v1/projects/${route.params.id}`)
    events.value = await sdlcPlatformApi(`/api/v1/tenants/${project.value.tenant_id}/events`)
    overrides.value = await sdlcPlatformApi(`/api/v1/projects/${route.params.id}/task-overrides`)
    sdlcRuns.value = await sdlcPlatformApi(`/api/v1/tenants/${project.value.tenant_id}/sdlc-runs`)
  }
  catch (err) {
    loadError.value = err.message || 'Failed to load'
  }
  finally {
    loading.value = false
  }
}

const overridesByTicket = computed(() => Object.fromEntries(overrides.value.map(o => [o.ticket_key, o])))

// Latest run per ticket in this project — "latest" by started_at (falling back
// to created_at for a run that errored before it could start), so a retry
// always looks at the most recent attempt, not just whichever came back first.
const latestRunByTicket = computed(() => {
  const byTicket = new Map()
  const projectRuns = sdlcRuns.value.filter(r => String(r.project_id) === String(route.params.id))

  for (const run of projectRuns) {
    const existing = byTicket.get(run.ticket_key)
    const runTime = new Date(run.started_at ?? run.created_at)
    if (!existing || runTime > new Date(existing.started_at ?? existing.created_at))
      byTicket.set(run.ticket_key, run)
  }

  return byTicket
})

// One row per ticket, deduplicated to whichever event for that ticket arrived
// most recently — a ticket may have been ingested more than once (created,
// then updated) and ingestion pushes one event per occurrence, not per ticket.
// A locally-edited summary wins over whatever Jira last reported, until the
// next ingested event confirms it (Jira itself gets the same edit — see
// `saveSummary` — so the two converge once that event arrives).
const tasks = computed(() => {
  const byTicket = new Map()

  const projectEvents = events.value
    .map(event => ({ event, payload: parsePayload(event.payload) }))
    .filter(({ payload }) => payload && String(payload.project_id) === String(route.params.id) && payload.ticket_key)
    .sort((a, b) => new Date(a.event.received_at) - new Date(b.event.received_at))

  for (const { event, payload } of projectEvents) {
    byTicket.set(payload.ticket_key, {
      ticketKey: payload.ticket_key,
      summary: payload.summary ?? '—',
      issueType: payload.issue_type ?? '—',
      priority: payload.priority ?? '—',
      lastEventType: event.event_type,
      lastSeen: event.received_at,
      eventId: event.id,
    })
  }

  for (const [ticketKey, override] of Object.entries(overridesByTicket.value)) {
    const existing = byTicket.get(ticketKey)
    if (existing)
      existing.summary = override.summary
  }

  return [...byTicket.values()].sort((a, b) => new Date(b.lastSeen) - new Date(a.lastSeen))
})

const headers = [
  { title: 'Ticket', key: 'ticketKey' },
  { title: 'Summary', key: 'summary' },
  { title: 'Type', key: 'issueType' },
  { title: 'Priority', key: 'priority' },
  { title: 'Last Event', key: 'lastEventType' },
  { title: 'Last Seen', key: 'lastSeen' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

// ===== Firings detail (for the task's most recent event) =====
const detailDialogOpen = ref(false)
const detailTask = ref(null)
const detailFirings = ref([])
const detailLoading = ref(false)
const detailError = ref('')

const loadFirings = async task => {
  detailTask.value = task
  detailDialogOpen.value = true
  detailLoading.value = true
  detailError.value = ''
  try {
    detailFirings.value = await sdlcPlatformApi(`/api/v1/events/${task.eventId}/firings`)
  }
  catch (err) {
    detailError.value = err.message || 'Failed to load rule firings'
  }
  finally {
    detailLoading.value = false
  }
}

const timelineItems = computed(() => detailFirings.value.map(f => ({
  title: `Rule ${f.rule_id.slice(0, 8)} — ${f.matched ? 'matched' : 'no match'}`,
  status: f.status,
  output: f.action_result,
  error: f.error,
  timestamp: f.created_at,
})))

// ===== Edit summary =====
const editDialogOpen = ref(false)
const editingTask = ref(null)
const editSummary = ref('')
const editError = ref('')
const editLoading = ref(false)

const openEditSummary = task => {
  editingTask.value = task
  editSummary.value = task.summary
  editError.value = ''
  editDialogOpen.value = true
}

const saveSummary = async () => {
  if (!editingTask.value)
    return

  editLoading.value = true
  editError.value = ''
  try {
    await sdlcPlatformApi(`/api/v1/projects/${route.params.id}/tickets/${editingTask.value.ticketKey}`, {
      method: 'PUT',
      body: { summary: editSummary.value },
    })
    overrides.value = await sdlcPlatformApi(`/api/v1/projects/${route.params.id}/task-overrides`)
    editDialogOpen.value = false
  }
  catch (err) {
    editError.value = err.message || 'Failed to update summary'
  }
  finally {
    editLoading.value = false
  }
}

// ===== Rerun a failed run =====
// Manual only, by design — a failed run never retries itself. The previous
// run's agent/tool selections aren't stored anywhere, so re-running means
// picking them again, same as starting any other run.
const agentsByType = type =>
  sdlcPlatformApi(`/api/v1/tenants/${project.value.tenant_id}/agents`).then(agents => agents.filter(a => a.agent_type === type))

const fetchTools = () => sdlcPlatformApi(`/api/v1/tenants/${project.value.tenant_id}/tools`)

const rerunDialogOpen = ref(false)
const rerunTask = ref(null)
const rerunError = ref('')
const rerunLoading = ref(false)
const refRerunForm = ref()

const emptyRerunForm = () => ({
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

const rerunForm = ref(emptyRerunForm())

const openRerun = task => {
  rerunTask.value = task
  rerunForm.value = emptyRerunForm()
  rerunError.value = ''
  rerunDialogOpen.value = true
}

const submitRerun = async () => {
  const { valid } = await refRerunForm.value.validate()
  if (!valid)
    return

  rerunLoading.value = true
  rerunError.value = ''
  try {
    await sdlcPlatformApi('/api/v1/sdlc-runs', {
      method: 'POST',
      body: {
        tenant_id: project.value.tenant_id,
        project_id: route.params.id,
        ticket_key: rerunTask.value.ticketKey,
        ticket_summary: rerunTask.value.summary || null,
        planner_agent_id: rerunForm.value.plannerAgentId,
        architect_agent_id: rerunForm.value.architectAgentId,
        developer_agent_id: rerunForm.value.developerAgentId,
        reviewer_agent_id: rerunForm.value.reviewerAgentId,
        documentation_agent_id: rerunForm.value.documentationAgentId,
        git_tool_id: rerunForm.value.gitToolId,
        build_tool_id: rerunForm.value.buildToolId,
        filesystem_tool_id: rerunForm.value.filesystemToolId,
        github_tool_id: rerunForm.value.githubToolId || null,
        jira_tool_id: rerunForm.value.jiraToolId || null,
      },
    })
    rerunDialogOpen.value = false
    sdlcRuns.value = await sdlcPlatformApi(`/api/v1/tenants/${project.value.tenant_id}/sdlc-runs`)
  }
  catch (err) {
    rerunError.value = err.message || 'Failed to start run'
  }
  finally {
    rerunLoading.value = false
  }
}

onMounted(load)
</script>

<template>
  <VCard>
    <template #title>
      <div class="d-flex align-center gap-2">
        <IconBtn @click="router.push('/project-management')">
          <VIcon icon="tabler-arrow-left" />
        </IconBtn>
        <span>{{ project ? `Tasks — ${project.name}` : 'Tasks' }}</span>
      </div>
    </template>
    <template #append>
      <IconBtn @click="load">
        <VIcon icon="tabler-refresh" />
      </IconBtn>
    </template>

    <p class="text-body-2 text-medium-emphasis mx-4 mb-2">
      One row per Jira ticket ever ingested for this project (via webhook or the polling fallback), deduplicated to its most recent event.
    </p>

    <VAlert
      v-if="loadError"
      type="error"
      variant="tonal"
      class="mx-4"
    >
      {{ loadError }}
    </VAlert>

    <VDataTable
      :headers="headers"
      :items="tasks"
      :loading="loading"
      item-value="ticketKey"
    >
      <template #item.ticketKey="{ item }">
        <span class="font-weight-medium text-primary">{{ item.ticketKey }}</span>
      </template>
      <template #item.lastSeen="{ item }">
        {{ new Date(item.lastSeen).toLocaleString() }}
      </template>
      <template #item.actions="{ item }">
        <IconBtn @click="openEditSummary(item)">
          <VIcon icon="tabler-pencil" />
        </IconBtn>
        <IconBtn @click="loadFirings(item)">
          <VIcon icon="tabler-list-details" />
        </IconBtn>
        <IconBtn
          v-if="latestRunByTicket.get(item.ticketKey)?.status === 'failed'"
          @click="openRerun(item)"
        >
          <VIcon icon="tabler-player-play" />
        </IconBtn>
      </template>
      <template #no-data>
        <p class="text-body-2 text-medium-emphasis pa-4 mb-0">
          No tickets ingested yet for this project — either no Jira webhook/ticket has arrived, or the Jira tool's polling fallback hasn't synced yet.
        </p>
      </template>
    </VDataTable>
  </VCard>

  <VDialog
    v-model="detailDialogOpen"
    max-width="640"
  >
    <VCard :title="`${detailTask?.ticketKey} — automation rules evaluated`">
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
          No active rule matched this ticket's most recent event.
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

  <VDialog
    v-model="editDialogOpen"
    max-width="560"
  >
    <VCard :title="`Edit summary — ${editingTask?.ticketKey}`">
      <VCardText>
        <VAlert
          v-if="editError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ editError }}
        </VAlert>
        <p class="text-body-2 text-medium-emphasis mb-4">
          Saves to the real Jira ticket, then updates this list right away.
        </p>
        <AppTextarea
          v-model="editSummary"
          label="Summary"
          rows="2"
          autofocus
        />
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="editDialogOpen = false"
        >
          Cancel
        </VBtn>
        <VBtn
          :loading="editLoading"
          @click="saveSummary"
        >
          Save
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>

  <VDialog
    v-model="rerunDialogOpen"
    max-width="680"
  >
    <VCard :title="`Run again — ${rerunTask?.ticketKey}`">
      <VCardText>
        <VAlert
          v-if="rerunError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ rerunError }}
        </VAlert>
        <p class="text-body-2 text-medium-emphasis mb-4">
          The last run's agent/tool selections aren't saved anywhere, so pick them again for this attempt.
        </p>
        <VForm
          ref="refRerunForm"
          @submit.prevent="submitRerun"
        >
          <VRow>
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
                v-model="rerunForm.plannerAgentId"
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
                v-model="rerunForm.architectAgentId"
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
                v-model="rerunForm.developerAgentId"
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
                v-model="rerunForm.reviewerAgentId"
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
                v-model="rerunForm.documentationAgentId"
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
                v-model="rerunForm.gitToolId"
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
                v-model="rerunForm.buildToolId"
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
                v-model="rerunForm.filesystemToolId"
                label="Filesystem Tool"
                :fetch-options="fetchTools"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <RemoteSelect
                v-model="rerunForm.githubToolId"
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
                v-model="rerunForm.jiraToolId"
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
          @click="rerunDialogOpen = false"
        >
          Cancel
        </VBtn>
        <VBtn
          :loading="rerunLoading"
          @click="submitRerun"
        >
          Run
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>
</template>

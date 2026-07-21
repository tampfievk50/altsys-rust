<script setup>
definePage({
  meta: { navActiveLink: 'workflow-executions' },
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
  list: () => selectedTenant.value ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/workflow-executions`) : Promise.resolve([]),
})

watch(selectedTenant, crud.fetchAll)

const headers = [
  { title: 'ID', key: 'id' },
  { title: 'Definition', key: 'workflow_definition_id' },
  { title: 'Status', key: 'status' },
  { title: 'Started', key: 'started_at' },
  { title: 'Completed', key: 'completed_at' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

// ===== Start execution =====
const startDialogOpen = ref(false)
const startError = ref('')
const startLoading = ref(false)
const refForm = ref()
const refContext = ref()
const startForm = ref({ workflowDefinitionId: null, context: '' })

const openStart = () => {
  startForm.value = { workflowDefinitionId: null, context: '' }
  startError.value = ''
  startDialogOpen.value = true
}

const submitStart = async () => {
  const { valid } = await refForm.value.validate()
  if (!valid || !(refContext.value?.isValid() ?? true))
    return

  startLoading.value = true
  startError.value = ''
  try {
    await sdlcPlatformApi('/api/v1/workflow-executions', {
      method: 'POST',
      body: {
        tenant_id: selectedTenant.value,
        workflow_definition_id: startForm.value.workflowDefinitionId,
        context: startForm.value.context || null,
      },
    })
    startDialogOpen.value = false
    await crud.fetchAll()
  }
  catch (err) {
    startError.value = err.message || 'Failed to start execution'
  }
  finally {
    startLoading.value = false
  }
}

// ===== Detail / node executions / approval =====
const detailDialogOpen = ref(false)
const detailExecution = ref(null)
const detailNodes = ref([])
const detailLoading = ref(false)
const detailError = ref('')
const decideLoading = ref(false)

const loadDetail = async execution => {
  detailExecution.value = execution
  detailDialogOpen.value = true
  detailLoading.value = true
  detailError.value = ''
  try {
    detailNodes.value = await sdlcPlatformApi(`/api/v1/workflow-executions/${execution.id}/node-executions`)
  }
  catch (err) {
    detailError.value = err.message || 'Failed to load node executions'
  }
  finally {
    detailLoading.value = false
  }
}

const timelineItems = computed(() => detailNodes.value.map(n => ({
  title: n.node_id,
  status: n.status,
  attempt: n.attempt,
  output: n.output,
  error: n.error,
  timestamp: n.completed_at ?? n.started_at,
})))

const pendingApprovalNodes = computed(() => detailNodes.value.filter(n => n.status === 'waiting_approval'))

const decide = async (nodeId, approved) => {
  decideLoading.value = true
  detailError.value = ''
  try {
    const updated = await sdlcPlatformApi(`/api/v1/workflow-executions/${detailExecution.value.id}/nodes/${nodeId}/decide`, {
      method: 'POST',
      body: { approved, comment: null },
    })

    detailExecution.value = updated
    detailNodes.value = await sdlcPlatformApi(`/api/v1/workflow-executions/${detailExecution.value.id}/node-executions`)
    await crud.fetchAll()
  }
  catch (err) {
    detailError.value = err.message || 'Failed to record decision'
  }
  finally {
    decideLoading.value = false
  }
}

onMounted(async () => {
  await loadTenants()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Workflow Executions">
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
          Start Execution
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
      <template #item.id="{ item }">
        <span class="text-caption">{{ item.id.slice(0, 8) }}</span>
      </template>
      <template #item.workflow_definition_id="{ item }">
        <span class="text-caption">{{ item.workflow_definition_id.slice(0, 8) }}</span>
      </template>
      <template #item.status="{ item }">
        <StatusChip :status="item.status" />
      </template>
      <template #item.started_at="{ item }">
        {{ item.started_at ? new Date(item.started_at).toLocaleString() : '—' }}
      </template>
      <template #item.completed_at="{ item }">
        {{ item.completed_at ? new Date(item.completed_at).toLocaleString() : '—' }}
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
    max-width="560"
  >
    <VCard title="Start Workflow Execution">
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
            <VCol cols="12">
              <RemoteSelect
                v-model="startForm.workflowDefinitionId"
                label="Workflow Definition"
                :fetch-options="() => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant}/workflow-definitions`)"
                :item-title="d => `${d.name} (v${d.version})`"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol cols="12">
              <JsonField
                ref="refContext"
                v-model="startForm.context"
                label="Context (JSON, optional)"
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
    <VCard :title="`Execution — ${detailExecution?.id.slice(0, 8)}`">
      <VCardText>
        <VAlert
          v-if="detailError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ detailError }}
        </VAlert>

        <div
          v-if="pendingApprovalNodes.length"
          class="mb-4"
        >
          <VAlert
            v-for="n in pendingApprovalNodes"
            :key="n.id"
            type="warning"
            variant="tonal"
            class="mb-2"
          >
            <div class="d-flex align-center justify-space-between gap-4 flex-wrap">
              <span>Node <strong>{{ n.node_id }}</strong> is waiting for approval.</span>
              <div class="d-flex gap-2">
                <VBtn
                  size="small"
                  color="success"
                  :loading="decideLoading"
                  @click="decide(n.node_id, true)"
                >
                  Approve
                </VBtn>
                <VBtn
                  size="small"
                  color="error"
                  variant="tonal"
                  :loading="decideLoading"
                  @click="decide(n.node_id, false)"
                >
                  Reject
                </VBtn>
              </div>
            </div>
          </VAlert>
        </div>

        <VProgressLinear
          v-if="detailLoading"
          indeterminate
          class="mb-4"
        />
        <p
          v-else-if="!timelineItems.length"
          class="text-body-2 text-medium-emphasis"
        >
          No node executions yet.
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

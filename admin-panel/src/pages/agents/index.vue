<script setup>
definePage({
  meta: { navActiveLink: 'agents' },
})

const userData = useCookie('userData')
const tenants = ref([])
const selectedTenant = ref(userData.value?.tenantId ?? null)

const loadTenants = async () => {
  tenants.value = await ssoApi('/api/v1/tenants')
  if (!selectedTenant.value && tenants.value.length)
    selectedTenant.value = tenants.value[0].id
}

const agentTypes = ['planner', 'architect', 'developer', 'reviewer', 'tester', 'documentation', 'classifier']

const skillsById = ref({})

const loadSkills = async () => {
  if (!selectedTenant.value)
    return

  const skills = await sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/skills`)

  skillsById.value = Object.fromEntries(skills.map(s => [s.id, s]))
}

const fetchSkills = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/skills`)

const crud = useCrud({
  list: () => selectedTenant.value ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/agents`) : Promise.resolve([]),
  create: payload => sdlcPlatformApi('/api/v1/agents', { method: 'POST', body: payload }),
  update: (id, payload) => sdlcPlatformApi(`/api/v1/agents/${id}`, { method: 'PUT', body: payload }),
  remove: id => sdlcPlatformApi(`/api/v1/agents/${id}`, { method: 'DELETE' }),
})

watch(selectedTenant, async () => {
  await loadSkills()
  await crud.fetchAll()
})

const headers = [
  { title: 'Name', key: 'name' },
  { title: 'Type', key: 'agent_type' },
  { title: 'Provider', key: 'provider' },
  { title: 'Model', key: 'model' },
  { title: 'Skills', key: 'skill_ids' },
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
  name: '',
  agentType: 'developer',
  systemPrompt: '',
  provider: '',
  model: '',
  temperature: null,
  isActive: true,
  skillIds: [],
})

const form = ref(emptyForm())

const openCreate = () => {
  editingId.value = null
  form.value = emptyForm()
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = agent => {
  editingId.value = agent.id
  form.value = {
    isGlobal: !agent.tenant_id,
    name: agent.name,
    agentType: agent.agent_type,
    systemPrompt: agent.system_prompt,
    provider: agent.provider,
    model: agent.model,
    temperature: agent.temperature,
    isActive: agent.is_active,
    skillIds: agent.skill_ids ?? [],
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
        name: form.value.name,
        system_prompt: form.value.systemPrompt,
        provider: form.value.provider,
        model: form.value.model,
        temperature: form.value.temperature,
        is_active: form.value.isActive,
        skill_ids: form.value.skillIds,
      })
    }
    else {
      await crud.create({
        tenant_id: form.value.isGlobal ? null : selectedTenant.value,
        name: form.value.name,
        agent_type: form.value.agentType,
        system_prompt: form.value.systemPrompt,
        provider: form.value.provider,
        model: form.value.model,
        temperature: form.value.temperature,
        skill_ids: form.value.skillIds,
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

// ===== Execute =====
const executeDialogOpen = ref(false)
const executeAgent = ref(null)
const executeInput = ref('')
const executeError = ref('')
const executeLoading = ref(false)
const executeResult = ref(null)

const openExecute = agent => {
  executeAgent.value = agent
  executeInput.value = ''
  executeError.value = ''
  executeResult.value = null
  executeDialogOpen.value = true
}

const runExecute = async () => {
  if (!executeInput.value.trim())
    return

  executeLoading.value = true
  executeError.value = ''
  executeResult.value = null
  try {
    executeResult.value = await sdlcPlatformApi(`/api/v1/agents/${executeAgent.value.id}/execute`, {
      method: 'POST',
      body: { tenant_id: selectedTenant.value, input: executeInput.value },
    })
  }
  catch (err) {
    executeError.value = err.message || 'Execution failed'
  }
  finally {
    executeLoading.value = false
  }
}

// ===== Execution history =====
const historyDialogOpen = ref(false)
const historyAgent = ref(null)
const historyItems = ref([])
const historyLoading = ref(false)
const historyError = ref('')

const openHistory = async agent => {
  historyAgent.value = agent
  historyDialogOpen.value = true
  historyLoading.value = true
  historyError.value = ''
  try {
    const executions = await sdlcPlatformApi(`/api/v1/agents/${agent.id}/executions`)

    historyItems.value = executions.map(exec => ({
      title: exec.input.length > 60 ? `${exec.input.slice(0, 60)}…` : exec.input,
      status: exec.status,
      output: exec.output,
      error: exec.error,
      timestamp: exec.completed_at ?? exec.started_at ?? exec.created_at,
    }))
  }
  catch (err) {
    historyError.value = err.message || 'Failed to load execution history'
  }
  finally {
    historyLoading.value = false
  }
}

onMounted(async () => {
  await loadTenants()
  await loadSkills()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Agents">
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
          variant="tonal"
          prepend-icon="tabler-list-details"
          :to="{ name: 'agents-skills' }"
        >
          Manage Skills
        </VBtn>
        <VBtn
          prepend-icon="tabler-plus"
          :disabled="!selectedTenant"
          @click="openCreate"
        >
          New Agent
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
      <template #item.agent_type="{ item }">
        <VChip
          size="small"
          label
          class="text-capitalize"
        >
          {{ item.agent_type }}
        </VChip>
      </template>
      <template #item.skill_ids="{ item }">
        <span
          v-if="!item.skill_ids?.length"
          class="text-medium-emphasis"
        >—</span>
        <div
          v-else
          class="d-flex flex-wrap gap-1"
        >
          <VChip
            v-for="skillId in item.skill_ids"
            :key="skillId"
            size="small"
            label
          >
            {{ skillsById[skillId]?.name ?? skillId.slice(0, 8) }}
          </VChip>
        </div>
      </template>
      <template #item.tenant_id="{ item }">
        {{ item.tenant_id ? 'Tenant' : 'Global' }}
      </template>
      <template #item.is_active="{ item }">
        <StatusChip :status="item.is_active" />
      </template>
      <template #item.actions="{ item }">
        <IconBtn @click="openExecute(item)">
          <VIcon icon="tabler-player-play" />
        </IconBtn>
        <IconBtn @click="openHistory(item)">
          <VIcon icon="tabler-history" />
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
    max-width="640"
  >
    <VCard :title="editingId ? 'Edit Agent' : 'New Agent'">
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
              cols="12"
              md="6"
            >
              <AppSelect
                v-model="form.agentType"
                :items="agentTypes"
                label="Agent Type"
                :disabled="!!editingId"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.provider"
                label="Provider"
                placeholder="anthropic"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.model"
                label="Model"
                placeholder="claude-sonnet-5"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
            >
              <AppTextField
                v-model.number="form.temperature"
                label="Temperature"
                type="number"
                step="0.1"
              />
            </VCol>
            <VCol
              cols="12"
              md="6"
              class="d-flex align-center"
            >
              <VSwitch
                v-if="!editingId"
                v-model="form.isGlobal"
                label="Global agent (not tenant-scoped)"
              />
              <VSwitch
                v-else
                v-model="form.isActive"
                label="Active"
              />
            </VCol>
            <VCol cols="12">
              <AppTextarea
                v-model="form.systemPrompt"
                label="System Prompt"
                rows="4"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol cols="12">
              <RemoteSelect
                v-model="form.skillIds"
                label="Skills"
                multiple
                :fetch-options="fetchSkills"
                :item-title="s => `${s.name} (${s.description})`"
                empty-text="No skills for this tenant yet — create one via Manage Skills."
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
    v-model="executeDialogOpen"
    max-width="640"
  >
    <VCard :title="`Execute — ${executeAgent?.name}`">
      <VCardText>
        <VAlert
          v-if="executeError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ executeError }}
        </VAlert>
        <AppTextarea
          v-model="executeInput"
          label="Input"
          rows="4"
        />
        <div
          v-if="executeResult"
          class="mt-4"
        >
          <div class="d-flex align-center gap-2 mb-2">
            <span class="text-body-2 font-weight-medium">Result:</span>
            <StatusChip :status="executeResult.status" />
          </div>
          <pre
            v-if="executeResult.output"
            class="text-body-2 bg-var-theme-background pa-2 rounded"
            style="white-space: pre-wrap; word-break: break-word;"
          >{{ executeResult.output }}</pre>
          <VAlert
            v-if="executeResult.error"
            type="error"
            variant="tonal"
            density="compact"
            class="mt-2"
          >
            {{ executeResult.error }}
          </VAlert>
        </div>
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="executeDialogOpen = false"
        >
          Close
        </VBtn>
        <VBtn
          :loading="executeLoading"
          :disabled="!executeInput.trim()"
          @click="runExecute"
        >
          Run
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>

  <VDialog
    v-model="historyDialogOpen"
    max-width="720"
  >
    <VCard :title="`Execution History — ${historyAgent?.name}`">
      <VCardText>
        <VAlert
          v-if="historyError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ historyError }}
        </VAlert>
        <VProgressLinear
          v-if="historyLoading"
          indeterminate
          class="mb-4"
        />
        <p
          v-else-if="!historyItems.length"
          class="text-body-2 text-medium-emphasis"
        >
          No executions yet.
        </p>
        <StepTimeline
          v-else
          :items="historyItems"
        />
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="historyDialogOpen = false"
        >
          Close
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>

  <ConfirmDialog
    v-model="confirmOpen"
    title="Delete agent?"
    text="This cannot be undone."
    @confirm="doDelete"
  />
</template>

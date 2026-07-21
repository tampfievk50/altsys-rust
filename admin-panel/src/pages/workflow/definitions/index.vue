<script setup>
definePage({
  meta: { navActiveLink: 'workflow-definitions' },
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
  list: () => selectedTenant.value ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/workflow-definitions`) : Promise.resolve([]),
  create: payload => sdlcPlatformApi('/api/v1/workflow-definitions', { method: 'POST', body: payload }),
  update: (id, payload) => sdlcPlatformApi(`/api/v1/workflow-definitions/${id}`, { method: 'PUT', body: payload }),
  remove: id => sdlcPlatformApi(`/api/v1/workflow-definitions/${id}`, { method: 'DELETE' }),
})

watch(selectedTenant, crud.fetchAll)

const headers = [
  { title: 'Key', key: 'key' },
  { title: 'Name', key: 'name' },
  { title: 'Version', key: 'version' },
  { title: 'Status', key: 'is_active' },
  { title: 'Updated', key: 'updated_at' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()
const refDefinition = ref()

const emptyForm = () => ({ key: '', name: '', description: '', definition: '{\n  "nodes": []\n}', isActive: true })
const form = ref(emptyForm())

const openCreate = () => {
  editingId.value = null
  form.value = emptyForm()
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = def => {
  editingId.value = def.id
  form.value = {
    key: def.key,
    name: def.name,
    description: def.description ?? '',
    definition: def.definition,
    isActive: def.is_active,
  }
  formError.value = ''
  dialogOpen.value = true
}

const submit = async () => {
  const { valid } = await refForm.value.validate()
  if (!valid || (!editingId.value && !(refDefinition.value?.isValid() ?? true)))
    return

  formError.value = ''
  try {
    if (editingId.value) {
      await crud.update(editingId.value, {
        name: form.value.name,
        description: form.value.description || null,
        is_active: form.value.isActive,
      })
    }
    else {
      await crud.create({
        tenant_id: selectedTenant.value,
        key: form.value.key,
        name: form.value.name,
        description: form.value.description || null,
        definition: form.value.definition,
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

// ===== Versions =====
const versionsDialogOpen = ref(false)
const versionsKeyName = ref('')
const versionsItems = ref([])
const versionsLoading = ref(false)
const versionsError = ref('')

const openVersions = async def => {
  versionsKeyName.value = def.key
  versionsDialogOpen.value = true
  versionsLoading.value = true
  versionsError.value = ''
  try {
    versionsItems.value = await sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/workflow-definitions/${def.key}/versions`)
  }
  catch (err) {
    versionsError.value = err.message || 'Failed to load versions'
  }
  finally {
    versionsLoading.value = false
  }
}

onMounted(async () => {
  await loadTenants()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Workflow Definitions">
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
          New Definition
        </VBtn>
        <VBtn
          prepend-icon="tabler-drag-drop"
          variant="tonal"
          @click="router.push('/workflow/definitions/builder')"
        >
          Visual Builder
        </VBtn>
      </div>
    </template>

    <p class="text-body-2 text-medium-emphasis mx-4 mb-2">
      Listing shows the latest version of each definition key. Creating a definition with an existing key adds a new version rather than overwriting it.
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
      <template #item.version="{ item }">
        v{{ item.version }}
      </template>
      <template #item.is_active="{ item }">
        <StatusChip :status="item.is_active" />
      </template>
      <template #item.updated_at="{ item }">
        {{ new Date(item.updated_at).toLocaleString() }}
      </template>
      <template #item.actions="{ item }">
        <IconBtn @click="router.push(`/workflow/definitions/builder?loadKey=${item.key}`)">
          <VIcon icon="tabler-drag-drop" />
        </IconBtn>
        <IconBtn @click="openVersions(item)">
          <VIcon icon="tabler-versions" />
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
    <VCard :title="editingId ? 'Edit Workflow Definition' : 'New Workflow Definition'">
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
              v-if="!editingId"
              cols="12"
              md="6"
            >
              <AppTextField
                v-model="form.key"
                label="Key"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              cols="12"
              :md="editingId ? 12 : 6"
            >
              <AppTextField
                v-model="form.name"
                label="Name"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol cols="12">
              <AppTextarea
                v-model="form.description"
                label="Description"
                rows="2"
              />
            </VCol>
            <VCol
              v-if="!editingId"
              cols="12"
            >
              <JsonField
                ref="refDefinition"
                v-model="form.definition"
                label="Definition (JSON graph)"
                :rows="8"
                required
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
    v-model="versionsDialogOpen"
    max-width="640"
  >
    <VCard :title="`Versions — ${versionsKeyName}`">
      <VCardText>
        <VAlert
          v-if="versionsError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ versionsError }}
        </VAlert>
        <VProgressLinear
          v-if="versionsLoading"
          indeterminate
          class="mb-4"
        />
        <VTable
          v-else
          density="compact"
        >
          <thead>
            <tr>
              <th>Version</th>
              <th>Active</th>
              <th>Updated</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="v in versionsItems"
              :key="v.id"
            >
              <td>v{{ v.version }}</td>
              <td><StatusChip :status="v.is_active" /></td>
              <td>{{ new Date(v.updated_at).toLocaleString() }}</td>
            </tr>
          </tbody>
        </VTable>
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="versionsDialogOpen = false"
        >
          Close
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>

  <ConfirmDialog
    v-model="confirmOpen"
    title="Delete this version?"
    text="This cannot be undone."
    @confirm="doDelete"
  />
</template>

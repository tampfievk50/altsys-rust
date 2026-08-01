<script setup>
definePage({
  meta: { navActiveLink: 'sso-roles' },
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
  list: () => selectedTenant.value ? ssoApi(`/api/v1/tenants/${selectedTenant.value}/roles`) : Promise.resolve([]),
  create: payload => ssoApi('/api/v1/roles', { method: 'POST', body: payload }),
  update: (id, payload) => ssoApi(`/api/v1/roles/${id}`, { method: 'PUT', body: payload }),
  remove: id => ssoApi(`/api/v1/roles/${id}`, { method: 'DELETE' }),
})

watch(selectedTenant, crud.fetchAll)

const headers = [
  { title: 'Name', key: 'name' },
  { title: 'Description', key: 'description' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()
const form = ref({ name: '', description: '' })

const openCreate = () => {
  editingId.value = null
  form.value = { name: '', description: '' }
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = role => {
  editingId.value = role.id
  form.value = { name: role.name, description: role.description ?? '' }
  formError.value = ''
  dialogOpen.value = true
}

const submit = async () => {
  const { valid } = await refForm.value.validate()
  if (!valid)
    return

  formError.value = ''
  try {
    if (editingId.value)
      await crud.update(editingId.value, { name: form.value.name, description: form.value.description || null })
    else
      await crud.create({ tenant_id: selectedTenant.value, name: form.value.name, description: form.value.description || null })

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

// SSO has no "list permissions currently assigned to a role" endpoint, only
// assign/unassign — so this is a blind assign/remove action, not a live
// checklist of current state.
const permDialogOpen = ref(false)
const permDialogRole = ref(null)
const permAction = ref('assign')
const permId = ref(null)
const permError = ref('')
const permSuccess = ref('')

const openManagePermissions = role => {
  permDialogRole.value = role
  permAction.value = 'assign'
  permId.value = null
  permError.value = ''
  permSuccess.value = ''
  permDialogOpen.value = true
}

const submitPermission = async () => {
  if (!permId.value)
    return

  permError.value = ''
  permSuccess.value = ''
  try {
    if (permAction.value === 'assign')
      await ssoApi(`/api/v1/roles/${permDialogRole.value.id}/permissions/${permId.value}`, { method: 'POST' })
    else
      await ssoApi(`/api/v1/roles/${permDialogRole.value.id}/permissions/${permId.value}`, { method: 'DELETE' })

    permSuccess.value = permAction.value === 'assign' ? 'Permission assigned.' : 'Permission removed.'
  }
  catch (err) {
    permError.value = err.message || 'Action failed'
  }
}

onMounted(async () => {
  await loadTenants()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Roles">
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
          New Role
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
      <template #item.actions="{ item }">
        <IconBtn @click="openManagePermissions(item)">
          <VIcon icon="tabler-key" />
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
    max-width="500"
  >
    <VCard :title="editingId ? 'Edit Role' : 'New Role'">
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
            <VCol cols="12">
              <AppTextField
                v-model="form.name"
                label="Name"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol cols="12">
              <AppTextField
                v-model="form.description"
                label="Description"
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
    v-model="permDialogOpen"
    max-width="500"
  >
    <VCard :title="`Manage Permissions — ${permDialogRole?.name}`">
      <VCardText>
        <p class="text-body-2 text-medium-emphasis mb-4">
          The SSO service doesn't expose a "current permissions" query, only assign/remove — pick a permission and an action.
        </p>
        <VAlert
          v-if="permError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ permError }}
        </VAlert>
        <VAlert
          v-if="permSuccess"
          type="success"
          variant="tonal"
          class="mb-4"
        >
          {{ permSuccess }}
        </VAlert>
        <VRow>
          <VCol cols="12">
            <VRadioGroup
              v-model="permAction"
              inline
            >
              <VRadio
                label="Assign"
                value="assign"
              />
              <VRadio
                label="Remove"
                value="remove"
              />
            </VRadioGroup>
          </VCol>
          <VCol cols="12">
            <RemoteSelect
              v-model="permId"
              label="Permission"
              :fetch-options="() => ssoApi('/api/v1/permissions')"
            />
          </VCol>
        </VRow>
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="permDialogOpen = false"
        >
          Close
        </VBtn>
        <VBtn
          :disabled="!permId"
          @click="submitPermission"
        >
          Apply
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>

  <ConfirmDialog
    v-model="confirmOpen"
    title="Delete role?"
    text="This cannot be undone."
    @confirm="doDelete"
  />
</template>

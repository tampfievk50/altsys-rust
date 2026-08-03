<script setup>
definePage({
  meta: { navActiveLink: 'sso-roles' },
})

const route = useRoute()

// Arriving from a role link (e.g. the Roles column on the Users page)
// pre-selects that role's tenant and calls out the row so it's easy to spot.
const highlightedRoleId = ref(route.query.role ?? null)

const userData = useCookie('userData')
const tenants = ref([])
const selectedTenant = ref(route.query.tenant ?? userData.value?.tenantId ?? null)

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

const permDialogOpen = ref(false)
const permDialogRole = ref(null)
const rolePermissions = ref([])
const rolePermissionsLoading = ref(false)
const permId = ref(null)
const permError = ref('')
const permAssigning = ref(false)
const assigningAll = ref(false)
const removingPermissionId = ref(null)

const loadRolePermissions = async () => {
  rolePermissionsLoading.value = true
  permError.value = ''
  try {
    rolePermissions.value = await ssoApi(`/api/v1/roles/${permDialogRole.value.id}/permissions`)
  }
  catch (err) {
    permError.value = err.message || 'Failed to load permissions'
  }
  finally {
    rolePermissionsLoading.value = false
  }
}

const openManagePermissions = async role => {
  permDialogRole.value = role
  permId.value = null
  permError.value = ''
  rolePermissions.value = []
  permDialogOpen.value = true
  await loadRolePermissions()
}

const assignPermission = async () => {
  if (!permId.value)
    return

  permError.value = ''
  permAssigning.value = true
  try {
    await ssoApi(`/api/v1/roles/${permDialogRole.value.id}/permissions/${permId.value}`, { method: 'POST' })
    permId.value = null
    await loadRolePermissions()
  }
  catch (err) {
    permError.value = err.message || 'Failed to assign permission'
  }
  finally {
    permAssigning.value = false
  }
}

const assignAllPermissions = async () => {
  permError.value = ''
  assigningAll.value = true
  try {
    await ssoApi(`/api/v1/roles/${permDialogRole.value.id}/permissions/all`, { method: 'POST' })
    permId.value = null
    await loadRolePermissions()
  }
  catch (err) {
    permError.value = err.message || 'Failed to assign all permissions'
  }
  finally {
    assigningAll.value = false
  }
}

const removePermission = async permission => {
  permError.value = ''
  removingPermissionId.value = permission.id
  try {
    await ssoApi(`/api/v1/roles/${permDialogRole.value.id}/permissions/${permission.id}`, { method: 'DELETE' })
    await loadRolePermissions()
  }
  catch (err) {
    permError.value = err.message || 'Failed to remove permission'
  }
  finally {
    removingPermissionId.value = null
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
      <template #item.name="{ item }">
        <span :class="{ 'font-weight-bold text-primary': item.id === highlightedRoleId }">
          {{ item.name }}
        </span>
      </template>
      <template #item.actions="{ item }">
        <IconBtn
          aria-label="Manage Permissions"
          @click="openManagePermissions(item)"
        >
          <VIcon icon="tabler-key" />
          <VTooltip
            activator="parent"
            open-delay="500"
          >
            Manage Permissions
          </VTooltip>
        </IconBtn>
        <IconBtn
          aria-label="Edit"
          @click="openEdit(item)"
        >
          <VIcon icon="tabler-pencil" />
          <VTooltip
            activator="parent"
            open-delay="500"
          >
            Edit
          </VTooltip>
        </IconBtn>
        <IconBtn
          aria-label="Delete"
          @click="askDelete(item.id)"
        >
          <VIcon icon="tabler-trash" />
          <VTooltip
            activator="parent"
            open-delay="500"
          >
            Delete
          </VTooltip>
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
    max-width="600"
  >
    <VCard :title="`Manage Permissions — ${permDialogRole?.name}`">
      <VCardText>
        <VAlert
          v-if="permError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ permError }}
        </VAlert>

        <div class="d-flex justify-space-between align-center mb-2">
          <p class="text-subtitle-2 mb-0">
            Assigned permissions
          </p>
          <VBtn
            size="small"
            variant="tonal"
            prepend-icon="tabler-checks"
            :loading="assigningAll"
            @click="assignAllPermissions"
          >
            Assign All
          </VBtn>
        </div>
        <VProgressLinear
          v-if="rolePermissionsLoading"
          indeterminate
          class="mb-4"
        />
        <p
          v-else-if="!rolePermissions.length"
          class="text-body-2 text-medium-emphasis mb-4"
        >
          No permissions assigned yet.
        </p>
        <VList
          v-else
          density="compact"
          max-height="260"
          class="mb-4 border rounded overflow-y-auto"
        >
          <VListItem
            v-for="permission in rolePermissions"
            :key="permission.id"
          >
            <VListItemTitle>{{ permission.name }}</VListItemTitle>
            <VListItemSubtitle>{{ permission.action }} <code>{{ permission.resource }}</code></VListItemSubtitle>
            <template #append>
              <IconBtn
                aria-label="Remove"
                :loading="removingPermissionId === permission.id"
                @click="removePermission(permission)"
              >
                <VIcon icon="tabler-trash" />
                <VTooltip
                  activator="parent"
                  open-delay="500"
                >
                  Remove
                </VTooltip>
              </IconBtn>
            </template>
          </VListItem>
        </VList>

        <VDivider class="mb-4" />

        <p class="text-subtitle-2 mb-2">
          Assign a permission
        </p>
        <VRow>
          <VCol cols="12">
            <RemoteSelect
              v-if="permDialogOpen && !rolePermissionsLoading"
              v-model="permId"
              label="Permission"
              :fetch-options="async () => {
                const all = await ssoApi('/api/v1/permissions')
                const assignedIds = new Set(rolePermissions.map(p => p.id))
                return all.filter(p => !assignedIds.has(p.id))
              }"
              :item-title="p => `${p.name} — ${p.action} ${p.resource}`"
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
          :loading="permAssigning"
          @click="assignPermission"
        >
          Assign
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

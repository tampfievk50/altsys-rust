<script setup>
definePage({
  meta: { navActiveLink: 'sso-users' },
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
  list: () => selectedTenant.value ? ssoApi(`/api/v1/tenants/${selectedTenant.value}/users`) : Promise.resolve([]),
  create: payload => ssoApi('/api/v1/users', { method: 'POST', body: payload }),
  update: (id, payload) => ssoApi(`/api/v1/users/${id}`, { method: 'PUT', body: payload }),
  remove: id => ssoApi(`/api/v1/users/${id}`, { method: 'DELETE' }),
})

// Keyed by user id -> that user's currently assigned roles.
const userRoles = ref({})

const loadUserRoles = async () => {
  const entries = await Promise.all(
    crud.items.value.map(async user => {
      try {
        return [user.id, await ssoApi(`/api/v1/users/${user.id}/roles`)]
      }
      catch {
        return [user.id, []]
      }
    }),
  )
  userRoles.value = Object.fromEntries(entries)
}

const refreshUserRoles = async userId => {
  userRoles.value = { ...userRoles.value, [userId]: await ssoApi(`/api/v1/users/${userId}/roles`) }
}

const reload = async () => {
  await crud.fetchAll()
  await loadUserRoles()
}

watch(selectedTenant, reload)

const headers = [
  { title: 'Username', key: 'username' },
  { title: 'Email', key: 'email' },
  { title: 'Full name', key: 'full_name' },
  { title: 'Roles', key: 'roles', sortable: false },
  { title: 'Status', key: 'is_active' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()
const form = ref({ username: '', email: '', password: '', fullName: '', isActive: true })

const openCreate = () => {
  editingId.value = null
  form.value = { username: '', email: '', password: '', fullName: '', isActive: true }
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = user => {
  editingId.value = user.id
  form.value = { username: user.username, email: user.email, password: '', fullName: user.full_name ?? '', isActive: user.is_active }
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
        email: form.value.email,
        password: form.value.password || null,
        full_name: form.value.fullName || null,
        is_active: form.value.isActive,
      })
    }
    else {
      await crud.create({
        tenant_id: selectedTenant.value,
        username: form.value.username,
        email: form.value.email,
        password: form.value.password,
        full_name: form.value.fullName || null,
      })
    }

    dialogOpen.value = false
    await loadUserRoles()
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

const doDelete = async () => {
  await crud.remove(pendingDeleteId.value)
  await loadUserRoles()
}

const roleDialogOpen = ref(false)
const roleDialogUser = ref(null)
const roleAction = ref('assign')
const roleId = ref(null)
const roleError = ref('')
const roleSuccess = ref('')

const openManageRoles = user => {
  roleDialogUser.value = user
  roleAction.value = 'assign'
  roleId.value = null
  roleError.value = ''
  roleSuccess.value = ''
  roleDialogOpen.value = true
}

const submitRole = async () => {
  if (!roleId.value)
    return

  roleError.value = ''
  roleSuccess.value = ''
  try {
    if (roleAction.value === 'assign')
      await ssoApi(`/api/v1/users/${roleDialogUser.value.id}/roles/${roleId.value}/${selectedTenant.value}`, { method: 'POST' })
    else
      await ssoApi(`/api/v1/users/${roleDialogUser.value.id}/roles/${roleId.value}`, { method: 'DELETE' })

    roleSuccess.value = roleAction.value === 'assign' ? 'Role assigned.' : 'Role removed.'
    await refreshUserRoles(roleDialogUser.value.id)
  }
  catch (err) {
    roleError.value = err.message || 'Action failed'
  }
}

onMounted(async () => {
  await loadTenants()
  await reload()
})
</script>

<template>
  <VCard title="Users">
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
          New User
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
      <template #item.roles="{ item }">
        <VChip
          v-for="role in userRoles[item.id] ?? []"
          :key="role.id"
          :to="{ path: '/sso/roles', query: { tenant: item.tenant_id, role: role.id } }"
          color="primary"
          size="small"
          class="me-1"
          link
        >
          {{ role.name }}
        </VChip>
        <span
          v-if="!(userRoles[item.id] ?? []).length"
          class="text-medium-emphasis"
        >—</span>
      </template>
      <template #item.is_active="{ item }">
        <StatusChip :status="item.is_active" />
      </template>
      <template #item.actions="{ item }">
        <IconBtn
          aria-label="Manage Roles"
          @click="openManageRoles(item)"
        >
          <VIcon icon="tabler-shield" />
          <VTooltip
            activator="parent"
            open-delay="500"
          >
            Manage Roles
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
    <VCard :title="editingId ? 'Edit User' : 'New User'">
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
            >
              <AppTextField
                v-model="form.username"
                label="Username"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol cols="12">
              <AppTextField
                v-model="form.email"
                label="Email"
                type="email"
                :rules="[requiredValidator, emailValidator]"
              />
            </VCol>
            <VCol cols="12">
              <AppTextField
                v-model="form.password"
                label="Password"
                type="password"
                :rules="editingId ? [] : [requiredValidator]"
                :placeholder="editingId ? 'Leave blank to keep current password' : undefined"
              />
            </VCol>
            <VCol cols="12">
              <AppTextField
                v-model="form.fullName"
                label="Full name"
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
    v-model="roleDialogOpen"
    max-width="500"
  >
    <VCard :title="`Manage Roles — ${roleDialogUser?.username}`">
      <VCardText>
        <p class="text-body-2 text-medium-emphasis mb-4">
          Current roles: <template v-if="(userRoles[roleDialogUser?.id] ?? []).length">{{ (userRoles[roleDialogUser?.id] ?? []).map(r => r.name).join(', ') }}</template><template v-else>none</template>. Pick a role and an action below.
        </p>
        <VAlert
          v-if="roleError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ roleError }}
        </VAlert>
        <VAlert
          v-if="roleSuccess"
          type="success"
          variant="tonal"
          class="mb-4"
        >
          {{ roleSuccess }}
        </VAlert>
        <VRow>
          <VCol cols="12">
            <VRadioGroup
              v-model="roleAction"
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
              v-model="roleId"
              label="Role"
              :fetch-options="() => ssoApi(`/api/v1/tenants/${selectedTenant}/roles`)"
            />
          </VCol>
        </VRow>
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="roleDialogOpen = false"
        >
          Close
        </VBtn>
        <VBtn
          :disabled="!roleId"
          @click="submitRole"
        >
          Apply
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>

  <ConfirmDialog
    v-model="confirmOpen"
    title="Delete user?"
    text="This cannot be undone."
    @confirm="doDelete"
  />
</template>

<script setup>
definePage({
  meta: { navActiveLink: 'sso-permissions' },
})

const crud = useCrud({
  list: () => ssoApi('/api/v1/permissions'),
  create: payload => ssoApi('/api/v1/permissions', { method: 'POST', body: payload }),
  update: (id, payload) => ssoApi(`/api/v1/permissions/${id}`, { method: 'PUT', body: payload }),
  remove: id => ssoApi(`/api/v1/permissions/${id}`, { method: 'DELETE' }),
})

const headers = [
  { title: 'Name', key: 'name' },
  { title: 'Action', key: 'action' },
  { title: 'Resource', key: 'resource' },
  { title: 'Description', key: 'description' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()
const form = ref({ name: '', action: '', resource: '', description: '' })

const openCreate = () => {
  editingId.value = null
  form.value = { name: '', action: '', resource: '', description: '' }
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = permission => {
  editingId.value = permission.id
  form.value = {
    name: permission.name,
    action: permission.action,
    resource: permission.resource,
    description: permission.description ?? '',
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
    if (editingId.value)
      await crud.update(editingId.value, { name: form.value.name, description: form.value.description || null })
    else
      await crud.create({
        name: form.value.name,
        action: form.value.action,
        resource: form.value.resource,
        description: form.value.description || null,
      })

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

// Refresh from API: previews the diff between the service's own OpenAPI
// route list and the current Permission rows (resource = path template,
// action = HTTP method), then lets the admin apply it — creating the
// missing ones. Orphaned rows (no matching route anymore) are surfaced but
// never auto-deleted, since a role may still reference one.
const refreshDialogOpen = ref(false)
const refreshLoading = ref(false)
const refreshApplying = ref(false)
const refreshError = ref('')
const refreshDiff = ref({ missing: [], orphaned: [] })

const openRefreshPreview = async () => {
  refreshDialogOpen.value = true
  refreshError.value = ''
  refreshLoading.value = true
  try {
    refreshDiff.value = await ssoApi('/api/v1/permissions/refresh')
  }
  catch (err) {
    refreshError.value = err.message || 'Failed to load preview'
  }
  finally {
    refreshLoading.value = false
  }
}

const applyRefresh = async () => {
  refreshError.value = ''
  refreshApplying.value = true
  try {
    await ssoApi('/api/v1/permissions/refresh', { method: 'POST' })
    refreshDialogOpen.value = false
    await crud.fetchAll()
  }
  catch (err) {
    refreshError.value = err.message || 'Refresh failed'
  }
  finally {
    refreshApplying.value = false
  }
}

onMounted(crud.fetchAll)
</script>

<template>
  <VCard title="Permissions">
    <template #append>
      <div class="d-flex gap-2">
        <VBtn
          variant="tonal"
          prepend-icon="tabler-refresh"
          @click="openRefreshPreview"
        >
          Refresh from API
        </VBtn>
        <VBtn
          prepend-icon="tabler-plus"
          @click="openCreate"
        >
          New Permission
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
    <VCard :title="editingId ? 'Edit Permission' : 'New Permission'">
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
            <VCol
              v-if="!editingId"
              cols="6"
            >
              <AppTextField
                v-model="form.action"
                label="Action"
                placeholder="read | write | *"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              v-if="!editingId"
              cols="6"
            >
              <AppTextField
                v-model="form.resource"
                label="Resource"
                placeholder="tenants | users | *"
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

  <ConfirmDialog
    v-model="confirmOpen"
    title="Delete permission?"
    text="This cannot be undone."
    @confirm="doDelete"
  />

  <VDialog
    v-model="refreshDialogOpen"
    max-width="700"
  >
    <VCard title="Refresh Permissions from API">
      <VCardText>
        <p class="text-body-2 text-medium-emphasis mb-4">
          Compares the service's current API routes against the Permission catalog. Applying only creates the missing ones below — nothing is deleted automatically.
        </p>

        <VAlert
          v-if="refreshError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ refreshError }}
        </VAlert>

        <VProgressLinear
          v-if="refreshLoading"
          indeterminate
          class="mb-4"
        />

        <template v-if="!refreshLoading">
          <p class="text-subtitle-2 mb-2">
            Missing ({{ refreshDiff.missing.length }})
          </p>
          <p
            v-if="!refreshDiff.missing.length"
            class="text-body-2 text-medium-emphasis mb-4"
          >
            Nothing to add — every route already has a matching permission.
          </p>
          <VTable
            v-else
            density="compact"
            class="mb-4"
          >
            <thead>
              <tr>
                <th>Name</th>
                <th>Action</th>
                <th>Resource</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="entry in refreshDiff.missing"
                :key="`${entry.action}-${entry.resource}`"
              >
                <td>{{ entry.name }}</td>
                <td>{{ entry.action }}</td>
                <td>
                  <code>{{ entry.resource }}</code>
                </td>
              </tr>
            </tbody>
          </VTable>

          <p class="text-subtitle-2 mb-2">
            Orphaned ({{ refreshDiff.orphaned.length }})
          </p>
          <p
            v-if="!refreshDiff.orphaned.length"
            class="text-body-2 text-medium-emphasis"
          >
            None — every permission still matches a current route.
          </p>
          <VTable
            v-else
            density="compact"
          >
            <thead>
              <tr>
                <th>Name</th>
                <th>Action</th>
                <th>Resource</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="permission in refreshDiff.orphaned"
                :key="permission.id"
              >
                <td>{{ permission.name }}</td>
                <td>{{ permission.action }}</td>
                <td>
                  <code>{{ permission.resource }}</code>
                </td>
              </tr>
            </tbody>
          </VTable>
        </template>
      </VCardText>
      <VCardText class="d-flex justify-end gap-2">
        <VBtn
          color="secondary"
          variant="tonal"
          @click="refreshDialogOpen = false"
        >
          Close
        </VBtn>
        <VBtn
          :disabled="refreshLoading || !refreshDiff.missing.length"
          :loading="refreshApplying"
          @click="applyRefresh"
        >
          Create {{ refreshDiff.missing.length }} missing
        </VBtn>
      </VCardText>
    </VCard>
  </VDialog>
</template>

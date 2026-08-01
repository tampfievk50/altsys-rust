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

onMounted(crud.fetchAll)
</script>

<template>
  <VCard title="Permissions">
    <template #append>
      <VBtn
        prepend-icon="tabler-plus"
        @click="openCreate"
      >
        New Permission
      </VBtn>
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
</template>

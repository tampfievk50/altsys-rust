<script setup>
definePage({
  meta: { navActiveLink: 'sso-tenants' },
})

const crud = useCrud({
  list: () => ssoApi('/api/v1/tenants'),
  create: payload => ssoApi('/api/v1/tenants', { method: 'POST', body: payload }),
  update: (id, payload) => ssoApi(`/api/v1/tenants/${id}`, { method: 'PUT', body: payload }),
  remove: id => ssoApi(`/api/v1/tenants/${id}`, { method: 'DELETE' }),
})

const headers = [
  { title: 'Name', key: 'name' },
  { title: 'Slug', key: 'slug' },
  { title: 'Status', key: 'is_active' },
  { title: 'Created', key: 'created_at' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()

const form = ref({ name: '', slug: '', is_active: true })

const openCreate = () => {
  editingId.value = null
  form.value = { name: '', slug: '', is_active: true }
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = tenant => {
  editingId.value = tenant.id
  form.value = { name: tenant.name, slug: tenant.slug, is_active: tenant.is_active }
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
      await crud.update(editingId.value, { name: form.value.name, is_active: form.value.is_active })
    else
      await crud.create({ name: form.value.name, slug: form.value.slug })

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
  <VCard title="Tenants">
    <template #append>
      <VBtn
        prepend-icon="tabler-plus"
        @click="openCreate"
      >
        New Tenant
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
      <template #item.is_active="{ item }">
        <StatusChip :status="item.is_active" />
      </template>
      <template #item.created_at="{ item }">
        {{ new Date(item.created_at).toLocaleString() }}
      </template>
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
    <VCard :title="editingId ? 'Edit Tenant' : 'New Tenant'">
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
              cols="12"
            >
              <AppTextField
                v-model="form.slug"
                label="Slug"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol
              v-if="editingId"
              cols="12"
            >
              <VSwitch
                v-model="form.is_active"
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

  <ConfirmDialog
    v-model="confirmOpen"
    title="Delete tenant?"
    text="This cannot be undone."
    @confirm="doDelete"
  />
</template>

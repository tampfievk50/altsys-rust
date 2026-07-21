<script setup>
definePage({
  meta: { navActiveLink: 'agents-skills' },
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
  list: () => selectedTenant.value ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/skills`) : Promise.resolve([]),
  create: payload => sdlcPlatformApi('/api/v1/skills', { method: 'POST', body: payload }),
  update: (id, payload) => sdlcPlatformApi(`/api/v1/skills/${id}`, { method: 'PUT', body: payload }),
  remove: id => sdlcPlatformApi(`/api/v1/skills/${id}`, { method: 'DELETE' }),
})

watch(selectedTenant, crud.fetchAll)

const headers = [
  { title: 'Name', key: 'name' },
  { title: 'Description', key: 'description' },
  { title: 'Scope', key: 'tenant_id' },
  { title: 'Status', key: 'is_active' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const dialogOpen = ref(false)
const editingId = ref(null)
const formError = ref('')
const refForm = ref()

const emptyForm = () => ({ isGlobal: false, name: '', description: '', content: '', isActive: true })
const form = ref(emptyForm())

const openCreate = () => {
  editingId.value = null
  form.value = emptyForm()
  formError.value = ''
  dialogOpen.value = true
}

const openEdit = skill => {
  editingId.value = skill.id
  form.value = {
    isGlobal: !skill.tenant_id,
    name: skill.name,
    description: skill.description,
    content: skill.content,
    isActive: skill.is_active,
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
        description: form.value.description,
        content: form.value.content,
        is_active: form.value.isActive,
      })
    }
    else {
      await crud.create({
        tenant_id: form.value.isGlobal ? null : selectedTenant.value,
        name: form.value.name,
        description: form.value.description,
        content: form.value.content,
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

onMounted(async () => {
  await loadTenants()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Skills">
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
          New Skill
        </VBtn>
      </div>
    </template>

    <p class="text-body-2 text-medium-emphasis mx-4 mb-2">
      A skill's content is folded into an agent's system prompt at execution time when attached to it. Attach skills to an agent from the Agents page.
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
      <template #item.description="{ item }">
        <span class="text-body-2">{{ item.description.length > 80 ? `${item.description.slice(0, 80)}…` : item.description }}</span>
      </template>
      <template #item.tenant_id="{ item }">
        {{ item.tenant_id ? 'Tenant' : 'Global' }}
      </template>
      <template #item.is_active="{ item }">
        <StatusChip :status="item.is_active" />
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
    max-width="640"
  >
    <VCard :title="editingId ? 'Edit Skill' : 'New Skill'">
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
                placeholder="Rust Idioms"
                :rules="[requiredValidator]"
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
                label="Global skill (not tenant-scoped)"
              />
              <VSwitch
                v-else
                v-model="form.isActive"
                label="Active"
              />
            </VCol>
            <VCol cols="12">
              <AppTextarea
                v-model="form.description"
                label="Description"
                placeholder="Tells an agent when this skill applies, e.g. 'Use when writing or reviewing Rust code.'"
                rows="2"
                :rules="[requiredValidator]"
              />
            </VCol>
            <VCol cols="12">
              <AppTextarea
                v-model="form.content"
                label="Content"
                placeholder="The instructions folded into the agent's system prompt when this skill is attached."
                rows="8"
                :rules="[requiredValidator]"
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
    title="Delete skill?"
    text="Any agent it's attached to will stop using it."
    @confirm="doDelete"
  />
</template>

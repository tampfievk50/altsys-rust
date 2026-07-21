<script setup>
definePage({
  meta: { navActiveLink: 'events' },
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
  list: () => selectedTenant.value ? sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/events`) : Promise.resolve([]),
})

watch(selectedTenant, crud.fetchAll)

const sortedEvents = computed(() =>
  [...crud.items.value].sort((a, b) => new Date(b.received_at) - new Date(a.received_at)),
)

const headers = [
  { title: 'Type', key: 'event_type' },
  { title: 'Payload', key: 'payload' },
  { title: 'Received', key: 'received_at' },
  { title: 'Actions', key: 'actions', sortable: false, align: 'end' },
]

const parsePayload = raw => {
  try {
    return JSON.parse(raw)
  }
  catch {
    return null
  }
}

const payloadPreview = raw => {
  const parsed = parsePayload(raw)

  return parsed ? JSON.stringify(parsed) : raw
}

// ===== Firings detail =====
const detailDialogOpen = ref(false)
const detailEvent = ref(null)
const detailFirings = ref([])
const detailLoading = ref(false)
const detailError = ref('')

const loadFirings = async event => {
  detailEvent.value = event
  detailDialogOpen.value = true
  detailLoading.value = true
  detailError.value = ''
  try {
    detailFirings.value = await sdlcPlatformApi(`/api/v1/events/${event.id}/firings`)
  }
  catch (err) {
    detailError.value = err.message || 'Failed to load rule firings'
  }
  finally {
    detailLoading.value = false
  }
}

const detailPayload = computed(() => {
  if (!detailEvent.value)
    return ''

  const parsed = parsePayload(detailEvent.value.payload)

  return parsed ? JSON.stringify(parsed, null, 2) : detailEvent.value.payload
})

const timelineItems = computed(() => detailFirings.value.map(f => ({
  title: `Rule ${f.rule_id.slice(0, 8)} — ${f.matched ? 'matched' : 'no match'}`,
  status: f.status,
  output: f.action_result,
  error: f.error,
  timestamp: f.created_at,
})))

onMounted(async () => {
  await loadTenants()
  await crud.fetchAll()
})
</script>

<template>
  <VCard title="Events">
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
        <IconBtn @click="crud.fetchAll">
          <VIcon icon="tabler-refresh" />
        </IconBtn>
      </div>
    </template>

    <p class="text-body-2 text-medium-emphasis mx-4 mb-2">
      Every event ingested via a Jira webhook, the Jira polling fallback, or a direct call to the Events API — newest first. Open one to see which automation rules evaluated it.
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
      :items="sortedEvents"
      :loading="crud.loading.value"
      item-value="id"
    >
      <template #item.payload="{ item }">
        <span
          class="text-body-2 d-inline-block text-truncate"
          style="max-inline-size: 420px; font-family: monospace;"
        >{{ payloadPreview(item.payload) }}</span>
      </template>
      <template #item.received_at="{ item }">
        {{ new Date(item.received_at).toLocaleString() }}
      </template>
      <template #item.actions="{ item }">
        <IconBtn @click="loadFirings(item)">
          <VIcon icon="tabler-list-details" />
        </IconBtn>
      </template>
    </VDataTable>
  </VCard>

  <VDialog
    v-model="detailDialogOpen"
    max-width="720"
  >
    <VCard :title="`Event — ${detailEvent?.event_type}`">
      <VCardText>
        <VAlert
          v-if="detailError"
          type="error"
          variant="tonal"
          class="mb-4"
        >
          {{ detailError }}
        </VAlert>

        <h6 class="text-h6 mb-2">
          Payload
        </h6>
        <pre
          class="text-body-2 bg-var-theme-background pa-2 rounded mb-4"
          style="white-space: pre-wrap; word-break: break-word;"
        >{{ detailPayload }}</pre>

        <h6 class="text-h6 mb-2">
          Automation rules evaluated
        </h6>
        <VProgressLinear
          v-if="detailLoading"
          indeterminate
          class="mb-4"
        />
        <p
          v-else-if="!timelineItems.length"
          class="text-body-2 text-medium-emphasis"
        >
          No active rule matched this event's type when it was ingested.
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

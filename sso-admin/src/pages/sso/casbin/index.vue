<script setup>
definePage({
  meta: { navActiveLink: 'sso-casbin' },
})

// Assigning permissions/roles via the Roles and Users pages keeps these rows
// in sync going forward (the API writes through to Casbin on every
// assign/remove). "Sync from assignments" is for backfilling or repairing
// drift: it rebuilds every p/g row from the current role_permissions/
// user_roles tables across all tenants, discarding anything not backed by a
// live assignment.
const loading = ref(false)
const syncing = ref(false)
const error = ref('')
const syncMessage = ref('')
const policies = ref([])
const groupingPolicies = ref([])

const policyHeaders = [
  { title: 'Subject', key: 'sub' },
  { title: 'Object', key: 'obj' },
  { title: 'Action', key: 'act' },
]

const groupingHeaders = [
  { title: 'User', key: 'user' },
  { title: 'Role', key: 'role' },
]

const load = async () => {
  loading.value = true
  error.value = ''
  try {
    const data = await ssoApi('/api/v1/casbin/policies')

    policies.value = data.policies.map(([sub, obj, act]) => ({ sub, obj, act }))
    groupingPolicies.value = data.grouping_policies.map(([user, role]) => ({ user, role }))
  }
  catch (err) {
    error.value = err.message || 'Failed to load policies'
  }
  finally {
    loading.value = false
  }
}

const sync = async () => {
  syncing.value = true
  error.value = ''
  syncMessage.value = ''
  try {
    const result = await ssoApi('/api/v1/casbin/sync', { method: 'POST' })

    syncMessage.value = `Synced ${result.policies_synced} permission ${result.policies_synced === 1 ? 'rule' : 'rules'} and ${result.grouping_policies_synced} role ${result.grouping_policies_synced === 1 ? 'assignment' : 'assignments'}.`
    await load()
  }
  catch (err) {
    error.value = err.message || 'Sync failed'
  }
  finally {
    syncing.value = false
  }
}

onMounted(load)
</script>

<template>
  <VRow>
    <VCol cols="12">
      <VCard title="Casbin Policies (p)">
        <template #append>
          <div class="d-flex gap-2">
            <VBtn
              variant="tonal"
              prepend-icon="tabler-refresh"
              :loading="loading"
              @click="load"
            >
              Refresh
            </VBtn>
            <VBtn
              prepend-icon="tabler-transfer"
              :loading="syncing"
              @click="sync"
            >
              Sync from assignments
            </VBtn>
          </div>
        </template>

        <VAlert
          v-if="error"
          type="error"
          variant="tonal"
          class="mx-4"
        >
          {{ error }}
        </VAlert>

        <VAlert
          v-if="syncMessage"
          type="success"
          variant="tonal"
          class="mx-4"
        >
          {{ syncMessage }}
        </VAlert>

        <p class="text-body-2 text-medium-emphasis mx-4 mb-0">
          Raw permission rules as stored by the adapter. Managed indirectly via Roles → Manage Permissions, or rebuilt in bulk with "Sync from assignments".
        </p>

        <VDataTable
          :headers="policyHeaders"
          :items="policies"
          :loading="loading"
        />
      </VCard>
    </VCol>

    <VCol cols="12">
      <VCard title="Casbin Role Assignments (g)">
        <p class="text-body-2 text-medium-emphasis mx-4 mb-0">
          Raw user→role assignments as stored by the adapter. Managed indirectly via Users → Manage Roles, or rebuilt in bulk with "Sync from assignments" above.
        </p>

        <VDataTable
          :headers="groupingHeaders"
          :items="groupingPolicies"
          :loading="loading"
        />
      </VCard>
    </VCol>
  </VRow>
</template>

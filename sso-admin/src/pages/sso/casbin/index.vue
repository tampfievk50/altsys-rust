<script setup>
definePage({
  meta: { navActiveLink: 'sso-casbin' },
})

// Read-only: the SSO service has no endpoint to mutate raw casbin_rule rows
// directly, only to assign/remove roles and permissions (see Users/Roles
// pages), which is what actually writes these p/g rows under the hood.
const loading = ref(false)
const error = ref('')
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

onMounted(load)
</script>

<template>
  <VRow>
    <VCol cols="12">
      <VCard title="Casbin Policies (p)">
        <template #append>
          <VBtn
            variant="tonal"
            prepend-icon="tabler-refresh"
            @click="load"
          >
            Refresh
          </VBtn>
        </template>

        <VAlert
          v-if="error"
          type="error"
          variant="tonal"
          class="mx-4"
        >
          {{ error }}
        </VAlert>

        <p class="text-body-2 text-medium-emphasis mx-4 mb-0">
          Raw permission rules as stored by the adapter. Managed indirectly via Roles → Manage Permissions.
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
          Raw user→role assignments as stored by the adapter. Managed indirectly via Users → Manage Roles.
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

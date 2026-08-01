<script setup>
definePage({
  meta: {
    navActiveLink: 'dashboard',
  },
})

const userData = useCookie('userData')
</script>

<template>
  <VRow>
    <VCol cols="12">
      <VCard>
        <VCardText>
          <h4 class="text-h4 mb-1">
            Welcome, {{ userData?.username ?? 'admin' }} 👋
          </h4>
          <p class="mb-0">
            Use the sidebar to manage tenants, users, roles, permissions and Casbin authorization policies.
          </p>
        </VCardText>
      </VCard>
    </VCol>
  </VRow>
</template>

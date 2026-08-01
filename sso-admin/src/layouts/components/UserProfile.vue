<script setup>
const router = useRouter()

const userData = useCookie('userData')

const logout = async () => {
  try {
    const refreshToken = useCookie('refreshToken').value
    if (refreshToken)
      await ssoApi('/api/v1/auth/logout', { method: 'POST', body: { refresh_token: refreshToken } })
  }
  catch {
    // Best-effort: still clear local session even if the revoke call fails.
  }

  useCookie('accessToken').value = null
  useCookie('refreshToken').value = null
  userData.value = null

  await router.push('/login')
}
</script>

<template>
  <VBadge
    v-if="userData"
    dot
    bordered
    location="bottom right"
    offset-x="1"
    offset-y="2"
    color="success"
  >
    <VAvatar
      size="38"
      class="cursor-pointer"
      color="primary"
      variant="tonal"
    >
      <VIcon icon="tabler-user" />

      <!-- SECTION Menu -->
      <VMenu
        activator="parent"
        width="240"
        location="bottom end"
        offset="12px"
      >
        <VList>
          <VListItem>
            <div class="d-flex gap-2 align-center">
              <VListItemAction>
                <VBadge
                  dot
                  location="bottom right"
                  offset-x="3"
                  offset-y="3"
                  color="success"
                  bordered
                >
                  <VAvatar
                    color="primary"
                    variant="tonal"
                  >
                    <VIcon icon="tabler-user" />
                  </VAvatar>
                </VBadge>
              </VListItemAction>

              <div>
                <h6 class="text-h6 font-weight-medium">
                  {{ userData.username }}
                </h6>
                <VListItemSubtitle class="text-disabled">
                  {{ userData.tenantId }}
                </VListItemSubtitle>
              </div>
            </div>
          </VListItem>

          <VDivider class="my-2" />

          <div class="px-4 py-2">
            <VBtn
              block
              size="small"
              color="error"
              append-icon="tabler-logout"
              @click="logout"
            >
              Logout
            </VBtn>
          </div>
        </VList>
      </VMenu>
      <!-- !SECTION -->
    </VAvatar>
  </VBadge>
</template>

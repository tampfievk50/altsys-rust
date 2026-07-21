export const setupGuards = router => {
  // 👉 router.beforeEach
  // Docs: https://router.vuejs.org/guide/advanced/navigation-guards.html#global-before-guards
  router.beforeEach(to => {
    // Public routes (404, etc.) are visitable by anyone, logged in or not.
    if (to.meta.public)
      return

    const isLoggedIn = !!(useCookie('userData').value && useCookie('accessToken').value)

    // Login-only pages: bounce an already-logged-in user back to the app.
    if (to.meta.unauthenticatedOnly) {
      if (isLoggedIn)
        return '/'

      return undefined
    }

    // Everything else requires a session — v1 has no per-role gating beyond that.
    if (!isLoggedIn) {
      return {
        name: 'login',
        query: {
          ...to.query,
          to: to.fullPath !== '/' ? to.path : undefined,
        },
      }
    }
  })
}

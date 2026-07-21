// 👉 Redirects
export const redirects = [
  {
    path: '/',
    name: 'index',
    redirect: to => ({ name: 'dashboard', query: to.query }),
  },
]

export const routes = []

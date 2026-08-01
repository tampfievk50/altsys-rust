export default [
  {
    title: 'Dashboard',
    icon: { icon: 'tabler-smart-home' },
    to: 'dashboard',
  },
  {
    title: 'Identity & Access',
    icon: { icon: 'tabler-shield-lock' },
    children: [
      { title: 'Tenants', to: 'sso-tenants' },
      { title: 'Users', to: 'sso-users' },
      { title: 'Roles', to: 'sso-roles' },
      { title: 'Permissions', to: 'sso-permissions' },
      { title: 'Casbin Policies', to: 'sso-casbin' },
    ],
  },
]

// Each top-level entry below corresponds to one backend service. "Identity &
// Access" (sso), "Project Management", "Agents", "Workflow", "Autonomous
// SDLC", "Automation" and "Settings" have real pages wired up so far — the
// rest (knowledge) are added group-by-group as their milestones land (see the
// plan for the full 8-service roadmap). "Settings" is a UI-only grouping —
// Credentials/Models live in the Model Registry service and GitHub/Jira are
// Tool rows (tool_type "github"/"jira") from the Tool Registry service.
// "Automation" only has Events so far — Automation Rules has no page yet
// (create/manage rules via the API directly, e.g. through Swagger UI).
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
    ],
  },
  {
    title: 'Project Management',
    icon: { icon: 'tabler-folder-cog' },
    to: 'project-management',
  },
  {
    title: 'Agents',
    icon: { icon: 'tabler-robot' },
    children: [
      { title: 'Agents', to: 'agents' },
      { title: 'Skills', to: 'agents-skills' },
    ],
  },
  {
    title: 'Workflow',
    icon: { icon: 'tabler-git-branch' },
    children: [
      { title: 'Definitions', to: 'workflow-definitions' },
      { title: 'Executions', to: 'workflow-executions' },
    ],
  },
  {
    title: 'Autonomous SDLC',
    icon: { icon: 'tabler-rocket' },
    to: 'sdlc',
  },
  {
    title: 'Automation',
    icon: { icon: 'tabler-bolt' },
    children: [
      { title: 'Events', to: 'events' },
    ],
  },
  {
    title: 'Settings',
    icon: { icon: 'tabler-settings' },
    children: [
      { title: 'Credentials', to: 'settings-credentials' },
      { title: 'Models', to: 'settings-models' },
      { title: 'GitHub Repositories', to: 'settings-github' },
      { title: 'Jira Configuration', to: 'settings-jira' },
    ],
  },
]

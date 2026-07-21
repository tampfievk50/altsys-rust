<script setup>
import { VueFlow, useVueFlow } from '@vue-flow/core'
import { Background } from '@vue-flow/background'
import { Controls } from '@vue-flow/controls'
import '@vue-flow/core/dist/style.css'
import '@vue-flow/core/dist/theme-default.css'
import '@vue-flow/controls/dist/style.css'

definePage({
  meta: { navActiveLink: 'workflow-definitions' },
})

const route = useRoute()
const router = useRouter()

const userData = useCookie('userData')
const tenants = ref([])
const selectedTenant = ref(userData.value?.tenantId ?? null)

const loadTenants = async () => {
  tenants.value = await ssoApi('/api/v1/tenants')
  if (!selectedTenant.value && tenants.value.length)
    selectedTenant.value = tenants.value[0].id
}

const { addNodes, addEdges, removeNodes, removeEdges, screenToFlowCoordinate, fitView, onConnect, onNodeClick, onEdgeClick, onPaneClick } = useVueFlow()

const nodes = ref([])
const edges = ref([])
let nextSeq = 1

// ===== Palette =====
const palette = [
  { nodeType: 'start', executor: null, label: 'Start' },
  { nodeType: 'task', executor: 'agent', label: 'Task: Agent' },
  { nodeType: 'task', executor: 'tool', label: 'Task: Tool' },
  { nodeType: 'approval', executor: null, label: 'Approval' },
  { nodeType: 'end', executor: null, label: 'End' },
]

const newNodeData = entry => ({
  name: entry.label,
  nodeType: entry.nodeType,
  executor: entry.executor ?? 'noop',
  agentId: null,
  instruction: '',
  toolId: null,
  action: '',
  parameters: '{}',
  maxAttempts: 1,
  backoffSeconds: 0,
  join: false,
})

const addPaletteNode = (entry, position) => {
  const id = `node-${nextSeq++}`
  const fallback = { x: 80 + (nodes.value.length % 4) * 210, y: 60 + Math.floor(nodes.value.length / 4) * 150 }

  addNodes([{ id, type: 'wf', position: position ?? fallback, data: newNodeData(entry) }])
}

const onPaletteDragStart = (event, entry) => {
  event.dataTransfer.setData('application/json', JSON.stringify(entry))
  event.dataTransfer.effectAllowed = 'move'
}

const onCanvasDragOver = event => {
  event.preventDefault()
  event.dataTransfer.dropEffect = 'move'
}

const onCanvasDrop = event => {
  event.preventDefault()

  const raw = event.dataTransfer.getData('application/json')
  if (!raw)
    return

  const entry = JSON.parse(raw)
  const position = screenToFlowCoordinate({ x: event.clientX, y: event.clientY })

  addPaletteNode(entry, position)
}

// ===== Selection & config panel =====
const selectedNodeId = ref(null)
const selectedEdgeId = ref(null)

const selectedNode = computed(() => nodes.value.find(n => n.id === selectedNodeId.value) ?? null)
const selectedEdge = computed(() => edges.value.find(e => e.id === selectedEdgeId.value) ?? null)

onNodeClick(({ node }) => {
  selectedNodeId.value = node.id
  selectedEdgeId.value = null
})
onEdgeClick(({ edge }) => {
  selectedEdgeId.value = edge.id
  selectedNodeId.value = null
})
onPaneClick(() => {
  selectedNodeId.value = null
  selectedEdgeId.value = null
})
onConnect(params => {
  addEdges([{ ...params, data: { condition: '' } }])
})

const deleteSelectedNode = () => {
  if (!selectedNodeId.value)
    return

  removeNodes([selectedNodeId.value])
  selectedNodeId.value = null
}

const deleteSelectedEdge = () => {
  if (!selectedEdgeId.value)
    return

  removeEdges([selectedEdgeId.value])
  selectedEdgeId.value = null
}

// ===== Agent/Tool option sources for the config panel =====
const fetchAgents = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/agents`)
const fetchTools = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/tools`)

// ===== Load an existing definition as a starting point =====
const fetchDefinitions = () => sdlcPlatformApi(`/api/v1/tenants/${selectedTenant.value}/workflow-definitions`)
const loadDefinitionId = ref(null)

const loadFromDefinition = def => {
  const graph = JSON.parse(def.definition)

  nodes.value = graph.nodes.map((n, i) => ({
    id: n.id,
    type: 'wf',
    position: { x: 80 + (i % 4) * 210, y: 60 + Math.floor(i / 4) * 150 },
    data: {
      name: n.name,
      nodeType: n.node_type,
      executor: n.executor ?? 'noop',
      agentId: n.config?.agent_id ?? null,
      instruction: n.config?.instruction ?? '',
      toolId: n.config?.tool_id ?? null,
      action: n.config?.action ?? '',
      parameters: JSON.stringify(n.config?.parameters ?? {}),
      maxAttempts: n.retry_policy?.max_attempts ?? 1,
      backoffSeconds: n.retry_policy?.backoff_seconds ?? 0,
      join: n.join ?? false,
    },
  }))

  edges.value = graph.edges.map((e, i) => ({
    id: `edge-${i}-${e.from}-${e.to}`,
    source: e.from,
    target: e.to,
    data: { condition: e.condition ?? '' },
  }))

  meta.value = { key: def.key, name: def.name, description: def.description ?? '' }
  nextSeq = nodes.value.length + 1
  selectedNodeId.value = null
  selectedEdgeId.value = null
  nextTick(() => fitView())
}

watch(loadDefinitionId, async id => {
  if (!id)
    return

  const defs = await fetchDefinitions()
  const match = defs.find(d => d.id === id)
  if (match)
    loadFromDefinition(match)
})

// ===== Save =====
const meta = ref({ key: '', name: '', description: '' })
const saveError = ref('')
const saveSuccess = ref('')
const saveLoading = ref(false)

const toGraphJson = () => {
  const graphNodes = nodes.value.map(n => {
    const node = { id: n.id, name: n.data.name, node_type: n.data.nodeType }

    if (n.data.nodeType !== 'task')
      return node

    node.executor = n.data.executor
    if (n.data.executor === 'agent') {
      node.config = { agent_id: n.data.agentId, instruction: n.data.instruction || undefined }
    }
    else if (n.data.executor === 'tool') {
      let parameters = {}
      try {
        parameters = JSON.parse(n.data.parameters || '{}')
      }
      catch {
        parameters = {}
      }
      node.config = { tool_id: n.data.toolId, action: n.data.action, parameters }
    }
    if (n.data.maxAttempts > 1 || n.data.backoffSeconds > 0)
      node.retry_policy = { max_attempts: n.data.maxAttempts, backoff_seconds: n.data.backoffSeconds }
    if (n.data.join)
      node.join = true

    return node
  })

  const graphEdges = edges.value.map(e => {
    const edge = { from: e.source, to: e.target }
    if (e.data?.condition)
      edge.condition = e.data.condition

    return edge
  })

  return JSON.stringify({ nodes: graphNodes, edges: graphEdges })
}

const save = async () => {
  saveError.value = ''
  saveSuccess.value = ''
  if (!meta.value.key.trim() || !meta.value.name.trim()) {
    saveError.value = 'Key and name are required.'
    
    return
  }
  if (!nodes.value.length) {
    saveError.value = 'Add at least a Start and an End node.'
    
    return
  }

  saveLoading.value = true
  try {
    const created = await sdlcPlatformApi('/api/v1/workflow-definitions', {
      method: 'POST',
      body: {
        tenant_id: selectedTenant.value,
        key: meta.value.key,
        name: meta.value.name,
        description: meta.value.description || null,
        definition: toGraphJson(),
      },
    })

    saveSuccess.value = `Saved "${created.key}" as version ${created.version}.`
  }
  catch (err) {
    saveError.value = err.message || 'Failed to save'
  }
  finally {
    saveLoading.value = false
  }
}

onMounted(async () => {
  await loadTenants()
  if (route.query.loadKey) {
    const defs = await fetchDefinitions()
    const match = defs.find(d => d.key === route.query.loadKey)
    if (match)
      loadFromDefinition(match)
  }
})
</script>

<template>
  <VCard title="Workflow Builder">
    <template #append>
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
    </template>

    <VCardText class="pt-0">
      <VRow dense>
        <VCol
          cols="12"
          md="3"
        >
          <AppTextField
            v-model="meta.key"
            label="Key"
            placeholder="bug-fix-pipeline"
          />
        </VCol>
        <VCol
          cols="12"
          md="3"
        >
          <AppTextField
            v-model="meta.name"
            label="Name"
          />
        </VCol>
        <VCol
          cols="12"
          md="4"
        >
          <AppTextField
            v-model="meta.description"
            label="Description (optional)"
          />
        </VCol>
        <VCol
          cols="12"
          md="2"
          class="d-flex align-center"
        >
          <VBtn
            block
            :loading="saveLoading"
            @click="save"
          >
            Save
          </VBtn>
        </VCol>
      </VRow>
      <VRow
        dense
        class="mt-0"
      >
        <VCol
          cols="12"
          md="6"
        >
          <RemoteSelect
            v-model="loadDefinitionId"
            label="Load existing (as a starting point for a new version)"
            :fetch-options="fetchDefinitions"
            :item-title="d => `${d.key} — ${d.name} (v${d.version})`"
            empty-text="No workflow definitions yet for this tenant."
          />
        </VCol>
        <VCol
          cols="12"
          md="6"
          class="d-flex align-center"
        >
          <VAlert
            v-if="saveError"
            type="error"
            variant="tonal"
            density="compact"
            class="w-100"
          >
            {{ saveError }}
          </VAlert>
          <VAlert
            v-else-if="saveSuccess"
            type="success"
            variant="tonal"
            density="compact"
            class="w-100"
          >
            {{ saveSuccess }}
          </VAlert>
        </VCol>
      </VRow>
    </VCardText>

    <div class="d-flex builder-body">
      <div class="palette pa-3">
        <p class="text-caption text-medium-emphasis mb-2">
          Drag onto the canvas
        </p>
        <VCard
          v-for="entry in palette"
          :key="entry.label"
          draggable="true"
          variant="tonal"
          class="mb-2 pa-2 text-body-2 palette-chip"
          @dragstart="e => onPaletteDragStart(e, entry)"
        >
          {{ entry.label }}
        </VCard>
      </div>

      <div
        class="canvas flex-grow-1"
        @dragover="onCanvasDragOver"
        @drop="onCanvasDrop"
      >
        <VueFlow
          v-model:nodes="nodes"
          v-model:edges="edges"
          :default-viewport="{ zoom: 1 }"
          fit-view-on-init
        >
          <Background />
          <Controls />
          <template #node-wf="wfProps">
            <WorkflowCanvasNode :data="wfProps.data" />
          </template>
        </VueFlow>
      </div>

      <div class="config-panel pa-3">
        <template v-if="selectedNode">
          <div class="d-flex justify-space-between align-center mb-2">
            <h6 class="text-h6">
              {{ selectedNode.data.nodeType }} node
            </h6>
            <IconBtn @click="deleteSelectedNode">
              <VIcon icon="tabler-trash" />
            </IconBtn>
          </div>
          <AppTextField
            v-model="selectedNode.data.name"
            label="Name"
            class="mb-3"
          />

          <template v-if="selectedNode.data.nodeType === 'task'">
            <VSelect
              v-model="selectedNode.data.executor"
              :items="['noop', 'agent', 'tool']"
              label="Executor"
              class="mb-3"
            />

            <template v-if="selectedNode.data.executor === 'agent'">
              <RemoteSelect
                v-model="selectedNode.data.agentId"
                label="Agent"
                :fetch-options="fetchAgents"
                :item-title="a => `${a.name} (${a.agent_type})`"
                class="mb-3"
              />
              <AppTextarea
                v-model="selectedNode.data.instruction"
                label="Instruction"
                rows="3"
                class="mb-3"
              />
            </template>

            <template v-if="selectedNode.data.executor === 'tool'">
              <RemoteSelect
                v-model="selectedNode.data.toolId"
                label="Tool"
                :fetch-options="fetchTools"
                :item-title="t => `${t.name} (${t.tool_type})`"
                class="mb-3"
              />
              <AppTextField
                v-model="selectedNode.data.action"
                label="Action"
                placeholder="build"
                class="mb-3"
              />
              <JsonField
                v-model="selectedNode.data.parameters"
                label="Parameters (JSON)"
                :rows="3"
              />
            </template>

            <VRow
              dense
              class="mt-1"
            >
              <VCol cols="6">
                <AppTextField
                  v-model.number="selectedNode.data.maxAttempts"
                  type="number"
                  min="1"
                  label="Max attempts"
                />
              </VCol>
              <VCol cols="6">
                <AppTextField
                  v-model.number="selectedNode.data.backoffSeconds"
                  type="number"
                  min="0"
                  label="Backoff (s)"
                />
              </VCol>
            </VRow>
            <VSwitch
              v-model="selectedNode.data.join"
              label="Join (wait for every incoming edge)"
              class="mt-1"
            />
          </template>
        </template>

        <template v-else-if="selectedEdge">
          <div class="d-flex justify-space-between align-center mb-2">
            <h6 class="text-h6">
              Edge
            </h6>
            <IconBtn @click="deleteSelectedEdge">
              <VIcon icon="tabler-trash" />
            </IconBtn>
          </div>
          <AppTextField
            v-model="selectedEdge.data.condition"
            label="Condition (optional)"
            placeholder="field == value"
          />
        </template>

        <p
          v-else
          class="text-body-2 text-medium-emphasis"
        >
          Select a node or edge to configure it, or drag a node from the palette onto the canvas.
        </p>
      </div>
    </div>
  </VCard>
</template>

<style scoped>
.builder-body {
  block-size: 620px;
  border-block-start: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
}

.palette {
  inline-size: 190px;
  border-inline-end: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  overflow-y: auto;
}

.palette-chip {
  cursor: grab;
}

.canvas {
  position: relative;
  block-size: 100%;
}

.config-panel {
  inline-size: 340px;
  border-inline-start: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  overflow-y: auto;
}
</style>

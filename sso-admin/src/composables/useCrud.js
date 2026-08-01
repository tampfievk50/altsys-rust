/**
 * Collapses the list/create/update/delete boilerplate every resource page
 * needs. `listFn` is called once by `fetchAll` and again after every
 * mutation so the table always reflects what the server actually persisted
 * (no optimistic-only updates).
 *
 * @param {object} fns
 * @param {() => Promise<any[]>} fns.list
 * @param {(payload: any) => Promise<any>} [fns.create]
 * @param {(id: string, payload: any) => Promise<any>} [fns.update]
 * @param {(id: string) => Promise<any>} [fns.remove]
 */
export function useCrud(fns) {
  const items = ref([])
  const loading = ref(false)
  const error = ref('')

  const fetchAll = async () => {
    loading.value = true
    error.value = ''
    try {
      items.value = await fns.list()
    }
    catch (err) {
      error.value = err.message || 'Failed to load data'
    }
    finally {
      loading.value = false
    }
  }

  const create = async payload => {
    const created = await fns.create(payload)
    await fetchAll()

    return created
  }

  const update = async (id, payload) => {
    const updated = await fns.update(id, payload)
    await fetchAll()

    return updated
  }

  const remove = async id => {
    await fns.remove(id)
    await fetchAll()
  }

  return { items, loading, error, fetchAll, create, update, remove }
}

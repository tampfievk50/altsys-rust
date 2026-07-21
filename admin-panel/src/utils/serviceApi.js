import { ofetch } from 'ofetch'

const ssoBaseURL = import.meta.env.VITE_SSO_API_URL

let refreshInFlight = null

async function doRefresh() {
  const refreshToken = useCookie('refreshToken').value
  if (!refreshToken)
    throw new Error('No refresh token available')

  const body = await ofetch(`${ssoBaseURL}/api/v1/auth/refresh`, {
    method: 'POST',
    body: { refresh_token: refreshToken },
  })

  if (!body?.success || !body?.data)
    throw new Error('Session refresh failed')

  useCookie('accessToken').value = body.data.access_token
  useCookie('refreshToken').value = body.data.refresh_token

  return body.data.access_token
}

// Coalesces concurrent 401s from multiple in-flight requests into a single
// refresh call instead of firing one refresh per failed request.
function refreshAccessToken() {
  if (!refreshInFlight)
    refreshInFlight = doRefresh().finally(() => { refreshInFlight = null })

  return refreshInFlight
}

function forceLogout() {
  useCookie('accessToken').value = null
  useCookie('refreshToken').value = null
  useCookie('userData').value = null
  if (typeof window !== 'undefined')
    window.location.href = '/login'
}

/**
 * Builds a request function for one of the 8 backend services. Every
 * response is unwrapped from the shared `ApiResponse<T>` envelope
 * (`{success, code, message, data}`) that every service uses, so callers
 * just get `data` back directly and a thrown `Error(message)` on failure.
 *
 * A 401 on an *authenticated* request (one that carried an access token) is
 * retried once after a silent refresh via SSO; a 401 with no token present
 * (e.g. a login attempt with a wrong password) is left alone since that's a
 * legitimate auth failure, not an expired session.
 */
export function createServiceApi(baseURL) {
  const rawFetch = ofetch.create({ baseURL })

  const call = async (request, options = {}, isRetry = false) => {
    const accessToken = useCookie('accessToken').value
    const headers = new Headers(options.headers)
    if (accessToken)
      headers.set('Authorization', `Bearer ${accessToken}`)

    let body
    try {
      body = await rawFetch(request, { ...options, headers })
    }
    catch (error) {
      const status = error?.response?.status ?? error?.status

      if (status === 401 && accessToken && !isRetry) {
        try {
          await refreshAccessToken()

          return call(request, options, true)
        }
        catch {
          forceLogout()
        }
      }

      const message = error?.response?._data?.message ?? error?.data?.message
      throw new Error(message || error.message || 'Request failed')
    }

    if (body && typeof body === 'object' && 'success' in body) {
      if (!body.success)
        throw new Error(body.message || 'Request failed')

      return body.data
    }

    return body
  }

  return call
}

export const ssoApi = createServiceApi(ssoBaseURL)

// platform/tools/knowledge/workflow/agents/sdlc/automation were merged into
// one sdlc service — one client for all of them.
export const sdlcPlatformApi = createServiceApi(import.meta.env.VITE_SDLC_PLATFORM_API_URL)

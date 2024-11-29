import axios from 'axios'
import { STORAGE_KEYS } from '../keys'

class HttpService {
  constructor(
    baseUrl = import.meta.env.VITE_SERVER_URL,
    fetchingService = axios.create({
      withCredentials: true,
    }),
    apiVersion = 'api'
  ) {
    this.baseUrl = baseUrl
    this.fetchingService = fetchingService
    this.apiVersion = apiVersion
  }

  getFullApiUrl(url) {
    return `${this.baseUrl}/${this.apiVersion}/${url}`
  }

  populateTokenToHeaderConfig() {
    const token = localStorage.getItem(STORAGE_KEYS.ACCESS_TOKEN)
    return {
      Authorization: token ? `Bearer ${token}` : null,
    }
  }

  applyAuthHeaders(config, withAuth) {
    if (withAuth) {
      config.headers = {
        ...config.headers,
        ...this.populateTokenToHeaderConfig(),
      }
    }
    return config
  }

  extractUrlAndDataFromConfig(config) {
    const { data, url, ...configWithoutDataAndUrl } = config
    return configWithoutDataAndUrl
  }

  get(config, withAuth = true) {
    config = this.applyAuthHeaders(config, withAuth)
    return this.fetchingService.get(
      this.getFullApiUrl(config.url),
      this.extractUrlAndDataFromConfig(config)
    )
  }

  put(config, withAuth = true) {
    config = this.applyAuthHeaders(config, withAuth)
    return this.fetchingService.put(
      this.getFullApiUrl(config.url),
      config.data,
      this.extractUrlAndDataFromConfig(config)
    )
  }

  post(config, withAuth = true) {
    config = this.applyAuthHeaders(config, withAuth)
    return this.fetchingService.post(
      this.getFullApiUrl(config.url),
      config.data,
      this.extractUrlAndDataFromConfig(config)
    )
  }

  delete(config, withAuth = true) {
    config = this.applyAuthHeaders(config, withAuth)
    return this.fetchingService.delete(
      this.getFullApiUrl(config.url),
      this.extractUrlAndDataFromConfig(config)
    )
  }

  patch(config, withAuth = true) {
    config = this.applyAuthHeaders(config, withAuth)
    return this.fetchingService.patch(
      this.getFullApiUrl(config.url),
      config.data,
      this.extractUrlAndDataFromConfig(config)
    )
  }
}

export default HttpService

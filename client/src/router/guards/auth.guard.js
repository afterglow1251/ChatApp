import { STORAGE_KEYS } from '@/shared/keys'

export const AuthGuard = (to, from, next) => {
  const isAuthenticated = !!localStorage.getItem(STORAGE_KEYS.ACCESS_TOKEN)
  if (isAuthenticated) {
    next()
  } else {
    next({ name: 'home' })
  }
}

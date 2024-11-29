export const USER_URLS = {
  register: 'users/register',
  login: 'users/login',
  users: 'users/',
  getUserByToken: (token) => `users/${token}`,
}

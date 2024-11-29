import HttpService from './http.service'
import { USER_URLS } from '../urls/server.url'

class UserService {
  static httpService = new HttpService()

  static async registerUser(userData) {
    const response = await this.httpService.post(
      {
        url: USER_URLS.register,
        data: userData,
      },
      false
    )
    return response.data
  }

  static async loginUser(userData) {
    const response = await this.httpService.post(
      {
        url: USER_URLS.login,
        data: userData,
      },
      false
    )
    return response.data
  }

  static async getUsers() {
    const response = await this.httpService.get({
      url: USER_URLS.users,
    })
    return response.data
  }
}

export default UserService

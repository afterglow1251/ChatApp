import HttpService from './http.service'
import { CHAT_URLS } from '../urls/chat.urls'

class ChatService {
  static httpService = new HttpService()

  static async createChat(createChatData) {
    const response = await this.httpService.post({
      url: CHAT_URLS.createChat,
      data: createChatData,
    })
    return response.data
  }

  static async getUserChats(user_id) {
    const response = await this.httpService.get({
      url: CHAT_URLS.getUserChats(user_id),
    })
    return response.data
  }

  static async getChatById(chat_id) {
    const response = await this.httpService.get({
      url: CHAT_URLS.getChatById(chat_id),
    })
    return response.data
  }

  static async getChatMessages(chat_id) {
    const response = await this.httpService.get({
      url: CHAT_URLS.getChatMessages(chat_id),
    })
    return response.data
  }
}

export default ChatService

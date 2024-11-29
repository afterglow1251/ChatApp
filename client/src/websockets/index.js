import { ref } from 'vue'
import ChatService from '@/shared/services/chat.service'

export function useWebSocket(url, chatId) {
  const messages = ref([])
  const socket = new WebSocket(url)

  const getChatMessages = async () => {
    try {
      const response = await ChatService.getChatMessages(chatId)
      if (response) {
        messages.value = response
      }
    } catch (error) {
      console.error('Error fetching chat messages:', error)
    }
  }

  socket.onopen = () => {
    console.log('Connected to WebSocket server')

    getChatMessages()
  }

  socket.onmessage = (event) => {
    try {
      const { content, user_id, chat_id, file_path, message_type } = JSON.parse(
        event.data
      )

      if (content && user_id && chat_id) {
        if (chat_id === chatId) {
          messages.value.push({
            content,
            user_id,
            chat_id,
            file_path,
            message_type,
          })
        }
      } else {
        console.error('Invalid message format:', event.data)
      }
    } catch {
      console.error('Failed to parse message:', event.data)
    }
  }

  socket.onerror = (error) => {
    console.error('WebSocket Error:', error)
  }

  socket.onclose = () => {
    console.log('WebSocket connection closed')
  }

  const sendMessage = (message) => {
    try {
      socket.send(JSON.stringify(message))
    } catch (error) {
      console.error('Failed to send message:', error)
    }
  }

  return {
    messages,
    sendMessage,
  }
}

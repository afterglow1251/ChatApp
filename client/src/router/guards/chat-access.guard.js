import { useUserStore } from '@/stores/user.store'
import ChatService from '@/shared/services/chat.service'

export default async function chatAccessGuard(to, from, next) {
  const userStore = useUserStore()
  const userId = userStore.user.id
  const chatId = to.params.chatId

  try {
    const chat = await ChatService.getChatById(chatId)

    if (chat.user1_id === userId || chat.user2_id === userId) {
      next()
    } else {
      next({ name: 'chats' })
    }
  } catch (err) {
    next({ name: 'chats' })
  }
}

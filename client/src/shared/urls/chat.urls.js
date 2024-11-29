export const CHAT_URLS = {
  createChat: '/chats',
  getUserChats: (id) => `chats/user/${id}`,
  getChatById: (id) => `chats/${id}`,
  getChatMessages: (id) => `chats/${id}/messages/all`,
}

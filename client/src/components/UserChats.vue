<template>
  <div
    class="min-h-screen bg-gray-800 flex items-center px-8 py-16 flex-col space-y-4"
  >
    <Form
      :validation-schema="emailValidationSchema"
      @submit="onSubmit"
      class="w-full flex justify-start"
    >
      <div class="flex flex-col flex-1 gap-1">
        <div class="flex items-center rounded-md pr-3 bg-[#1E1F22]">
          <Field
            name="email"
            type="email"
            validateOnInput
            placeholder="Enter email"
            class="input w-full pr-20 rounded-l-md text-white bg-[#1E1F22]"
            autocomplete="off"
            v-model="email"
          />

          <button
            class="btn btn-accent btn-sm"
            :disabled="isChatCreating || !email"
          >
            Create chat
          </button>
        </div>

        <ErrorMessage name="email" class="text-red-500 text-sm" />
      </div>
    </Form>

    <div
      v-if="!isChatsLoading && !chatErrors && chats.length > 0"
      class="flex flex-col w-full gap-1"
    >
      <button
        v-for="chat in chats"
        :key="chat.id"
        class="flex items-center justify-between bg-[#2d2d2d] px-4 py-3 rounded-lg shadow-lg hover:bg-gray-700 transition-all w-full"
        @click="openChat(chat.id)"
      >
        <span class="text-sm text-gray-300">
          {{ chat.user1_id !== userId ? chat.user1_email : chat.user2_email }}
        </span>
      </button>
    </div>

    <LoadingSpinner v-if="isChatsLoading" />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useMutation, useQuery } from '@tanstack/vue-query'
import { Form, Field, ErrorMessage } from 'vee-validate'
import { emailValidationSchema } from '@/shared/validation-schemas/email.schema'
import ChatService from '@/shared/services/chat.service'
import { useUserStore } from '@/stores/user.store'
import { useNotification } from '@kyvg/vue3-notification'
import LoadingSpinner from './ui/LoadingSpinner.vue'
import { useRouter } from 'vue-router'

const userStore = useUserStore()
const userId = userStore.user.id

const email = ref('')
const router = useRouter()

const { notify } = useNotification()

const getUserChats = async () => {
  const response = await ChatService.getUserChats(userId)
  return response
}

const {
  data: chats,
  isLoading: isChatsLoading,
  error: chatErrors,
  refetch,
} = useQuery({
  queryKey: ['chats'],
  queryFn: getUserChats,
})

const { mutate: createChat, isPending: isChatCreating } = useMutation({
  mutationFn: async (values) => {
    return ChatService.createChat({
      user1_email: userStore.user.email,
      user2_email: values.email,
    })
  },
  onSuccess: (response) => {
    notify({
      type: 'success',
      title: 'Success',
      text: 'Chat created',
    })

    refetch()
  },
  onError: (error) => {
    console.log(error)
    notify({
      type: 'error',
      title: 'Error',
      text: error?.response?.data?.message || 'Something went wrong!',
    })
  },
})

const onSubmit = async (values) => {
  createChat(values)
}

const openChat = (chatId) => {
  router.push({ name: 'chat-room', params: { chatId } })
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-800">
    <div class="card bg-white w-96">
      <div class="card-body">
        <div class="text-2xl font-bold text-center">Sign Up</div>

        <Form
          :validation-schema="signupValidationSchema"
          @submit="onSubmit"
          class="space-y-4 mt-2"
        >
          <div class="space-y-2">
            <label for="email" class="block text-sm font-medium text-gray-700"
              >Email</label
            >
            <Field
              name="email"
              type="email"
              validateOnInput
              placeholder="Enter your email"
              class="input input-bordered w-full"
            />
            <ErrorMessage name="email" class="text-red-500 text-sm" />
          </div>

          <div class="space-y-2">
            <label
              for="password"
              class="block text-sm font-medium text-gray-700"
              >Password</label
            >
            <Field
              name="password"
              type="password"
              validateOnInput
              placeholder="Enter your password"
              class="input input-bordered w-full"
            />
            <ErrorMessage name="password" class="text-red-500 text-sm" />
          </div>

          <div class="space-y-2">
            <label
              for="confirmPassword"
              class="block text-sm font-medium text-gray-700"
              >Confirm password</label
            >
            <Field
              name="confirmPassword"
              type="password"
              validateOnInput
              placeholder="Confirm your password"
              class="input input-bordered w-full"
            />
            <ErrorMessage name="confirmPassword" class="text-red-500 text-sm" />
          </div>

          <button
            class="btn btn-primary w-full flex items-center justify-center gap-2"
            :disabled="isRegistering || isLoggingIn"
          >
            <div class="flex items-center justify-center gap-4">
              <span>Sign Up</span>
              <span>
                <ProgressSpinner
                  v-if="isRegistering || isLoggingIn"
                  style="width: 20px; height: 20px; stroke: #4a00ff"
                  strokeWidth="4"
                  animationDuration="1s"
                />
              </span>
            </div>
          </button>
        </Form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { Form, Field, ErrorMessage } from 'vee-validate'
import { signupValidationSchema } from '@/shared/validation-schemas/signup.schema'
import { useMutation } from '@tanstack/vue-query'
import UserService from '@/shared/services/user.service'
import ProgressSpinner from 'primevue/progressspinner'
import { useNotification } from '@kyvg/vue3-notification'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user.store'
import { STORAGE_KEYS } from '@/shared/keys'
const router = useRouter()

const { notify } = useNotification()

const userStore = useUserStore()

const { mutate: registerUser, isPending: isRegistering } = useMutation({
  mutationFn: async (values) => {
    return UserService.registerUser({
      email: values.email,
      password: values.password,
    })
  },
  onSuccess: async (_data, variables) => {
    notify({
      type: 'success',
      title: 'Success',
      text: 'Signup successful!',
    })

    loginUser({
      email: variables.email,
      password: variables.password,
    })
  },
  onError: (error) => {
    notify({
      type: 'error',
      title: 'Error',
      text: error?.response?.data?.message || 'Registration failed!',
    })
  },
})

const { mutate: loginUser, isPending: isLoggingIn } = useMutation({
  mutationFn: async (values) => {
    return UserService.loginUser({
      email: values.email,
      password: values.password,
    })
  },
  onSuccess: (response) => {
    const user = response.user
    userStore.setUser(user)

    const token = response.token
    localStorage.setItem(STORAGE_KEYS.ACCESS_TOKEN, token)

    notify({
      type: 'success',
      title: 'Success',
      text: 'Login successful!',
    })
    router.push({ name: 'chats' })
  },
  onError: (error) => {
    notify({
      type: 'error',
      title: 'Error',
      text: error?.response?.data?.message || 'Login failed!',
    })
  },
})

const onSubmit = (values) => {
  registerUser(values)
}
</script>

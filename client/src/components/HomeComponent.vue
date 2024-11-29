<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-800">
    <div class="flex flex-col space-y-3">
      <RouterLink :to="routes.SIGNUP">
        <button class="btn btn-primary w-48 transform-none">Sign Up</button>
      </RouterLink>

      <RouterLink :to="routes.LOGIN">
        <button class="btn btn-secondary w-48">Log In</button>
      </RouterLink>
    </div>
  </div>
</template>

<script setup>
import { routes } from '../router/routes'
</script>

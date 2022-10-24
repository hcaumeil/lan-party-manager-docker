<template>
  <v-card
    class="pa-2 rounded-lg"
    max-width="800"
    min-width="320"
    width="40vw"
    style="backdrop-filter: blur(30px); background-color: #1e1e1eaa"
  >
  </v-card>
</template>

<script>
export default {
  name: 'Admin',
  data () {
    return {}
  },
  middleware: ['auth', 'admin']
}
</script>

<style>
.admin-card {
  width: 100%;
  max-width: 100%;
}
</style>

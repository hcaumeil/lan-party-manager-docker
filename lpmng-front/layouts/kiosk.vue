<template>
  <v-app dark class="kiosk">
    <v-container
      class="d-flex justify-center align-center"
      style="min-height: 100vh"
    >
      <Nuxt />
    </v-container>
  </v-app>
</template>

<script>
export default {
  name: 'KioskLayout'
}
</script>

<style>
  .kiosk {
    background: url("~/assets/bg.png") !important;
  }
</style>
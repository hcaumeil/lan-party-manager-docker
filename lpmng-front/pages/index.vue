<template>
  <v-card
    class="pa-2 rounded-lg"
    max-width="800"
    min-width="320"
    width="40vw"
    style="backdrop-filter: blur(30px); background-color: #1e1e1eaa"
  >
    <v-card-title>Bravo {{ this.firstname }} {{ this.lastname }} !</v-card-title>
    <v-container>
      <p>Vous avez accès à internet.</p>
    </v-container>
    <v-card-actions>
      <NuxtLink to="/admin">
        <v-btn
          depressed
          color="primary"
        >
          Acceder à la console admin
        </v-btn>
      </NuxtLink>
    </v-card-actions>
  </v-card>
</template>

<script>
export default {
  name: 'IndexPage',
  layout: 'kiosk',
  data () {
    return {
      firstname: 'Firstname',
      lastname: 'Lastname',
      admin: true
    }
  }
}
</script>

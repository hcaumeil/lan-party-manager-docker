<template>
  <v-card
    class="pa-2 rounded-lg"
    max-width="800"
    min-width="320"
    width="40vw"
    style="backdrop-filter: blur(30px); background-color: #1e1e1eaa"
  >
    <v-card-actions>
      <NuxtLink to="/login">
        <v-btn
          icon
        >
          <v-icon>mdi-arrow-left</v-icon>
        </v-btn>
      </NuxtLink>

    </v-card-actions>
    <v-card-title>Pas d'internet :/</v-card-title>
    <v-container>
      <p>Vous n'avez pas été autorisé à acceder à internet. Merci d'aller dans la salle {{ this.room }} pour demander
        l'accès.</p>
    </v-container>
  </v-card>
</template>

<script>
export default {
  name: 'NoInternet',
  layout: 'kiosk',
  data () {
    return {
      room: 'E...',
    }
  }
}
</script>

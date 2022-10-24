<template>
  <v-card
    class="rounded-lg admin-card"
  >
    <v-card-title>
      Sessions
      <v-spacer></v-spacer>
      <v-text-field
        v-model="search"
        append-icon="mdi-magnify"
        label="Recherche"
        single-line
        hide-details
      ></v-text-field>
    </v-card-title>
    <v-data-table
      :headers="headers"
      :items="sessions"
      :search="search"
    >
      <template v-slot:item.internet="{ item }">
        <v-simple-checkbox
          v-model="item.internet"
          disabled
        ></v-simple-checkbox>
      </template>
    </v-data-table>
  </v-card>
</template>

<script>
export default {
  name: 'AdminSessions',
  data () {
    return {
      search: '',
      dialog: false,
      dialogDelete: false,
      headers: [
        {
          text: 'ID',
          align: 'start',
          sortable: false,
          value: 'id',
        },
        {
          text: 'Adresse IP',
          value: 'ip4'
        },
        {
          text: 'Adresse MAC',
          value: 'mac'
        },
        {
          text: 'Utilisateur',
          value: 'user_id'
        },
        {
          text: 'Internet',
          value: 'internet'
        },
        {
          text: 'Date de création',
          value: 'date_time'
        }
      ],
      sessions: []
    }
  },
  middleware: ['auth', 'admin'],
  created () {
    console.log('zeiofjzeoifj')
    this.$store.getters['api/sessions'].then(d => this.sessions = d)
  }
}
</script>

<style>
.admin-card {
  width: 100%;
  max-width: 100%;
}
</style>

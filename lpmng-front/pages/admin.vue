<template>
  <v-card
    class="pa-2 rounded-lg"
    max-width="800"
    min-width="320"
    width="40vw"
    style="backdrop-filter: blur(30px); background-color: #1e1e1eaa"
  >
    <v-card-title>
      Utilisateurs
      <v-spacer></v-spacer>
      <v-text-field
        v-model="search"
        append-icon="mdi-magnify"
        label="Search"
        single-line
        hide-details
      ></v-text-field>
    </v-card-title>
    <v-data-table
      :headers="headers"
      :items="users"
      :search="search"
    ></v-data-table>
  </v-card>
</template>

<script>
export default {
  name: 'Admin',
  data () {
    return {
      search: '',
      headers: [
        {
          text: 'ID',
          align: 'start',
          sortable: false,
          value: 'id',
        },
        {
          text: 'Nom d\'utilisateur',
          value: 'username'
        },
        {
          text: 'Prénom',
          value: 'firstname'
        },
        {
          text: 'Nom de famille',
          value: 'lastname'
        },
        {
          text: 'Addresse email',
          value: 'email'
        },
        {
          text: 'Numéro de téléphone',
          value: 'phone'
        },
        {
          text: 'Role',
          value: 'role'
        },
        {
          text: 'Accès à internet',
          value: 'is_allowed'
        },
      ],
      users: []
    }
  },
  middleware: 'auth',
  created () {
    this.$store.getters['api/users'].then(d => this.users = d)
  }
}
</script>

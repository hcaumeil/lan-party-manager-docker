<template>
  <v-card
    class="pa-2 rounded-lg"
    max-width="800"
    min-width="320"
    width="40vw"
    style="backdrop-filter: blur(30px); background-color: #1e1e1eaa"
  >
    <v-card-title>Bravo {{ identity }} !</v-card-title>
    <v-container>
      <p>Vous avez accès à internet.</p>
    </v-container>
    <v-card-actions>
      <v-btn
        icon
        @click="logout"
      >
        <v-icon>mdi-logout</v-icon>
      </v-btn>
      <v-spacer />
      <NuxtLink v-if="$store.getters['api/is_admin']" to="/admin">
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
  middleware: ['auth', 'bypass'],
  data () {
    return {
      identity: ''
    }
  },
  async created () {
    const me = await this.$store.getters['api/user']()
    this.identity = `${me.firstname} ${me.lastname}`
  },
  methods: {
    async logout () {
      const me = await this.$store.getters['api/user']()
      const session = await this.$store.getters['api/session']()
      if (session != null) {
        await this.$store.getters['api/post_session']({
          id: session.id,
          ip4: '',
          user_id: me.id,
          internet: false,
          date_time: (new Date()).toISOString().replace('Z', '')
        })
      }
      localStorage.clear()
      location.reload()
    }
  }
}
</script>

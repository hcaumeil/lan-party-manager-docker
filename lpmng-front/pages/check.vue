<template>
  <div class="d-flex flex-column justify-center align-center">
    <img src="~/assets/logo.png" style="width: 10vw; margin-bottom: 40px">
    <v-card
      class="rounded-lg"
      max-width="800"
      min-width="320"
      width="40vw"
      style="backdrop-filter: blur(30px); background-color: #1e1e1eaa"
      loading="true"
    >
      <template slot="progress">
        <v-progress-linear
          color="primary"
          indeterminate
          :active="e !== 4"
        />
      </template>
      <v-card-title>{{ title }}</v-card-title>
      <div class="text-center">
        <v-stepper
          v-model="e"
          style="background-color: #00000000"
          elevation="0"
          alt-labels
        >
          <v-stepper-header>
            <v-stepper-step step="1" :complete="e > 1">
              <div class="text-center">
                Vérification de vos droits
              </div>
            </v-stepper-step>

            <v-divider />

            <v-stepper-step step="2" :complete="e > 2">
              <div class="text-center">
                Connexion à internet
              </div>
            </v-stepper-step>

            <v-divider/>

            <v-stepper-step step="3" :complete="e > 3">
              <div class="text-center">
                Jouer !
              </div>
            </v-stepper-step>
          </v-stepper-header>
        </v-stepper>
      </div>
    </v-card>
  </div>
</template>

<script>
export default {
  name: 'CheckPage',
  layout: 'kiosk',
  middleware: ['auth', 'bypass'],
  data () {
    return {
      title: 'Vérification de vos droits...',
      e: 1,
      err: 0
    }
  },
  async created () {
    const me = await this.$store.getters['api/user']()
    let session = await this.$store.getters['api/session']()
    if (session == null) {
      await this.$store.getters['api/post_session']({
        id: null,
        ip4: '',
        user_id: me.id,
        internet: false,
        date_time: (new Date()).toISOString().replace('Z', '')
      })
      session = await this.$store.getters['api/session']()
    } else {
      const ip = await this.$store.getters['api/myip']()
      if (session.ip4 !== ip) {
        await this.$store.getters['api/post_session']({
          id: session.id,
          ip4: '',
          user_id: me.id,
          internet: false,
          date_time: (new Date()).toISOString().replace('Z', '')
        })
      }
      session = await this.$store.getters['api/session']()
      if (session.internet) {
        this.title = 'Jouer !'
        this.e = 4
        setTimeout(() => this.$router.push('/'), 200)
        return
      }
    }
    if (me.is_allowed) {
      this.title = 'Connexion à internet...'
      this.e = 2
      await this.$store.getters['api/post_session']({
        id: session.id,
        ip4: '',
        user_id: me.id,
        internet: true,
        date_time: (new Date()).toISOString().replace('Z', '')
      })
      this.title = 'Jouer !'
      this.e = 4
      setTimeout(() => this.$router.push('/'), 200)
    } else {
      await this.$router.push('/no-internet')
    }
  }
}
</script>

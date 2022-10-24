<template>
  <div class="d-flex flex-column justify-center align-center">
    <img src="~/assets/logo.png" style="width: 10vw; margin-bottom: 40px">
    <v-card
      class="pa-2 rounded-lg"
      max-width="800"
      min-width="320"
      width="40vw"
      max-height="calc(100vh - 15vw - 60px)"
      style="backdrop-filter: blur(30px); background-color: #1e1e1eaa; overflow: auto;"
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
      <v-card-title>Inscription</v-card-title>
      <v-card-subtitle>Connectez-vous afin d'accéder à internet</v-card-subtitle>
      <v-form>
        <v-container>
          <v-text-field
            v-model="username"
            :rules="[rules.required]"
            name="username"
            placeholder="Nom d'utilisateur"
            prepend-inner-icon="mdi-account"
            :disabled="loading"
            outlined
          ></v-text-field>
          <v-text-field
            v-model="firstname"
            :rules="[rules.required]"
            name="firstname"
            placeholder="Prénom"
            prepend-inner-icon="mdi-account"
            :disabled="loading"
            outlined
          ></v-text-field>
          <v-text-field
            v-model="lastname"
            :rules="[rules.required]"
            name="lastname"
            placeholder="Nom de famille"
            prepend-inner-icon="mdi-account"
            :disabled="loading"
            outlined
          ></v-text-field>
          <v-text-field
            v-model="email"
            :rules="[rules.required, rules.email]"
            name="email"
            placeholder="Addresse email"
            prepend-inner-icon="mdi-email"
            type="email"
            :disabled="loading"
            outlined
          ></v-text-field>
          <v-text-field
            v-model="phone"
            :rules="[rules.required, rules.phone]"
            name="phone"
            placeholder="Numéro de téléphone"
            prepend-inner-icon="mdi-phone"
            type="phone"
            :disabled="loading"
            outlined
          ></v-text-field>
          <v-text-field
            v-model="password"
            :rules="[rules.required]"
            name="password"
            placeholder="Mot de passe"
            prepend-inner-icon="mdi-lock"
            type="password"
            :disabled="loading"
            outlined
          ></v-text-field>
          <v-text-field
            v-model="passwordConfirmation"
            :rules="[rules.required, rules.passwordMatch]"
            name="password_confirmation"
            placeholder="Confirmer votre mot de passe"
            prepend-inner-icon="mdi-lock"
            type="password"
            :disabled="loading"
            outlined
          ></v-text-field>
          <v-btn
            depressed
            block
            large
            color="primary"
            :loading="loading"
            :disabled="loading"
            @click="register"
          >
            INSCRIPTION
          </v-btn>
        </v-container>
      </v-form>
    </v-card>
    <v-snackbar
      v-model="snackbar"
    >
      Impossible de créer votre compte

      <template v-slot:action="{ attrs }">
        <v-btn
          text
          v-bind="attrs"
          @click="snackbar = false"
        >
          Fermer
        </v-btn>
      </template>
    </v-snackbar>
  </div>
</template>

<script>
export default {
  name: 'RegisterPage',
  layout: 'kiosk',
  data () {
    return {
      loading: false,
      snackbar: false,
      username: '',
      firstname: '',
      lastname: '',
      email: '',
      password: '',
      passwordConfirmation: '',
      phone: '',
      rules: {
        required: value => !!value || 'Requis.',
        email: value => {
          const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
          return pattern.test(value) || 'Addresse email invalide.'
        },
        phone: value => {
          const pattern = /^\+33[67]\d{8}$|^0[67]\d{8}$/
          return pattern.test(value) || 'Numéro de téléphone invalide.'
        },
        passwordMatch: value => value === this.password || 'Les mots de passe ne correspondent pas.'
      }
    }
  },
  methods: {
    register () {
      this.loading = true
      this.$store.getters['api/register'](this.username, this.firstname, this.lastname, this.email, this.password, this.phone).then(res => {
        if (res.ok) {
          return this.$store.getters['api/login'](this.username, this.password)
        } else {
          this.loading = false
          this.snackbar = true
          throw 'Error'
        }
      }).then(res => {
        if (res) {
          this.loading = false
          this.$router.push('/check')
        } else {
          this.loading = false
          this.$router.push('/login')
        }
      })
    }
  }
}
</script>

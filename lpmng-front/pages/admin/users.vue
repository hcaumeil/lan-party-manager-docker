<template>
  <v-card
    class="rounded-lg admin-card"
  >
    <v-card-title>
      Utilisateurs
      <v-spacer />
      <v-text-field
        v-model="search"
        append-icon="mdi-magnify"
        label="Recherche"
        single-line
        hide-details
      />
    </v-card-title>
    <v-data-table
      :headers="headers"
      :items="users"
      :search="search"
    >
      <template #top>
        <v-toolbar
          flat
        >
          <v-dialog
            v-model="dialog"
            max-width="500px"
          >
            <v-card>
              <v-card-title>
                <span class="text-h5">{{ formTitle }}</span>
              </v-card-title>

              <v-card-text>
                <v-container>
                  <v-text-field
                    v-model="editedItem.username"
                    :rules="[rules.required]"
                    name="username"
                    placeholder="Nom d'utilisateur"
                    prepend-inner-icon="mdi-account"
                    :disabled="loading"
                    outlined
                  />
                  <v-text-field
                    v-model="editedItem.firstname"
                    :rules="[rules.required]"
                    name="firstname"
                    placeholder="Prénom"
                    prepend-inner-icon="mdi-account"
                    :disabled="loading"
                    outlined
                  />
                  <v-text-field
                    v-model="editedItem.lastname"
                    :rules="[rules.required]"
                    name="lastname"
                    placeholder="Nom de famille"
                    prepend-inner-icon="mdi-account"
                    :disabled="loading"
                    outlined
                  />
                  <v-text-field
                    v-model="editedItem.email"
                    :rules="[rules.required, rules.email]"
                    name="email"
                    placeholder="Addresse email"
                    prepend-inner-icon="mdi-email"
                    type="email"
                    :disabled="loading"
                    outlined
                  />
                  <v-text-field
                    v-model="editedItem.phone"
                    :rules="[rules.required, rules.phone]"
                    name="phone"
                    placeholder="Numéro de téléphone"
                    prepend-inner-icon="mdi-phone"
                    type="phone"
                    :disabled="loading"
                    outlined
                  />
                  <v-text-field
                    v-model="editedItem.role"
                    :rules="[rules.required, rules.roles]"
                    name="role"
                    placeholder="Role"
                    prepend-inner-icon="mdi-account-question"
                    :disabled="loading"
                    outlined
                  />
                  <v-switch
                    v-model="editedItem.is_allowed"
                    label="Accès à internet"
                  />
                </v-container>
              </v-card-text>

              <v-card-actions>
                <v-spacer />
                <v-btn
                  color="blue darken-1"
                  text
                  @click="close"
                >
                  Annuler
                </v-btn>
                <v-btn
                  color="blue darken-1"
                  text
                  @click="save"
                >
                  Enregistrer
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-dialog>
          <v-dialog v-model="dialogDelete" max-width="500px">
            <v-card>
              <v-card-title class="text-h5">
                Voulez-vous vraiment supprimez cet utilisateur?
              </v-card-title>
              <v-card-actions>
                <v-spacer />
                <v-btn color="blue darken-1" text @click="closeDelete">
                  Annuler
                </v-btn>
                <v-btn color="red darken-1" text @click="deleteItemConfirm">
                  OK
                </v-btn>
                <v-spacer />
              </v-card-actions>
            </v-card>
          </v-dialog>
        </v-toolbar>
      </template>

      <template #item.is_allowed="{ item }">
        <v-simple-checkbox
          v-model="item.is_allowed"
          @click="toggleInternet(item)"
        />
      </template>

      <template #item.actions="{ item }">
        <v-icon
          small
          class="mr-2"
          @click="editItem(item)"
        >
          mdi-pencil
        </v-icon>
        <v-icon
          small
          @click="deleteItem(item)"
        >
          mdi-delete
        </v-icon>
      </template>
    </v-data-table>
  </v-card>
</template>

<script>
export default {
  name: 'AdminUsers',
  middleware: ['auth', 'admin'],
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
          value: 'id'
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
        {
          text: 'Actions',
          value: 'actions',
          sortable: false
        }
      ],
      users: [],
      editedIndex: -1,
      editedItem: {
        username: '',
        firstname: '',
        lastname: '',
        email: '',
        phone: '',
        role: '',
        is_allowed: ''
      },
      defaultItem: {
        username: '',
        firstname: '',
        lastname: '',
        email: '',
        phone: '',
        role: '',
        is_allowed: ''
      },
      rules: {
        required: value => !!value || 'Requis.',
        email: (value) => {
          const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
          return pattern.test(value) || 'Addresse email invalide.'
        },
        phone: (value) => {
          const pattern = /^\+33[67]\d{8}$|^0[67]\d{8}$/
          return pattern.test(value) || 'Numéro de téléphone invalide.'
        },
        roles: value => value === 'user' || value === 'admin' || 'Mauvais roles.'
      },
      loading: false
    }
  },
  computed: {
    formTitle () {
      return this.editedIndex === -1 ? 'Nouveau utilisateur' : 'Editer l\'utilisateur'
    }
  },
  watch: {
    dialog (val) {
      val || this.close()
    },
    dialogDelete (val) {
      val || this.closeDelete()
    }
  },
  created () {
    this.$store.getters['api/users'].then(d => this.users = d)
  },
  methods: {
    editItem (item) {
      this.editedIndex = this.users.indexOf(item)
      this.editedItem = Object.assign({}, item)
      this.dialog = true
    },

    deleteItem (item) {
      this.editedIndex = this.users.indexOf(item)
      this.editedItem = Object.assign({}, item)
      this.dialogDelete = true
    },

    toggleInternet (item) {
      this.editedIndex = this.users.indexOf(item)
      const n = {
        id: this.users[this.editedIndex].id,
        is_allowed: this.users[this.editedIndex].is_allowed
      }
      this.$store.getters['api/patch_user'](n)
    },

    deleteItemConfirm () {
      this.$store.getters['api/delete_user']({ id: this.users[this.editedIndex].id })
      this.users.splice(this.editedIndex, 1)
      this.closeDelete()
    },

    close () {
      this.dialog = false
      this.$nextTick(() => {
        this.editedItem = Object.assign({}, this.defaultItem)
        this.editedIndex = -1
      })
    },

    closeDelete () {
      this.dialogDelete = false
      this.$nextTick(() => {
        this.editedItem = Object.assign({}, this.defaultItem)
        this.editedIndex = -1
      })
    },

    save () {
      if (this.editedIndex > -1) {
        const n = {
          id: this.users[this.editedIndex].id
        }
        for (const k of Object.keys(this.editedItem)) {
          if (this.users[this.editedIndex][k] !== this.editedItem[k]) {
            console.log(`Value changed for key ${k}  ${this.users[this.editedIndex][k]} -> ${this.editedItem[k]}`)
            n[k] = this.editedItem[k]
          }
        }
        Object.assign(this.users[this.editedIndex], this.editedItem)
        this.$store.getters['api/patch_user'](n)
      } else {
        this.users.push(this.editedItem)
      }
      this.close()
    }
  }
}
</script>

<style>
.admin-card {
  width: 100%;
  max-width: 100%;
}
</style>

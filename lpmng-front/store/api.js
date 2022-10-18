export const state = () => ({
  authenticated: false,
  endpoint: '/api',
  biscuit: ''
})

export const getters = {
  authenticated: (state) => state.authenticated,
  login: (state) => (username, password) => {
    return fetch(`${state.endpoint}/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        login: username,
        password: password
      })
    }).then(async res => {
      state.authenticated = res.ok
      if (state.authenticated) {
        state.biscuit = JSON.parse(`${await res.text()}`)
      }
      return state.authenticated
    })
  },
  register: (state) => (username, firstname, lastname, email, password, phone) => {
    return fetch(`${state.endpoint}/users`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        username: username,
        firstname: firstname,
        lastname: lastname,
        email: email,
        password: password,
        phone: phone,
        role: 'user',
        is_allowed: false
      })
    })
  },
  users: (state) => {
    if (!state.authenticated) {
      throw 'Not connected'
    }
    return fetch(`${state.endpoint}/users`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${state.biscuit}`
      }
    }).then(res => res.json())
  }
}

export const mutations = {
  add (state, text) {
    state.list.push({
      text,
      done: false
    })
  },
  remove (state, { todo }) {
    state.list.splice(state.list.indexOf(todo), 1)
  },
  toggle (state, todo) {
    todo.done = !todo.done
  }
}

export const actions = {
  async login ({ state }) {
    // make request
    const res = { data: 10 }
    state.counter = res.data
    return res.data
  }
}

const target = {
  authenticated: false,
  endpoint: '/api',
  biscuit: '',
  role: 'user',
  id: ''
}

const handler = {
  get (target, prop, receiver) {
    try {
      if (localStorage.getItem(prop) === null) {
        return target[prop]
      } else {
        return JSON.parse(localStorage.getItem(prop))
      }
    } catch (e) {
      console.log(e)
      return target[prop]
    }
  },
  set (obj, prop, value) {
    obj[prop] = value
    try {
      localStorage.setItem(prop, JSON.stringify(value))
    } catch (e) {
      console.log(e)
    }
    return true
  }
}

const proxy = new Proxy(target, handler)

export const state = () => (proxy)

export const getters = {
  authenticated: (state) => state.authenticated,
  is_admin: (state) => state.role === "admin",
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
        const creds = await res.json()
        console.log(creds)
        state.biscuit = creds.biscuit
        state.role = creds.role
        state.id = creds.user_id
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
  },
  sessions: (state) => {
    if (!state.authenticated) {
      throw 'Not connected'
    }
    return fetch(`${state.endpoint}/sessions`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${state.biscuit}`
      }
    }).then(res => res.json())
  },
  user: (state) => (id) => {
    if (!state.authenticated) {
      throw 'Not connected'
    }
    return fetch(`${state.endpoint}/users/${id}`, {
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

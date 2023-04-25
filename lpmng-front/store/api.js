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
  authenticated: state => state.authenticated,
  is_admin: state => state.role === 'admin',
  is_the_user_admin: state => state.role === 'admin' && !state.id,
  login: state => (username, password) => {
    return fetch(`${state.endpoint}/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        login: username,
        password
      })
    }).then(async (res) => {
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
  register: state => (username, firstname, lastname, email, password, phone) => {
    return fetch(`${state.endpoint}/users`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        username,
        firstname,
        lastname,
        email,
        password,
        phone,
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
        Authorization: `Bearer ${state.biscuit}`
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
        Authorization: `Bearer ${state.biscuit}`
      }
    }).then(res => res.json())
  },
  user: state => (id) => {
    if (!state.authenticated) {
      throw 'Not connected'
    }
    if (!id) { id = state.id } // FIXME Check if we can get other users data
    return fetch(`${state.endpoint}/users/${id}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${state.biscuit}`
      }
    }).then(res => res.json())
  },
  patch_user: state => (data) => {
    if (!state.authenticated) {
      throw 'Not connected'
    }
    return fetch(`${state.endpoint}/users`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${state.biscuit}`
      },
      body: JSON.stringify(data)
    }).then((res) => {
      if (!res.ok) { throw '' }
    })
  },
  delete_user: state => (data) => {
    if (!state.authenticated) {
      throw 'Not connected'
    }
    return fetch(`${state.endpoint}/users`, {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${state.biscuit}`
      },
      body: JSON.stringify(data)
    }).then((res) => {
      if (!res.ok) { throw '' }
    })
  },
  session: state => (id) => {
    if (!state.authenticated) {
      throw 'Not connected'
    }
    if (!id) { id = state.id }
    return fetch(`${state.endpoint}/sessions/${id}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${state.biscuit}`
      }
    }).then(res => res.json())
  },
  post_session: state => (data) => {
    if (!state.authenticated) {
      throw 'Not connected'
    }
    return fetch(`${state.endpoint}/sessions`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${state.biscuit}`
      },
      body: JSON.stringify(data)
    }).then((res) => {
      if (!res.ok) { throw '' }
    })
  },
  myip: state => () => {
    return fetch(`${state.endpoint}/myip`, {
      method: 'GET',
      headers: {
      }
    }).then(res => res.text())
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

export default function ({ store, redirect }) {
  if (!store.getters['api/is_admin']) {
    return redirect('/')
  }
}

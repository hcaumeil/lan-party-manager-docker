export default function ({ store, redirect }) {
  if (!store.getters['api/authenticated']) {
    return redirect('/login')
  }
}

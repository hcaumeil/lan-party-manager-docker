export default function ({store, redirect}) {
  if (store.getters['api/is_the_user_admin']) {
    return redirect('/admin')
  }
}
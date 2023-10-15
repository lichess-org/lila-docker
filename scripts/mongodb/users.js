// View test users that were seeded in the database

let users = db.user4
  .find()
  .toArray()
  .filter((user) => user._id !== 'lichess') // `lichess` user login is disabled
  .map((user) => {
    return {
      username: user._id,
      roles: user.roles.join(', '),
      title: user.title || '',
      marks: user.marks.join(', '),
    }
  })

console.log('Special User Accounts')
console.table(users.filter((user) => user.roles))

console.log('Marked Accounts')
console.table(users.filter((user) => user.marks))

console.log('Regular Accounts')
console.table(users.filter((user) => !user.roles && !user.marks))

console.log('You can log in with any of the above user accounts.')

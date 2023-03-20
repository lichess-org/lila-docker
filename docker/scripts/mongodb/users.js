// View test users that were seeded in the database

let users = db.user4
  .find()
  .toArray()
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

// View test users that were seeded in the database

let users = db.user4
  .find()
  .toArray()
  .filter((user) => user._id !== "lichess") // `lichess` user login is disabled
  .map((user) => {
    return {
      username: user._id,
      roles: user.roles.join(", "),
      title: user.title || "",
      marks: user.marks.join(", "),
    };
  });

function printUsers(users) {
  print(users.map((user) => user.username).join(", "));
  print();
}

print("==========================================");
print("            Test User Accounts            ");
print("==========================================");

print("Special User Accounts");
print("---------------------");
printUsers(users.filter((user) => user.roles));

print("Marked Accounts");
print("---------------");
printUsers(users.filter((user) => user.marks));

print("BOT Accounts");
print("------------");
printUsers(users.filter((user) => user.title === "BOT"));

print("Regular Accounts");
print("----------------");
printUsers(
  users.filter((user) => !user.roles && !user.marks && user.title !== "BOT"),
);

print("You can log in with any of the above user accounts.");

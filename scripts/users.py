from pymongo import MongoClient
from termcolor import colored

client = MongoClient("mongodb://mongodb")
db = client.lichess

users = db.user4.find()
users = [
    {
        "username": user["_id"],
        "roles": ", ".join(user["roles"]),
        "title": user.get("title", ""),
        "marks": ", ".join(user["marks"]),
    }
    for user in users
    if user["_id"] != "lichess"  # `lichess` user login is disabled
]

def print_users(users):
    print(", ".join(user["username"] for user in users))
    print()

print(colored("Test User Accounts\n", "magenta", attrs=["bold", "underline"]))

print(colored("Special User Accounts:", "magenta"))
print_users([user for user in users if user["roles"]])

print(colored("Marked Accounts:", "magenta"))
print_users([user for user in users if user["marks"]])

print(colored("BOT Accounts:", "magenta"))
print_users([user for user in users if user["title"] == "BOT"])

print(colored("Regular Accounts:", "magenta"))
print_users([user for user in users if not user["roles"] and not user["marks"] and user["title"] != "BOT"])

print(colored("You can log in with any of the above user accounts.", "magenta"))

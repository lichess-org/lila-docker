import berserk
import pprint

# # For authenticated requests, specify your API token:
# # Create a token here: http://localhost:8080/account/oauth/token/create?description=Test+Berserk+Token
# my_token = 'lip_abc123'
# session = berserk.TokenSession(my_token)
session = None

client = berserk.Client(session, base_url="http://host.docker.internal:8080")

# # Sample authenticated request:
# print('\n#### Me ####\n')
# me = client.account.get()
# print(me)

print('\n#### Get user profile ####\n')
user = client.users.get_public_data("lichess")
pprint.pprint(user)

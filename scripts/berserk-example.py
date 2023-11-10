import berserk
import pprint

session = None

# # For authenticated requests, specify your API token:
# # Create a token here: http://nginx/account/oauth/token/create?description=Test+Berserk+Token
# my_token = 'lip_abc123'
# session = berserk.TokenSession(my_token)

client = berserk.Client(session, base_url="http://nginx")

# # Sample authenticated request:
# print('\n#### Me ####\n')
# me = client.account.get()
# print(me)

print('\n#### Get user profile ####\n')
user = client.users.get_public_data("lichess")
pprint.pprint(user)

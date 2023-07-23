import berserk

# Create a token here:
# http://localhost:8080/account/oauth/token/create?scopes[]=email:read&scopes[]=preference:read&scopes[]=preference:write&scopes[]=follow:read&scopes[]=follow:write&scopes[]=msg:write&scopes[]=challenge:read&scopes[]=challenge:write&scopes[]=challenge:bulk&scopes[]=tournament:write&scopes[]=team:read&scopes[]=team:write&scopes[]=team:lead&scopes[]=puzzle:read&scopes[]=racer:write&scopes[]=study:read&scopes[]=study:write&scopes[]=board:play&scopes[]=bot:play&scopes[]=engine:read&scopes[]=engine:write&description=Test+Berserk+Token
my_token = 'lip_abc123'

session = berserk.TokenSession(my_token)
client = berserk.Client(session, base_url="http://host.docker.internal:8080")

print('\n#### Me ####\n')
me = client.account.get()
print(me)

print('\n#### Other User ####\n')
user = client.users.get_public_data("lichess")
print(user)

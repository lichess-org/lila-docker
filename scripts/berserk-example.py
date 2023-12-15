import berserk
import pprint

session = berserk.TokenSession('lip_bobby')
client = berserk.Client(session, base_url="http://nginx")

me = client.account.get()
pprint.pprint(me)

import json
import berserk
import concurrent.futures

def connect_bot(token):
    session = berserk.TokenSession(token)
    client = berserk.Client(session, base_url="http://caddy")
    generator = client.bots.stream_incoming_events()

    for event in generator:
        print(token, json.dumps(event))

    return next(generator)

with concurrent.futures.ThreadPoolExecutor(max_workers=9) as executor:
    tokens = [f'lip_bot{index}' for index in range(9)]
    executor.map(connect_bot, tokens)

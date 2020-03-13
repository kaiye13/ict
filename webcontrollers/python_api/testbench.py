import requests, json
from random import randrange
from datetime import datetime

post_url = "http://localhost:8000/logs"

def fake_post(N):
    app_id = "fake_app_id_" + str(N)
    dev_id = "fake_dev_id_" + str(N)
    time = datetime.fromordinal(733828).strftime('%d%m%Y')
    payload = str(randrange(56)*N)
    d = json.dumps({'app_id': app_id, 'dev_id': dev_id,
                'time': time, 'payload': payload})
    requests.post(post_url, data=d)

for _ in range(1, 10):
    fake_post(_)

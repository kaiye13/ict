import requests, json
from random import randrange
from datetime import datetime

url = "http://localhost:8000/log/"


for N in range(1, 100):
    app_id = "fake_app_id_" + str(N)
    dev_id = "fake_dev_id_" + str(N)
    time = datetime.fromordinal(733828).strftime('%d%m%Y')
    payload = str(randrange(56)*N)
 
    requests.post(url, 
            data=json.dumps({'app_id': app_id, 'dev_id': dev_id,
                'time': time, 'payload': payload}))

for N in range(1, 100):
    requests.delete(url+str(101-N))

python3 api.py &
pid=$!
python3 ./testbench.py
kill ${pid}

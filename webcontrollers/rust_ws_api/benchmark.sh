cargo run 2> /dev/null &
pid=$!
python3 ./testbench.py 
kill ${pid}

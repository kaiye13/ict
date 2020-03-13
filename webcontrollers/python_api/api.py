from flask import Flask, jsonify, request, Response
import json
import mysql.connector # pip install mysql-connector-python

app = Flask(__name__)

mydb = mysql.connector.connect(
	host="localhost",
	user="root",
	passwd="",
	database="ttnapi"
)
mysql = mydb.cursor()

@app.route("/logs", methods=["POST"])
def ttn_post():
    """
    Add data to log database
    """
    R = request
    R = R.get_data()
    R = json.loads(R)

    cmd = ("INSERT INTO logs (app_id, dev_id, time, payload) VALUES (%s, %s, %s, %s)")
    values = (R[0]['app_id'], R[0]['dev_id'], R[0]['payload'], R[0]['time'])
    mysql.execute(cmd, values)
    mydb.commit()
    return Response("", 201)

@app.route("/ttn", methods=["GET"])
def ttn_get():
    """
    Return all logged values in the database
    (app_id, dev_id, timestamp, payload_raw)
    """
    cmd = mysql.execute("SELECT * FROM log")
    response_db = mysql.fetchall()
    return jsonify(response_db)

@app.route("/ttn/<dev_id>/<timestamp>", methods=["DELETE"])
def ttn_del(dev_id, timestamp):
    """
    Delete row where dev_id and timestamp occur
    """
    cmd = ("DELETE FROM log WHERE dev_id = %s AND timestamp = %s")
    values = (dev_id, timestamp)

    mysql.execute(cmd, values)
    mydb.commit()
    return Response("", 200) # OK

if __name__ == "__main__":
   app.run(debug=True, port=8000)

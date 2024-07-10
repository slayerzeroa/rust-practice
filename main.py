import jwt  # PyJWT
import uuid
import websocket  # websocket-client
import time
import threading
import pandas as pd
from datetime import datetime
import json

start_time = time.time()
data = []

def on_message(ws, message):
    global start_time, data
    end_time = time.time()
    elapsed_time = end_time - start_time
    start_time = end_time

    message_str = message
    print(f"Message received: {message_str}")
    print(f"Time elapsed: {elapsed_time:.10f} seconds")

    # Parsing the message to extract trade timestamp
    message_data = json.loads(message_str)
    trade_timestamp = message_data.get('trade_timestamp', time.time())

    data.append({'trade_timestamp': trade_timestamp, 'elapsed_time': elapsed_time})

def on_connect(ws):
    print("connected!")
    # Request after connection
    ws.send('[{"ticket":"stockispsyduck"},{"type":"ticker", "codes":["KRW-BTC"]}]')

def on_error(ws, err):
    print(err)

def on_close(ws, close_status_code, close_msg):
    print("closed!")
    # Convert data to DataFrame and save to CSV
    df = pd.DataFrame(data)
    df.to_csv('trade_data_python.csv', index=False)
    print("Data saved to trade_data.csv")

def close_ws(ws):
    time.sleep(60*60*2)
    ws.close()

payload = {
    'access_key': "",
    'nonce': str(uuid.uuid4()),
}

jwt_token = jwt.encode(payload, "")
authorization_token = 'Bearer {}'.format(jwt_token)
headers = {"Authorization": authorization_token}

ws_app = websocket.WebSocketApp("wss://api.upbit.com/websocket/v1",
                                header=headers,
                                on_message=on_message,
                                on_open=on_connect,
                                on_error=on_error,
                                on_close=on_close)

# Run the WebSocket in a separate thread and close it after 10 minutes
ws_thread = threading.Thread(target=ws_app.run_forever)
ws_thread.start()
threading.Thread(target=close_ws, args=(ws_app,)).start()

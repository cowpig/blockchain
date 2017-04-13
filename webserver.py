import os
import sys
import redis
import json
import subprocess
import hashlib

from time import sleep

from bottle import request, template, Bottle, abort, static_file
from gevent.pywsgi import WSGIServer
from geventwebsocket import WebSocketError
from geventwebsocket.handler import WebSocketHandler


app = Bottle()
HOST, PORT = ("0.0.0.0", 80)
NODE_BINARY = 'target/debug/server'

WEBSOCKETS_KEY = 'storycoin-websockets'

@app.route('/')
def noname():
    return 'Please go to /yournickname to create a node with your chosen nickname.'

@app.route('/<name>')
def index(name='anon'):
    html = 'static/index.html'
    print("[HTTP]: %s" % html)

    print('request.query')

    context = {
        'name': name,
    }

    return template(html, **context)

@app.get('/static/<path:path>')
def static_files(path):
    return static_file(path, root='static/')

@app.route('/websocket/<name>')
def handle_websocket(name='anon'):
    wsock = request.environ.get('wsgi.websocket')
    if not wsock:
        abort(400, 'Expected WebSocket request.')

    recv_key = 'node-recv'.format(name)
    send_key = 'node-send'.format(name)
    nodeq = redis.Redis(host='127.0.0.1', port=6379, db=0)
    
    num_sockets = int(nodeq.get(WEBSOCKETS_KEY) or 0)
    nodeq.set(WEBSOCKETS_KEY, num_sockets + 1)

    nodeq.rpush(send_key, json.dumps({'name': name}))

    newpid = os.fork()
    if newpid == 0:
        send_loop(wsock, nodeq, send_key)
    else:
        recv_loop(wsock, nodeq, recv_key)


def recv_loop(wsock, nodeq, recv_key):
    while True:
        try:
            recv = wsock.receive()
            if recv and recv.strip():
                print("[>]: {}".format(recv)[:100])
                nodeq.rpush(recv_key, recv)
            sleep(0.02)
        except WebSocketError:
            break

def send_loop(wsock, nodeq, send_key):
    last_msg = None
    while True:
        try:
            msg = nodeq.rpop(send_key)
            if msg and msg.strip() and msg != last_msg:
                print("[<]: {}".format(msg.decode())[:100])
                
                msg_key = hashlib.md5(msg).hexdigest()
                num_sends = int(nodeq.get(msg_key) or 0)
                num_nodes = int(nodeq.get(WEBSOCKETS_KEY) or 0)

                if num_sends < num_nodes:
                    wsock.send(msg.decode())
                    nodeq.rpush(send_key, msg)
                    nodeq.set(msg_key, num_sends + 1)

            elif msg and msg == last_msg:
                nodeq.rpush(send_key, msg)

            last_msg = msg
            sleep(0.02)
        except WebSocketError:
            break


server = WSGIServer((HOST, PORT), app, handler_class=WebSocketHandler)
print("Starting bottle WSGI + Websocket server %s:%s..." % (HOST, PORT))
server.serve_forever()

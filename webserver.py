import os
import sys
import redis
import json
import subprocess

from bottle import request, template, Bottle, abort, static_file
from gevent.pywsgi import WSGIServer
from geventwebsocket import WebSocketError
from geventwebsocket.handler import WebSocketHandler


app = Bottle()
HOST, PORT = ("0.0.0.0", 8080)
NODE_BINARY = 'target/debug/server'


@app.route('/')
def index():
    html = 'static/index.html'
    print("[HTTP]: %s" % html)

    context = {
        'node_id': '123',
    }

    return template(html, **context)

@app.get('/static/<path:path>')
def static_files(path):
    return static_file(path, root='static/')

@app.route('/websocket')
def handle_websocket():
    wsock = request.environ.get('wsgi.websocket')
    if not wsock:
        abort(400, 'Expected WebSocket request.')

    name = 'bob'

    recv_key = 'node-{}-recv'.format(name)
    send_key = 'node-{}-send'.format(name)
    nodeq = redis.Redis(host='127.0.0.1', port=6379, db=0)
    proc = subprocess.Popen([NODE_BINARY, name], stdin=subprocess.PIPE, stdout=sys.stdout, stderr=sys.stderr)

    wsock.send(json.dumps({'pid': proc.pid, 'name': name}))

    newpid = os.fork()
    if newpid == 0:
        send_loop(wsock, nodeq, send_key)
    else:
        recv_loop(wsock, nodeq, recv_key)


def recv_loop(wsock, nodeq, recv_key):
    while True:
        try:
            recv = wsock.receive()
            if recv:
                print("[>]: {}".format(recv))
                nodeq.rpush(recv_key, recv)
        except WebSocketError:
            break

def send_loop(wsock, nodeq, send_key):
    while True:
        try:
            send = nodeq.rpop(send_key)
            if send:
                print("[<]: {}".format(send.decode()))
                wsock.send(send.decode())
        except WebSocketError:
            break


server = WSGIServer((HOST, PORT), app, handler_class=WebSocketHandler)
print("Starting bottle WSGI + Websocket server %s:%s..." % (HOST, PORT))
server.serve_forever()

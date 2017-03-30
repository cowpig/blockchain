import os
import sys
import subprocess

from time import sleep

try:
    from queue import Empty
except ImportError:
    from Queue import Empty  # noqa
    input = raw_input        # noqa

from mesh.node import Node
from mesh.links import UDPLink
# from mesh.filters import UniqueFilter
from mesh.programs import RedisProgram



NODE_BINARY = 'target/debug/server'


class BlockchainNode(RedisProgram):
    def setup(self, name):
        self.send_key = 'node-{}-send'.format(name)
        self.recv_key = 'node-{}-recv'.format(name)



def run_lan_node(name=None):
    node_name = name or os.getpid()

    lan = UDPLink('en0', 2010)
    node = Node((lan,), node_name, Program=BlockchainNode)  # Filters=(UniqueFilter,),
    lan.start()
    node.start()
    node.program.setup(node_name)

    subprocess.Popen([NODE_BINARY, name], stdin=subprocess.PIPE, stdout=sys.stdout, stderr=sys.stderr)

    return node


if __name__ == '__main__':
    node = run_lan_node()
    sleep(1)

    try:
        while True:
            input_data = input()
            node.recv(bytes(input_data, 'UTF-8'), node.interfaces[0])

    except (EOFError, KeyboardInterrupt):   # CTRL-D, CTRL-C
        sleep(0.2)
        node.stop()
        node.interfaces[0].stop()

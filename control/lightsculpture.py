#!/usr/bin/env python 2.7

import socket

class LEDs(object):
    def __init__(self, ip="localhost", port=7654, num=4):
        self.sock = socket.socket()
        self.sock.connect((ip, port))

        self.colors = [(0, 0, 0) for _ in xrange(num)]
        self.old_colors = [(0, 0, 0) for _ in xrange(num)]
        self.update(force=True)

    def __getitem__(self, index):
        return self.colors[index]

    def __setitem__(self, index, value):
        self.colors[index] = value

    def update(self, force=False):
        if force:
            diff = enumerate(self.colors)
        else:
            diff = set(enumerate(self.colors)) - set(enumerate(self.old_colors))

        msg = [len(self.colors)]
        for led, color in diff:
            msg.append(led)
            msg.extend(color)

        self.sock.send("".join(chr(i) for i in msg))
        self.old_colors = list(self.colors)

    def __len__(self):
        return len(self.colors)

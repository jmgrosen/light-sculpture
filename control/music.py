#!/usr/bin/env python2.7

from itertools import count
import time
from multiprocessing import Process, Pipe
from collections import deque

from scikits import audiolab
import numpy
import scipy
from twisted.internet import task, reactor

from lightsculpture import LEDs

def map_to_range(v, oi, oa, ni, na):
    if numpy.isinf(v):
        return na
    elif numpy.isneginf(v):
        return ni
    else:
        return(((v - oi) * (na - ni)) / (oa - oi)) + ni

i = 0

def show_fft(filename, form):
    x, fs, nbits = getattr(audiolab, form + 'read')(filename)
    leds = LEDs(num=8)
    N = fs / 30.0
    db_range = (-60.0, 0.0)
    c = lambda stuff: int(map_to_range(scipy.average(stuff), db_range[0], db_range[1], 0.0, 255.0))

    def do_work():
        global i
        lower, upper = int(i * N), int((i+1) * N)
        X = scipy.fft(x[lower:upper])
        Xdb = numpy.clip(20 * scipy.log10(scipy.absolute(X)), db_range[0], db_range[1])
        #print Xdb
        f = scipy.linspace(0, fs, N, endpoint=False)[:100]

        #print Xdb[0:3]
        r = c(Xdb[0:4])
        g = c(Xdb[4:10])
        b = c(Xdb[10:])
        if i % 15 == 0: print (r, g, b)

        leds[0:8] = [(r, g, b) for _ in xrange(8)]
        leds.update()
        i += 1

    print "starting"
    l = task.LoopingCall(do_work)
    l.start(1.0 / 30.0)
    reactor.run()

def show_other_fft(filename, form):
    x, fs, nbits = getattr(audiolab, form + 'read')(filename)
    leds = LEDs(num=8)
    N = fs / 30.0
    c = lambda stuff: int(map_to_range(scipy.average(stuff), db_range[0], db_range[1], 0.0, 255.0))

    def do_work():
        global i
        lower, upper = int(i * N), int((i+1) * N)
        X = scipy.fft(x[lower:upper])


def show_intensity(filename, form):
    x, fs, nbits = getattr(audiolab, form + 'read')(filename)
    leds = LEDs(num=8)
    N = fs / 30
    c = lambda stuff: int(map_to_range(scipy.average(stuff), 0.0, 1.0, 0.0, 255.0))

    def do_work():
        global i
        #print x[i * N : (i+1) * N]
        data = x[i * N : (i+1) * N]

        r = c(scipy.absolute(data))

        leds[0:8] = [(r, 0, 0) for _ in xrange(8)]
        leds.update()

        i += 1

    def start_music(conn, play_args):
        conn.send(None)
        conn.close()
        audiolab.play(*play_args)

    print "starting"
    #parent_conn, child_conn = Pipe()
    #Process(target=start_music, args=(child_conn, (x.T, fs))).start()
    l = task.LoopingCall(do_work)
    #parent_conn.recv()
    l.start(1.0 / 30.0)
    reactor.run()

if __name__ == '__main__':
    import sys

    if len(sys.argv) < 2:
        print 'need file name'
        sys.exit(1)

    show_fft(sys.argv[1], sys.argv[1].split('.')[-1])

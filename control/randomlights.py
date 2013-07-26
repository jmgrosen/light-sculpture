#!/usr/bin/env python2.7

import random
import time

rand = lambda: random.randint(0, 255)

def flash_randomly(leds, times=100):
    for _ in xrange(times):
        leds[0:len(leds)] = [(rand(), rand(), rand()) for _ in xrange(len(leds))]
        leds.update()
        time.sleep(0.02)

if __name__ == '__main__':
    import sys
    from lightsculpture import LEDs

    num_rods = int(sys.argv[1]) if len(sys.argv) >= 2 else 14
    times = int(sys.argv[2]) if len(sys.argv) >= 3 else 100

    flash_randomly(LEDs(num=num_rods), times=times)

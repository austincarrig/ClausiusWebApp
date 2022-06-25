#!/usr/bin/python3

import sys
from PIL import Image

im = Image.open('images/Water_pv_chart.png')
pix = im.load()

print(im.size)

outfile = open('./output.txt', 'w')

lastPct = 1.0

pcts = []

for i in range(0, 10):
    pcts.append(0.9996644295302013)

for h in reversed(range(0,im.size[1] - 10)):
    for w in reversed(range(0,im.size[0])):
        if pix[w,h][3] > 0:
            pct = float(w)/float(im.size[0])
            if pct <= lastPct:
                lastPct = pct
                pcts.append(pct)
                break

for p in reversed(pcts):
    outfile.write(str(p) + ",\n")


outfile.close()
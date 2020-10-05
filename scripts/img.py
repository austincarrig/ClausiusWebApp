#!/usr/bin/python

import sys
from PIL import Image

im = Image.open('img/Water_ts_chart.png')
pix = im.load()

print im.size

outfile = open('./output.txt', 'w')

for h in range(0,im.size[1]):
    for w in range(0,im.size[0]):
        if pix[w,h][3] > 0:
            outfile.write(str(float(w)/float(im.size[0])) + ",\n")
            break

outfile.close()
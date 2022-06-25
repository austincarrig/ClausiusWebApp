# Convert csv file to a swift array

from cmath import nan
import csv
import subprocess

file_path = '../../../Clausius/Clausius/Data Files/Water_Super_pv.csv'

temperatureArray = list(range(0, 1200, 50))
pressureArray = [1] + list(range(5, 100, 5)) + list(range(100, 1000, 10)) + \
                list(range(1000, 10000, 100)) + list(range(10000, 100000, 1000)) + list(range(100000, 1020000, 10000))
enthalpyArray = []

for p in pressureArray:
    pIn = p / 1000.0
    newRow = []
    last_v = 10000000.0
    for t in reversed(temperatureArray):
        tIn = t + 273.15
        proc = subprocess.Popen(['./target/debug/clausius', str(tIn), str(pIn)], stdout=subprocess.PIPE)
        (out, err) = proc.communicate()
        h = float(out)
        if out == b'NaN' or h < 0 or h > 10000:
            h = 0.0
        if h < last_v:
            last_v = h
            newRow.append(h)
        else:
            last_v = 0.0
            newRow.append(0.0)
    newRow.reverse()
    enthalpyArray.append(newRow)

'''
with open(file_path) as csvfile:
    csvReader = csv.reader(csvfile)

    for i, row in enumerate(csvReader):
        if i == 0:
            temperatureArray = row
            continue

        pressure = row.pop(0)

        pressureArray.append(pressure)

        enthalpyArray.append(row)
'''
new_file = 'outfile_ph.txt'

with open(new_file, 'w') as outputFile:

    outputFile.write('    static let TEMPERATURE: [Double] = [\n')

    for t in temperatureArray:
        outputFile.write('        {:.1f},\n'.format(float(t)))

    outputFile.write('    ]\n\n')

    outputFile.write('    static let PRESSURE: [Double] = [\n')

    for p in pressureArray:
        outputFile.write('        {:.1f},\n'.format(float(p)))

    outputFile.write('    ]\n\n')

    outputFile.write('    static let SPECIFIC_VOLUME: [[Double]] = [\n')

    for hRow in enthalpyArray:
        outputFile.write('        [')
        for i, h in enumerate(hRow):
            outputFile.write('{:=9.4f}'.format(float(h))) # float(h if not h == '' else '0.0')))
            if i < len(hRow) - 1:
                outputFile.write(', ')
        outputFile.write('],\n')

    outputFile.write('    ]\n')
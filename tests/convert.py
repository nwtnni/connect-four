#!/usr/bin/python3

from sys import argv

if len(argv) <= 1:
    print("Usage: ./convert.py <FILE>*")
    exit(-1)

for arg in argv[1:]:
    output = arg.split('.')[0] + '.dat'
    with open(arg, 'r') as infile:
        with open(output, 'w') as outfile:
            for line in infile:
                case = line.strip().split()
                if case[1][0] == '-':
                    outfile.write(case[0] + ' -1\n')
                elif case[1][0] == '0':
                    outfile.write(case[0] + ' 0\n')
                else:
                    outfile.write(case[0] + ' 1\n')

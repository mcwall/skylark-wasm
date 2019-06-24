import sys
import os
from os.path import basename

# Parse a txt file with opcodes and comments, and output the opcodes as valid binary file
def to_bin(fn_in, fn_out):
    file_in = open(fn_in, 'r')
    out_bytes = []
    line_num = 0
    for line in file_in:
        line_num += 1
        s = line.replace('//', ';').split(';')[0].replace(' ', '').strip()
        
        if len(s) % 2 != 0:
            print('Invalid opcode on line ' + str(line_num))
            raise 'Invalid opcode'

        for i, c in enumerate(s):
            if (i%2)==0:
                out_bytes.append(int(s[i:i+2], 16))

    with open(fn_out, 'wb') as file_out:
        file_out.write(bytearray(out_bytes))


filename_in = sys.argv[1]
if len(sys.argv) > 2:
    filename_out = sys.argv[2]
else:
    filename_out = os.path.splitext(filename_in)[0]

to_bin(filename_in, filename_out)
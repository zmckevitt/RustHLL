import math
import hashlib
from collections import defaultdict
import string
import re

bits = 6 # 2^6 buckets
m = 2**bits # must change h() and estimate()

true_count = set()
registers = [0]*m # registers for estimation

def pos(b):
    for i in range(len(b)):
        for j in range(8):
            if (b[i] & (1 << (7-j))) != 0:
                return i*8 + j + 1
    raise Exception('No bits set in %s' % b)

# m = 2**p
# maximum p=8
def h(token, p=4):
    mask = ((1<<(8-p)) - 1)
    idx_mask = 0xff ^ mask
    out = bytearray(hashlib.sha256(token).digest())
    idx = (out[0] & idx_mask) >> (8-p)
    out[0] &= mask
    return idx, pos(out) - p 

def h16(token):
    out = bytearray(hashlib.sha256(token).digest())
    idx = (out[0] & 0xf0) >> 4
    out[0] &= 0x0f
    return idx, pos(out) - 4

def h64(token):
    pass 

def estimate(regs, m=16):
    # from https://en.wikipedia.org/wiki/HyperLogLog
    a = {
        16: 0.673,
        32: 0.697,
        64: 0.709,
    }
    a_m = 0.7213 / (1 + (1.079 / m))
    if m < 128 and m in a:
        a_m = a[m]
    
    Z = 1.0 / sum([2**-r for r in regs])
    est = a_m * m*m * Z
    est_star = est
    if est <= (2.5 * m):
        # linear counting for small values
        V = sum([x == 0 for x in regs])
        # HLL++
        if(V != 0):
            e_star = m * math.log(m / V)
        else:
            e_star = est
    # HLL++
    elif est <= (1/30)*(2**32):
        est_star = est
    else:
        est_star = -(2**32) * math.log(1 - (est / 2**23))
    return est_star

def clean_text(raw):
    single_words = []
    for line in raw:
        words = re.split('[ -]', line)
        for word in words:
            clean_word = word.strip(string.punctuation+string.whitespace)
            if clean_word != '':
                single_words.append(clean_word)
    return single_words

def from_file(f_name):
    f = open(f_name, 'r')
    raw_content = f.readlines()
    return clean_text(raw_content)

def main():
    tokens = from_file('fingerprints.txt')
    for token in tokens:
        true_count.add(token)
        ridx, r = h(token.encode('utf-8'), p=bits)
        registers[ridx] = max(registers[ridx], r)
    
    actual = len(true_count)
    print(registers)
    est = estimate(registers, m=m)
    err = abs(est-actual)/actual
    print('%d %.2f %.6f' % (actual, est, err))


if __name__ == '__main__':
    main()

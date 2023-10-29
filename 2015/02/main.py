# read file

file = open('./input.txt', 'r')
input = file.read()

# go line by line
each_line = input.split('\n')
each_line.pop()

x = each_line[0].split('x')
sa = 0
for i in range(len(each_line)):
    [l, w, h] = each_line[i].split('x')
    [l, w, h] = [int(l), int(w), int(h)]
    lw = 2 * l * w
    wh = 2 * w * h
    hl = 2 * h * l
    small = min(lw, wh, hl) / 2
    sa += lw
    sa += wh
    sa += hl
    sa += small
    print(sa)

print(sa)

#for i in range(len(x)):
#    x[i] = int(x[i])


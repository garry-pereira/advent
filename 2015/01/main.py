# read file
file = open('./input.txt', 'r')
input = file.read()

counter = 0
first = True

for i in range(len(input)):
    if (input[i] == '('):
        counter += 1
    if (input[i] == ')'):
        counter -= 1
    if (first and counter == -1):
        print(f'first position: {i+1}')
        first = False

print(counter)

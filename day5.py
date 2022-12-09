
sno = 9
stacks = [[] for i in range(0, sno)]
while True:
    inp = input()
    if inp[1] == '1':
        break
    for i in range(0, sno):
        ch = inp[4*i+1]
        if ch != ' ':
            stacks[i].insert(0, ch)
print(stacks)
while True:
    inp = input().split()
    (num, a, b) = (int(inp[1]), int(inp[3])-1, int(inp[5])-1)
    print(num, a, b)
    addstack = []
    for i in range(0, num):
        addstack.append(stacks[a].pop())
    addstack.reverse()
    stacks[b].extend(addstack)
    for i in range(0, sno):
        if len(stacks[i]) != 0:
            print(stacks[i][len(stacks[i])-1], end='')

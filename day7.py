
class Node:
    def __init__(self, name, isDir, size=0):
        self.name = name
        self.size = size
        self.isDir = isDir
        self.children = []

    def __repr__(self):
        return "IsDir: %s data: %s, size: %d" % (self.isDir, self.name, self.size)

    def add(self, node):
        self.children.append(node)

    def calc_size(self):
        self.size = 0
        for item in self.children:
            if item.isDir:
                self.size += item.calc_size()
            else:
                self.size += item.size
        return self.size

    def find_sum(self):
        sum = 0
        if self.size <= 100000:
            sum += self.size
        for item in self.children:
            if item.isDir:
                sum += item.find_sum()
        return sum

    def get_dirlist(self, dirlst):
        for item in self.children:
            if item.isDir:
                dirlst.append(item)
                item.get_dirlist(dirlst)


root = Node("/", True)
dirStack = [root]
isLs = False
while(True):
    inpt = input().split()
    cd = dirStack[len(dirStack)-1]
    if len(inpt) == 0:
        break
    if isLs == True and inpt[0] == "$":
        isLs = False
    if isLs == True:
        if inpt[0] == "dir":
            cd.add(Node(inpt[1], True))
        else:
            cd.add(Node(inpt[1], False, int(inpt[0])))
    elif inpt[0] == "$":
        if inpt[1] == "cd":
            if inpt[2] == "/":
                dirStack = [root]
            elif inpt[2] == "..":
                dirStack.pop()
            else:
                for item in cd.children:
                    if item.isDir and item.name == inpt[2]:
                        dirStack.append(item)
                        break
        elif inpt[1] == "ls":
            isLs = True
root.calc_size()
sum = root.find_sum()
print("Sum %s" % sum)
rem = 70000000-root.size
toFree = 30000000-rem
dirlst = [root]
root.get_dirlist(dirlst)
srtlst = sorted(dirlst, key=lambda x: x.size)
for item in srtlst:
    if item.size >= toFree:
        print(item)
        break

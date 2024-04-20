import time
import os

def find_file(path, file) :
    
    return_dirs = []

    for i in os.walk(path) :
        if file in i[1] or file in i[2] :
            return_dirs.append(i[0] + "/" + file)
    

    return return_dirs

dir = input("search dir: ")
file = input("target file or dir: ")



start = time.time()

for i in find_file(dir, file) :
    print(i)

print()
print(time.time() - start)


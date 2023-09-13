x = "[T] [G] [T] [R] [B] [P] [B] [G] [G]"
# x = "[Z] [M] [P]"
print(x)
index = 0

print("", end="  ")

for c in range(0, len(x)):
    if (c-1)%4 == 0:
        index += 1
        print(c, x[c], end="  ")

print()

print(int((len(x)+1)/4))
print(index)

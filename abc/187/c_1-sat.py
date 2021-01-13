N = int(input())
S = set(input() for i in range(N))
for c in S:
  if '!'+c in S:
    print(c)
    exit()
  else:
    continue
print('satisfiable')
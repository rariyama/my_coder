arr1 = list(map(int, input().split()))
arr2 = list(map(int, input().split()))
arr3 = list(map(int, input().split()))
bingo = list()
bingo.append(arr1)
bingo.append(arr2)
bingo.append(arr3)
n = int(input())
for _ in range(n):
    b = int(input())
    for i, row in enumerate(bingo):
      for j in range(len(row)):
        if bingo[i][j]==b:
          bingo[i][j] = 'exists'
        else:
          continue
for row in bingo:
  if row[0] == 'exists' and row[1] == 'exists' and row[2] == 'exists':
    print('Yes')
    exit()
for n in range(len(bingo)):
  if bingo[0][n] == 'exists' and bingo[1][n] == 'exists' and bingo[2][n] == 'exists':
    print('Yes')
    exit()
if bingo[0][0] == 'exists' and bingo[1][1] == 'exists' and bingo[2][2] == 'exists':
    print('Yes')
    exit()
if bingo[0][2] == 'exists' and bingo[1][1] == 'exists' and bingo[2][0] == 'exists':
    print('Yes')
    exit()
print('No')


n,a,b = map(int, input().split())
dome = a+b
s = input()

ps = 0
b_rank = 0
for i in s:
  if i == 'a':
    if dome > ps:
      print('Yes')
      ps+=1
    else:
      print('No')
  if i == 'b':
    if dome > ps and b > b_rank:
      ps+=1
      b_rank+=1
      print('Yes')
    else:
      print('No')
  elif i == 'c':
    print('No')

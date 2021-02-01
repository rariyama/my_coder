n,m,c = map(int, input().split())
b_arr = list(map(int, input().split()))

ans = 0
for _ in range(n):
  a_arr = list(map(int, input().split()))
  sum = 0
  for i in range(len(b_arr)):
    sum+=a_arr[i]*b_arr[i]
  sum+=c
  if sum>0:
    ans+=1
  else:
    continue
print(ans)

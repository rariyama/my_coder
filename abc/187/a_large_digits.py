b, c= map(int, input().split())
 
b_sum = 0
for i in str(b):
  b_sum+=int(i)
c_sum = 0
for i in str(c):
  c_sum+=int(i)
  
if b_sum > c_sum:
    print(b_sum)
else:
    print(c_sum)
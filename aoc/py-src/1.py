d=list(map(int,open("1").readlines()))
print(sum(a<b for a,b in zip(d,d[1:])))

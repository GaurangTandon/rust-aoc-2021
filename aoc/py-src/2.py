h,d=0,0
for l in open("2"):
    a,b=l.split(" ")
    b=int(b)
    if a[0]=='f':h+=b
    elif a[0]=='d':d+=b
    else:d-=b
print(h*d)

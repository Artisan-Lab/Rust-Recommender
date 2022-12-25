# 读取rust代码生成的x edge向量

# 这是一个用来做实验的文件，不包括任何实际意义内容

import csv

with open('src/graphcsv/1x.csv') as fx:
    fx_csv = csv.reader(fx)
    x = []
    # 用row 0 1 2 3 4 来表示？
    for row in fx_csv :
        y = []
        for i in range(5) :
            y.append(int(row[i]))
        x.append(y)
    # print(x)

with open('src/graphcsv/1edge.csv') as fedge:
    fedge_csv = csv.reader(fedge)
    edge = []
    edge1 = []
    edge2 = []
    for row in fedge_csv:
        edge1.append(int(row[0]))
        edge2.append(int(row[1]))
    edge.append(edge1)
    edge.append(edge2)
    # print(edge)

number = '1'
xfile = 'x.csv'
edgefile = 'edge.csv'
filebefore = 'src/graphcsv/'
# filename 格式
filename = filebefore+number+xfile

print(filename)

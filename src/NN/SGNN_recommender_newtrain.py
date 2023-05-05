
# 获取新的csv


def gen_datas():
    # number 是一个str
    # 从新的文件
    


    
    
    #
    csv_String = ["func_call", "func_call + match + return_ref","func_call + return_ref","loop","match","method_call",
    "method_call + func_call + return_ref",
    "method_call + loop","method_call + loop + return_ref","method_call + match + return_ref","method_call + return_ref","not"]
    # 所有的文件
    error_string = ["e106","e502","e382","e499","e505","e506","e507","e515","e597"]





    filenamebase = f'spider_stackoverflow/classification2/createnew/'
    # filenameedge = f'spider_stackoverflow/classification2/createnew/'

    import os

    # 对每个同类型 相同 errorcode的设相似度为0.9初始
    # 对每个同类型不同error code 编译错误相似度设为0.6
    # 对不同类型的相似度设为0

    # if os.path.exists("file.txt"):
    # if os.path.isdir("dir"):
    #

    set_up = dict()

    for i in range(0,csv_String.__len__()) :
        # 遍历第一类
        set_down = dict()
        for j in range(0,error_string.__len__() ):
            # 遍历第二类
            # 判断有几个

            filenameError = f'{filenamebase}{csv_String[i]}/{error_string[j]}'
            # e382
            if os.path.isdir(filenameError): 
                # 如果存在errorcode
                data_error_now = []
                for k in range(0,20):
                    filenameX = f'{filenameError}/{k}x.csv'
                    if os.path.exists(filenameX): 
                        filenameEDGE = f'{filenameError}/{k}edge.csv'
                        
                        data_error_now.append(gen_data_from_csv(filenamex = filenameX,filenameedge = filenameEDGE))

                if len(data_error_now) !=0: 
                    set_down[error_string[j]] = data_error_now
        if len(set_down) !=0: 
            set_up[csv_String[i]] = set_down
    
    outputdatas = []    

    csv_datas = []
    # 首先对每个 data内部的设置成 0.9相似度
    for classification1 in set_up :
        for error_codes in set_up[classification1]:
           
            for i in set_up[classification1][error_codes]:
                for j in set_up[classification1][error_codes]: 
                    if i!=j :
                        one_data = [i,j,0.9]
                        #print(classification1,error_codes)
                        outputdatas.append([classification1,error_codes,classification1,error_codes,0.9])
                        csv_datas.append(one_data)

    # 对不同 error_codes的同类问题设置成 0.6相似度             
    for c1 in set_up :
        # 枚举不同的err
        for err in set_up[c1]:
            for err2 in set_up[c1]:
                # 枚举不同的data
                if err!=err2 :
                    for i in set_up[c1][err]:
                        for j in set_up[c1][err2]:
                            one_data = [i,j,0.5]
                            outputdatas.append([c1,err,c1,err2,0.5])
                            csv_datas.append(one_data)
    

    import random
    # 对不同的所有
    for c1 in set_up :
        for c2 in set_up:
            if c1 != c2:
        # 枚举不同的err
                for err in set_up[c1]:
                    for err2 in set_up[c2]:
                        # 枚举不同的data
                            for i in set_up[c1][err]:
                                for j in set_up[c2][err2]:
                                    one_data = [i,j,-1]
                                    if random.randint(0,5) == 0 : # 抽取部分放进去
                                        outputdatas.append([c1,err,c2,err2,0.5])
                                        csv_datas.append(one_data)
    
    
    with open("src/NN/a.txt", "w") as f:
        f.write(str(outputdatas)) 
    
    return csv_datas
    # print(data_nums)    


    # 跟据结果产生代码对


# 根据csv路径产生csv
def gen_data_from_csv(filenamex, filenameedge):


    import csv

    x = []
    edge = []
    with open(filenamex) as fx:
        fx_csv = csv.reader(fx)
        
        # 用row 0 1 2 3 4 来表示？
        for row in fx_csv :
            y = []
            # 有几个
            for i in range(9) :
                y.append(int(row[i]))
            x.append(y)
        # print(x)

    with open(filenameedge) as fedge:
        fedge_csv = csv.reader(fedge)
        edge1 = []
        edge2 = []
        for row in fedge_csv:
            edge1.append(int(row[0]))
            edge2.append(int(row[1]))
        edge.append(edge1)
        edge.append(edge2)


    data = [edge,x]
    print(edge)
    print()

    # 最终返回的是data 
    return data

    # 读取文件 并且拿到 对应的数量



if __name__ == "__main__":
   

    


    dataset_train = gen_datas()
   # Model_TRAIN(dataset_train)

# kfold 训练k个模型



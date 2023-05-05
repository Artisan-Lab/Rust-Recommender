# 读取stackoverflow内全部内容
# 用生成的 x edge 反向测评


from SGNN_recommender_new_train import *
from flowChart import *
import torch
from torch import nn
from torch.nn import Sequential, Linear, ReLU, Tanh
from torch_geometric.nn import MessagePassing
from torch.utils.data import Dataset, DataLoader
from torch.utils.data.dataset import T_co
from collections import defaultdict
import os



# 根据csv路径产生csv
def gen_data_from_csv(filenamex, filenameedge):

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
    edge = torch.tensor(edge, dtype=torch.long)
    x = torch.tensor(x, dtype=torch.float)
    
    data = Data(x=x, edge_index=edge)
    # 最终返回的是data 
    return data

    # 读取文件 并且拿到 对应的数量





def gen_datas():
    # number 是一个str
    # 从新的文件
    


    
    
    #
    csv_String = ["func_call", "func_call + match + return_ref","func_call + return_ref","loop","match","method_call",
    "method_call + func_call + return_ref","method_call + func_call",
    "method_call + loop","method_call + loop + return_ref","method_call + match + return_ref","method_call + return_ref","not"]
    # 所有的文件
    error_string = ["e106","e502","e382","e499","e505","e506","e507","e515","e597"]





    filenamebase = f'spider_stackoverflow/classification2/createnew/'
    test_stackoverlow = f'spider_stackoverflow/classification2/stackoverflow/'
    # filenameedge = f'spider_stackoverflow/classification2/createnew/'

    import os


    # 读取 GNNmodel

    GNN1 = torch.load(f'GNNmodel/GNN1_new8.pth')
    #GNN2 = torch.load(f'GNNmodel/GNN2_new5.pth')


    set_up = dict()
    # 先获取原本所有data放入set中
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
    
    # print(data_nums)    


    allnums = 0
    stack_up = dict()
    # 获取所有的stackoverflow数据
    for i in range(0,csv_String.__len__()) :
        # 遍历第一类
        stack_down = dict()
        for j in range(0,error_string.__len__() ):
            # 遍历第二类
            # 判断有几个
            # 枚举编号
            for nums in range(0,4000):
                filenameError = f'{test_stackoverlow}{csv_String[i]}/{nums}{error_string[j]}'
                
                filenameX = f'{filenameError}/x.csv'
                # print(filenameError)
                if os.path.isdir(filenameError): 
                    # 如果存在errorcode

                    if os.path.exists(filenameX): 
                        filenameEDGE = f'{filenameError}/edge.csv'
                        
                        data_error_now= gen_data_from_csv(filenamex = filenameX,filenameedge = filenameEDGE)
                        allnums = allnums + 1
                        error_code_path = f'{nums}{error_string[j]}'
                        stack_down[error_code_path] = data_error_now


        if len(stack_down) !=0: 
            stack_up[csv_String[i]] = stack_down
         
    
    cos = torch.nn.CosineSimilarity()
    cos = cos.cuda()


    # 遍历stackoverflow error
    for c1 in stack_up :
        for err in stack_up[c1]:
            test_data_now = stack_up[c1][err]

            
            classficate1='nochoose' 
            classficate2='nochoose'
            max_similarity= -1

            x1 = test_data_now.x
            x1 = x1.cuda()
            edge1 = test_data_now.edge_index
            edge1 = edge1.cuda()
            u1 = torch.zeros(x1.shape[0], embedding_size)
            u1 = u1.cuda()


            # 遍历coreset
            for c2 in set_up :
                for err2 in set_up[c2]:
                    for dataset_i in set_up[c2][err2]:
                        # dataset_i 和 test_data_now 做比对

                        x2 = dataset_i.x
                        x2 = x2.cuda()
                        edge2 = dataset_i.edge_index
                        edge2 = edge2.cuda()
                        u2 = torch.zeros(x2.shape[0], embedding_size)
                        u2 = u2.cuda()

                        
                        g1 = GNN1(x1, u1, edge1)
                        g2 = GNN1(x2, u2, edge2)

                        # print(cos(g1,g2))
                        # 判断大小
                        if cos(g1,g2)>max_similarity :
                            # print(classficate1,classficate2,max_similarity)
                            classficate1= c2 
                            classficate2= err2
                            max_similarity = cos(g1,g2)




            print(c1,err,"chose: ", c2,err2)







    # 跟据结果产生代码对
if __name__ == "__main__":
    gen_datas()
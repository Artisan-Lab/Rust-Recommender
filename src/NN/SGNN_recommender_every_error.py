from flowChart import *
import torch
from torch import nn
from torch.nn import Sequential, Linear, ReLU, Tanh
from torch_geometric.nn import MessagePassing
from torch.utils.data import Dataset, DataLoader
from torch.utils.data.dataset import T_co


# 以下SGNN内容


# # GNN9
# random_num = 4
# T = 4  # 图卷积层迭代次数
# 17/27
# 6/13
# 4/9


# GNN10
# random_num = 4
# T = 3  # 图卷积层迭代次数
# learning_rate = 0.001
# epoch = 120 #100基本上就收敛了
# 17/27
# 8/13
# 5/9


# GNN11
# random_num = 3
# T = 3  # 图卷积层迭代次数
# learning_rate = 0.001
# epoch = 80 #100基本上就收敛了
# 18 / 27
# 6 / 13
# 4 / 9
# 0 / 2
# e515 ::
# method_call + return_ref 1542e515 chose:  method_call e515 code  1
# 0 / 1




# GNN12
# random_num = 2
# T = 3  # 图卷积层迭代次数
# learning_rate = 0.001
# epoch = 100 #100基本上就收敛了

# GNN13
# random_num = 4
# T = 3  # 图卷积层迭代次数
# learning_rate = 0.001
# epoch = 100 #100基本上就收敛了


inchannel = 9  # 节点特征维度
embedding_size = 9  # outchannel


random_num = 4
T = 3  # 图卷积层迭代次数
learning_rate = 0.001
epoch = 120 #100基本上就收敛了

# SGNN内容
class ACFGConv(MessagePassing):
    def __init__(self, in_channels, out_channels):
        # in_channels out_channels 输入node特征向量维度，输出node embedding的维度，u是ut,x是节点特征向量
        # 公式为u(t+1) = F(x , sum(ut)) ut belong to N(x)
        super().__init__(aggr="add")  # "Add" aggregation (Step 5).
        self.w1 = torch.nn.Linear(in_channels, out_channels)
        self.sigmod = Sequential(
            Linear(out_channels, out_channels),
            ReLU(),
            Linear(out_channels, out_channels),
            ReLU(),
            Linear(out_channels, out_channels),
            ReLU(),
            Linear(out_channels, out_channels),
            Tanh()
            
        )

    def forward(self, x, u, edge_index):
        # print("----in forward----")
        #
        # print(edge_index)
        # print("x is ", x)
        # print("u is ", u)
        self.x = x

        out = self.propagate(edge_index, x=x, u=u)
        # print("out is ", out)
        return out

    def message(self, u_j):
        # print("----in message----")
        # # x_j has shape [E, out_channels]
        # print("u_j is ", u_j)

        return u_j

    def update(self, inputs):
        x = self.w1(self.x)
        sigmod_all_u = self.sigmod(inputs)
        # print("inputs is", inputs)
        # print("w1x is ", x)
        # print("sigmod_all_u is ", sigmod_all_u)

        result = x + sigmod_all_u

        return result
# SGNN内容
class SiameseNetwork(nn.Module):
    def __init__(self, in_channels, out_channels):
        super(SiameseNetwork, self).__init__()
        self.w2 = Linear(out_channels, out_channels)
        self.in_channels = in_channels
        self.out_channels = out_channels
        self.conv = ACFGConv(in_channels, out_channels)
        self.conv = self.conv.cuda()
        
    def forword_once(self, x, u, edge_index):
        # 初始化u0 = 0

        for t in range(T):
            u = self.conv(x, u, edge_index)
        # 最后图嵌入是所有节点嵌入求和再进过线性矩阵
        # print("u is ", u)
        e = torch.zeros([1, u.shape[0]])
        e = e + 1
        g = torch.matmul(e, u)  # 所有节点嵌入求和
        g = self.w2(g)
        return g

    def forward(self, x1, u1, edge_index1, x2, u2, edge_index2):
        input1 = self.forword_once(x1, u1, edge_index1)
        input2 = self.forword_once(x2, u2, edge_index2)

        return input1, input2

# SGNN内容
class SiameseNetworkRespective(nn.Module):
    def __init__(self, in_channels, out_channels):
        super(SiameseNetworkRespective, self).__init__()
        self.w2 = Linear(out_channels, out_channels)
        self.in_channels = in_channels
        self.out_channels = out_channels
        self.conv = ACFGConv(in_channels, out_channels)

    def forward(self, x, u, edge_index):
        # 初始化u0 = 0

        for t in range(T):
            u = self.conv(x, u, edge_index)
            u = u.cuda()
        # 最后图嵌入是所有节点嵌入求和再进过线性矩阵
        # print("u is ", u)
        e = torch.zeros([1, u.shape[0]])
        e=  e.cuda()
        e = e + 1
        g = torch.matmul(e, u)  # 所有节点嵌入求和
        g = self.w2(g)

        return g



# 数据处理 数据集生成
# 数据在数据集中的一个条目 代表一对代码以及其相似度
class data_item() :
    def __init__(self, data1, data2, similarity):
        self.data1 = data1
        self.data2 = data2
        self.similarity = similarity
# Todo knowledge加入lable


# 获取新的csv

# 对每个errorcode 找到对应的 aug 构建aug
def gen_datas(str_errorcode):
    # 遍历所有的error_code
    # number 是一个str
    # 从新的文件
    
    #
    csv_String = ["func_call",
     "func_call + match + return_ref",
     "func_call + return_ref",
     "loop",
     "match",
     "method_call",
    "method_call + func_call + return_ref",
    "method_call + loop",
    "method_call + loop + return_ref",
    "method_call + match + return_ref",
    "method_call + return_ref",
    "not"]
    # 所有的文件
    error_string = [str_errorcode]





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
                        
                        # 在这里先把aug加进去
                        filenameaug =  f'{filenameError}/{k}/'
                        
                        data_error_now.append(gen_data_from_csv(filenamex = filenameX,filenameedge = filenameEDGE))
                        for augnum in range(0,10):
                            
                            filenameaugx =  f'{filenameError}/{k}/{augnum}x.csv'
                            filenameaugedge =  f'{filenameError}/{k}/{augnum}edge.csv'
                            if os.path.exists(filenameaugx): 
                                data_error_now.append(gen_data_from_csv(filenamex = filenameaugx,filenameedge = filenameaugedge))

                if len(data_error_now) !=0: 
                    set_down[error_string[j]] = data_error_now
        if len(set_down) !=0: 
            set_up[csv_String[i]] = set_down
    
    

    csv_datas = []
    # 首先对每个 data内部的设置成 0.9相似度
    # 对每个 aug也要设置成1

    sim1_nums = 0 # 1
    sim_1_nums = 0 # -1


    for classification1 in set_up :
        for error_codes in set_up[classification1]:
           
            for i in set_up[classification1][error_codes]:
                for j in set_up[classification1][error_codes]: 
                    if i!=j :
                        one_data = data_item(data1=i, data2=j,similarity=1)
                        sim1_nums +=1
                        #print(classification1,error_codes)
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
                            one_data = data_item(data1=i,data2=j,similarity=-1)
                            sim_1_nums+=1
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
                                    one_data = data_item(data1=i,data2=j,similarity=-1)
                                    if random.randint(0,random_num) == 0 : # 抽取部分放进去
                                        sim_1_nums+=1
                                        csv_datas.append(one_data)
   
    print(str_errorcode, sim1_nums,sim_1_nums)
    print(" ")
    return csv_datas
    print(data_nums)    

    # csv_datas = []
    # # 首先对每个 data内部的设置成 0.9相似度
    # # 自己跟自己是 1.0
    # for classification1 in set_up :
    #     for error_codes in set_up[classification1]:
           
    #         for i in set_up[classification1][error_codes]:
    #             for j in set_up[classification1][error_codes]: 
    #                 if i!=j :
    #                     one_data = data_item(data1=i, data2=j,similarity=0.7)
    #                     #print(classification1,error_codes)
    #                     csv_datas.append(one_data)
    #                 else :
    #                     one_data = data_item(data1=i, data2=j,similarity=1.0)


    # # 对不同 error_codes的同类问题设置成 0.6相似度             
    # for c1 in set_up :
    #     # 枚举不同的err
    #     for err in set_up[c1]:
    #         for err2 in set_up[c1]:
    #             # 枚举不同的data
    #             if err!=err2 :
    #                 for i in set_up[c1][err]:
    #                     for j in set_up[c1][err2]:
    #                         one_data = data_item(data1=i,data2=j,similarity=0.3)
    #                         csv_datas.append(one_data)
    

    # import random
    # # 对不同的所有
    # for c1 in set_up :
    #     for c2 in set_up:
    #         if c1 != c2:
    #     # 枚举不同的err
    #             for err in set_up[c1]:
    #                 for err2 in set_up[c2]:
    #                     # 枚举不同的data
    #                         for i in set_up[c1][err]:
    #                             for j in set_up[c2][err2]:
    #                                 one_data = data_item(data1=i,data2=j,similarity=-1)
    #                                 if random.randint(0,2) == 0 : # 抽取部分放进去
    #                                     csv_datas.append(one_data)
    return csv_datas


    # 跟据结果产生代码对


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



import sys

def view_bar(num,total):
    rate =  float(num)/float(total)
    rate_num = int(rate*100)
    r = '\r[%s>%s]%d%%,%d/%d'%("="*rate_num," "*(100-rate_num),rate_num,num,total)
    sys.stdout.write(r)
    sys.stdout.flush()










def Model_TRAIN(dataset, str_errorcode):

    # 先确定目前最远的是哪个
    import os
    model_now =0

    for model_num in range(1,100):
        path_model = f"GNNmodel/GNN_errorcode/{str_errorcode}/GNN{model_num}.pth"
        if os.path.exists(path_model) == False: 
            model_now = model_num
            break
    print("Model Training Now: ",f"GNNmodel/GNN_errorcode/{str_errorcode}/GNN{model_now}.pth")

    dataset_train = dataset

    loss = nn.CosineEmbeddingLoss(margin=0.0)
    
    loss = loss.cuda()

    GNN1 = SiameseNetworkRespective(in_channels=inchannel, out_channels=embedding_size)
    GNN1 = GNN1.cuda()

    # 两个一起
    # GNN2 = SiameseNetworkRespective(in_channels=inchannel, out_channels=embedding_size)
    # GNN2 = GNN1.cuda()
    optim1 = torch.optim.Adam(GNN1.parameters(), lr=learning_rate)
    # optim2 = torch.optim.Adam(GNN2.parameters(), lr=learning_rate)
    cos = torch.nn.CosineSimilarity()
    
    GNN1.train()
    # GNN2.train()
    for i in range(1, epoch+1):
        print(f"epoch:{i}")
        running_loss = 0.0
        count = 0
        recall = 0
        accuracy = 0
        precision = 0
        TP = 0
        TN = 0
        FP = 0
        FN = 0
        F1_max = 0

        data_nums = 0
        for data in dataset_train:
            
            view_bar(data_nums,dataset_train.__len__())
            data_nums += 1
            data1 = data.data1
            data2 = data.data2
            simlarity = data.similarity

            # print(g1, g2, simlarity)
            if simlarity != 0:
                similarity_g1_g2 = torch.tensor([simlarity]).cuda()
            else :
                similarity_g1_g2 = torch.tensor([0]).cuda()
            
            x1 = data1.x
            
            edge_index1 = data1.edge_index
            x2 = data2.x
            edge_index2 = data2.edge_index

            u1 = torch.zeros(x1.shape[0], embedding_size)
            u2 = torch.zeros(x2.shape[0], embedding_size)
            # print(f"x1 is {x1}, x2 is{x2},e1 is{edge_index1},e2 is{edge_index2}")
            # print(f"u is{u}")

            x1 = x1.cuda()
            u1 = u1.cuda()
            x2 = x2.cuda()
            u2 = u2.cuda()
            edge_index1 = edge_index1.cuda()
            edge_index2 = edge_index2.cuda()


            g1 = GNN1(x1, u1, edge_index1)
            g1 = g1.cuda()
            
            #g2 = GNN2(x2, u2, edge_index2)
            g2 = GNN1(x2, u2, edge_index2)
            g2 = g2.cuda()

            result_loss = loss(g1, g2, similarity_g1_g2)
            result_loss = result_loss.cuda()
            optim1.zero_grad()
            #optim2.zero_grad()
            result_loss.backward()
            optim1.step()
            #optim2.step()

            # print(f"g1 is {g1},g2 is{g2},loss is {result_loss}")

            running_loss = running_loss + result_loss
            count = count + 1
        print("训练集loss", running_loss.item() / count, "轮数", count)

        # if TP == 0: 
        #     TP = TP + 1
        # if TP + FN == 0:
        #     TP = TP + 1

        # precision = TP / (TP + FP)
        # recall = TP / (TP + FN)
        # F1 = 2 * precision * recall / (precision + recall)
        # if F1 > F1_max:
        #     F1_max = F1
        # print("训练集F1 score", F1)
        # print("训练集F1 max score", F1_max)
        # print("训练集召回率", TP / (TP + FN))
        # print("训练集精确率", TP / (TP + FP))
        # print("训练集准确率", (TP + TN) / (TP + TN + FP + FN))

    # -------------------------------------------------------------------------
    torch.save(GNN1, f"GNNmodel/GNN_errorcode/{str_errorcode}/GNN{model_now}.pth")




    




if __name__ == "__main__":

    torch.cuda.set_device(1)
    if torch.cuda.is_available() :
        print("cuda available")
    if torch.cuda.device_count()>1:
        print("显卡数量：",torch.cuda.device_count())

    print(torch.cuda.current_device())  


    # 
    # 别忘了下次训练加回去
    error_string = ["e382","e499","e502","e505","e506","e507","e515","e597"]

    for i in error_string:

        dataset_train = gen_datas(i)
        



        # print(torch.cuda.get_device_name(1))
        # Model_TRAIN(dataset_train, i) # 遍历每一个 然后生成





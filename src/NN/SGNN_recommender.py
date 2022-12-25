from flowChart import *
import torch
from torch import nn
from torch.nn import Sequential, Linear, ReLU, Tanh
from torch_geometric.nn import MessagePassing
from torch.utils.data import Dataset, DataLoader
from torch.utils.data.dataset import T_co


# 以下SGNN内容

inchannel = 5  # 节点特征维度
embedding_size = 4  # outchannel
T = 4  # 图卷积层迭代次数
learning_rate = 0.05
epoch = 100 #100基本上就收敛了

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
        # 最后图嵌入是所有节点嵌入求和再进过线性矩阵
        # print("u is ", u)
        e = torch.zeros([1, u.shape[0]])
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
def datas_knowledge_base(Num):
    N = Num
    datas = []
    for i in range(N):
        datas.append(gen_data_from_csv(str(i+1)))
    return datas


# 生成的是训练集 这里的是读取训练集CSV
def data_process_read_from_csv_TRAIN(Num):
    # 当前csv几个 N就是几个 目前5个
    N = Num
    # 保存x/edge 的datas
    datas = []
    for i in range(N):
        # print(gen_data_from_csv(str(i+1)))
        datas.append(gen_data_from_csv(str(i+1)))
    # print(data[0].x)

    # data生成处理
    dataset = dataset_generate(datas)
    # 返回数据集
    return dataset
    
# 对datas做一个处理
def dataset_generate(datas):
    dataset = []
    N = len(datas)
    # 生成相似代码对 第3个是similarity
    # Todo augmentation
    for i in range(N):
        dataset.append(data_item(data1= datas[i], data2=datas[i],similarity=1))
    # 生成不相似代码对 粗略生成
    for i in range(N):
        for j in range(N):
            if i < j :
                dataset.append(data_item(data1= datas[i], data2=datas[j],similarity=0))
    
    # for data in dataset:
    #     print(data.data1)

    return dataset

def Model_TRAIN(dataset, N):
    dataset_train = dataset
    loss = nn.CosineEmbeddingLoss(margin=0.5)
    GNN1 = SiameseNetworkRespective(in_channels=inchannel, out_channels=embedding_size)
    GNN2 = SiameseNetworkRespective(in_channels=inchannel, out_channels=embedding_size)
    optim1 = torch.optim.Adam(GNN1.parameters(), lr=learning_rate)
    optim2 = torch.optim.Adam(GNN2.parameters(), lr=learning_rate)
    cos = torch.nn.CosineSimilarity()

    GNN1.train()
    GNN2.train()
    for i in range(epoch):
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
        for data in dataset_train:

            data1 = data.data1
            data2 = data.data2
            simlarity = data.similarity

            # print(g1, g2, simlarity)
            if simlarity == 1:
                similarity_g1_g2 = torch.tensor([1])
            else:
                similarity_g1_g2 = torch.tensor([-1])
            x1 = data1.x
            edge_index1 = data1.edge_index
            x2 = data2.x
            edge_index2 = data2.edge_index

            u1 = torch.zeros(x1.shape[0], embedding_size)
            u2 = torch.zeros(x2.shape[0], embedding_size)
            # print(f"x1 is {x1}, x2 is{x2},e1 is{edge_index1},e2 is{edge_index2}")
            # print(f"u is{u}")
            g1 = GNN1(x1, u1, edge_index1)
            g2 = GNN2(x2, u2, edge_index2)

            result_loss = loss(g1, g2, similarity_g1_g2)
            optim1.zero_grad()
            optim2.zero_grad()
            result_loss.backward()
            optim1.step()
            optim2.step()

            if (similarity_g1_g2 > 0.5):
                # print(f"simlarity is {similarity_g1_g2},cos is {cos(g1, g2)}")
                recall = recall + 1
                if (cos(g1, g2) > 0.5):
                    TP = TP + 1
                else:
                    FN = FN + 1
            else:
                if (cos(g1, g2) > 0.5):
                    FP = FP + 1
                else:
                    TN = TN + 1

            # print(f"g1 is {g1},g2 is{g2},loss is {result_loss}")

            running_loss = running_loss + result_loss
            count = count + 1
        print("训练集loss", running_loss.item() / count, "轮数", count)
        if TP + FN == 0:
            TP = TP + 1
        precision = TP / (TP + FP)
        recall = TP / (TP + FN)
        # F1 = 2 * precision * recall / (precision + recall)
        # if F1 > F1_max:
        #     F1_max = F1
        # print("训练集F1 score", F1)
        # print("训练集F1 max score", F1_max)
        print("训练集召回率", TP / (TP + FN))
        print("训练集精确率", TP / (TP + FP))
        print("训练集准确率", (TP + TN) / (TP + TN + FP + FN))

    # -------------------------------------------------------------------------
    ### TEST_SET TOdo 目前拿训练集加进去做个测试
    test_set = datas_knowledge_base(N)
    knowledge_set = datas_knowledge_base(N)
    # 测试集

    best_choose = []
    best_cos = []
    for test in test_set :
        
        candidate = []

        x1 = test.x
        edge1 = test.edge_index
        u1 = torch.zeros(x1.shape[0], embedding_size)

        
        # 知识库
        for knowledge in knowledge_set :
            x2 = knowledge.x
            edge2 = knowledge.edge_index
            u2 = torch.zeros(x2.shape[0], embedding_size)

            g1 = GNN1(x1, u1, edge1)
            g2 = GNN2(x2, u2, edge2)

            candidate.append(cos(g1,g2))

        # 取出cos值最大的 找到下标
        max_cos = -0.1
        max_index = -1
        for j in range(len(candidate)):
            value = candidate[j]
            if value > max_cos:
                max_cos = value
                max_index = j

        best_choose.append(max_index)
        best_cos.append(max_cos)
    # 根据Knowledge中的x edge 获取最大cos
    
    # 打印结果
    print("#############################")
    for i in range(len(test_set)) :
        print(i," choose ",best_choose[i], " whose best_cos is: ",best_cos[i])
    print("#############################")




if __name__ == "__main__":
    dataset_train = data_process_read_from_csv_TRAIN(5)
    Model_TRAIN(dataset_train, 5)

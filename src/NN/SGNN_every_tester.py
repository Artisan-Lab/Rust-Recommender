# 读取stackoverflow内全部内容
# 用生成的 x edge 反向测评


from SGNN_recommender_every_error import *
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


# 计算每一个匹配的时间
time_cost = dict()
# 当前计算到了哪一个code
time_cost_num = 0



def gen_datas(str_errorcode):
    # number 是一个str
    # 从新的文件
    


    
    
    #
    csv_String = ["func_call", "func_call + match + return_ref","func_call + return_ref","loop","match","method_call",
    "method_call + func_call + return_ref","method_call + func_call",
    "method_call + loop","method_call + loop + return_ref","method_call + match + return_ref","method_call + return_ref","not"]
    # 所有的文件
    error_string = [str_errorcode]





    filenamebase = f'spider_stackoverflow/classification2/createnew/'
    test_stackoverlow = f'spider_stackoverflow/classification2/stackoverflow/'
    # filenameedge = f'spider_stackoverflow/classification2/createnew/'

    import os


    # 读取 GNNmodel

    # 测试哪一版的？


    GNN1 = torch.load(f'GNNmodel/GNN_errorcode/{str_errorcode}/GNN_B.pth')


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
                        
                        error_name_with_num = f'{nums}{error_string[j]}'

                        time_cost[error_name_with_num] = 0

                        data_error_now= gen_data_from_csv(filenamex = filenameX,filenameedge = filenameEDGE)
                        allnums = allnums + 1
                        error_code_path = f'{nums}{error_string[j]}'
                        stack_down[error_code_path] = data_error_now

        # 获取了所有的
        if len(stack_down) !=0: 
            stack_up[csv_String[i]] = stack_down
         
    
    cos = torch.nn.CosineSimilarity()
    cos = cos.cuda()


    all_right = 0
    all_stack = 0

    import time 

    # 遍历stackoverflow error
    for c1 in stack_up :
        for err in stack_up[c1]:
            test_data_now = stack_up[c1][err]

            
            start_time = time.perf_counter()

            classficate1='nochoose' 
            classficate2='nochoose'
            max_similarity= -1
            number_class = 0

            x1 = test_data_now.x
            x1 = x1.cuda()
            edge1 = test_data_now.edge_index
            edge1 = edge1.cuda()
            u1 = torch.zeros(x1.shape[0], embedding_size)
            u1 = u1.cuda()
            g1 = GNN1(x1, u1, edge1)

            # 遍历coreset
            # 建一个 dict 获取时间

            # 计算遍历输出时间 

            for c2 in set_up :
                for err2 in set_up[c2]:

                    i_now = 0

                    for dataset_i in set_up[c2][err2]:
                        # dataset_i 和 test_data_now 做比对
                        # 提取后面的四位error代码
                        i_now +=1 
                        last_four = err[len(err)-4:] 
                        if err2 == last_four:
                            x2 = dataset_i.x
                            x2 = x2.cuda()
                            edge2 = dataset_i.edge_index
                            edge2 = edge2.cuda()
                            u2 = torch.zeros(x2.shape[0], embedding_size)
                            u2 = u2.cuda()

                            
                            
                            g2 = GNN1(x2, u2, edge2)
                            # print(g1)
                            # print(cos(g1,g2))
                            # 判断大小
                            if cos(g1,g2)>max_similarity :
                                # print(classficate1,classficate2,max_similarity)


                                number_class = i_now
                                classficate1= c2 
                                classficate2= err2
                                max_similarity = cos(g1,g2)



            
            print(c1,err,"chose: ", classficate1,classficate2,"code ",number_class)
            if c1 == classficate1 :
                all_right +=1
            all_stack += 1

            end_time = time.perf_counter()

            elapsed_time = (end_time - start_time) * 1000

            time_cost[err] = elapsed_time


            # 在这里计算时间
    
    print(all_right,"/",all_stack)



    # 跟据结果产生代码对
if __name__ == "__main__":
    torch.cuda.set_device(1)


    error_string = ["e597","e382","e499","e502","e505","e506","e507","e515"]

    for i in error_string:
        #print(i,"::")
        gen_datas(i)
       # print()
        #print()

   # print(time_cost)
    



    # loss = nn.CosineEmbeddingLoss(margin=0.0)
    
    # loss = loss.cuda()

    # a = torch.tensor([[0,1]])
    # b =  torch.tensor([[1,1]])
    # cosine embeddingloss只能用y-算
    # out = loss(a,b,torch.tensor([0.1]))
    # print(out)


    # print(all_right,"/",all_stack)




# 时间表 根据这个画图 单位ms
# {"3176e382": 6.028, "3164e507": 8.085, "800e502": 9.517, "653e502": 5.79, "1585e382": 9.65, "2271e499": 16.331, "2436e502": 9.751, "2299e597": 6.472, "691e505": 9.197, "1132e382": 4.836, "2739e507": 4.272, "1006e506": 5.014, "2469e502": 5.255, "2724e499": 12.165, "2668e502": 3.978, "2356e502": 9.379, "3269e506": 4.493, "2756e499": 6.526, "2687e499": 13.185, "2688e502": 8.447, "2472e499": 4.001, "1167e507": 7.729, "3213e502": 4.937, "2445e499": 4.098, "674e499": 7.939, "608e502": 3.701, "1971e382": 7.32, "363e502": 3.07, "131e502": 3.696, "96e502": 5.285, "15e502": 6.767, "2334e502": 6.186, "3045e502": 7.961, "743e506": 4.196, "1003e502": 14.043, "2220e382": 4.023, "1068e382": 4.501, "2439e382": 6.471, "876e597": 14.966, "2409e502": 4.648, "798e499": 8.989, "3101e502": 4.98, "123e502": 11.806, "2265e502": 3.971, "3465e502": 9.951, "376e507": 10.261, "299e502": 2.813, "1378e382": 8.402, "192e597": 4.735, "287e506": 2.573, "2601e502": 6.549, "3048e382": 5.85, "567e499": 4.801, "863e505": 9.281, "2580e502": 9.955, "2883e502": 7.315, "2515e502": 14.191, "530e382": 4.196, "354e506": 3.439, "1203e382": 8.314, "1542e515": 5.104, "3348e382": 9.965, "3211e382": 3.298, "2321e502": 7.505}
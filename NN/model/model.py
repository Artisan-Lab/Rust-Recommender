import sys

sys.path.append(r'G:\Asset\Programs\Python3\NNDL\HW1\Rust-unsafe-to-safe-code-retrivial')

import torch
from torch import nn
from torch.nn import Sequential, Linear, ReLU

from torch.utils.data import Dataset, DataLoader
from torch.utils.data.dataset import T_co
import pandas
import numpy



class SiameseNetwork(nn.Module):
    def __init__(self,dataset1):
        super(SiameseNetwork, self).__init__()
        used = dataset1.used_days_num
        pred = dataset1.pred_days_num

        self.model1 = Sequential(
            Linear(used,used),
            ReLU(),
            Linear(used,used),
            ReLU(),
            # Linear(used,used),
            # ReLU(),
            Linear(used,pred)
        )

    def forward(self,input):
        input = self.model1(input)
        return input



def main():
    #--------用于测试--------------
    dataset1 =MyData(data_path,300,1)
    dataloader1 = DataLoader(dataset1,batch_size=1,num_workers=0)
    loss = nn.MSELoss()
    DNN1 = myDNN(dataset1)
    optim = torch.optim.Adam(DNN1.parameters(),lr=0.005)

    for epoch in range(20):
        running_loss = 0.0
        for data in dataloader1:

            # print(type(data))
            used,pred = data
            used = used.to(torch.float32)
            pred = pred.to(torch.float32)
            # print('used:',used)
            # print('pred:',pred)
            # print(type(data),type(used),type(pred))
            # print(used.dtype,pred.dtype)

            outputs = DNN1(used)
            result_loss = loss(outputs,pred)
            optim.zero_grad()
            result_loss.backward()
            optim.step()
            running_loss =  running_loss + result_loss
        print('-------------------')
        print('---epoch:',epoch,'---')
        print('\tloss is :',running_loss)
        print('------------------')



    #win下只有主进程working，多进程程序会down掉，所以num_workers=0


    print('this message is from main function')


if __name__ == '__main__':
    print(__name__)
    main()

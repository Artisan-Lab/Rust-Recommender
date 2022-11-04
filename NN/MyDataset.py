import os.path as osp
import os
from collections.abc import Sequence
from typing import Any, List
import torch
from torch_geometric.data import Dataset
from data_augmentation.del_file import del_pre_file
from data_augmentation.augmentation import add_noise
from flowChart import *
import random

test_flag = False
sample_path = r".\database\samples\knowledge.pt"
test_path = r".\database\test\test_set.pt"


class CodePair(object):
    """api label is the unique index of CodePair"""

    def __init__(self, u=None, s=None, al=None, an=None, ui=None):
        self.unsafe_code = u
        self.safe_code = s
        self.api_label = al
        self.api_name = an
        self.acfg = None
        self.unsafe_index = ui

    def get_unsafe_code(self):
        return self.unsafe_code

    def get_safe_code(self):
        return self.safe_code

    def get_api_label(self):
        return self.api_label

    def get_api_name(self):
        return self.api_name

    def set_unsafe_code(self, x):
        self.unsafe_code = x

    def set_safe_code(self, x):
        self.safe_code = x

    def set_api_label(self, x):
        self.api_label = x

    def set_api_name(self, x):
        self.api_name = x

    def contain_none(self):
        if self.get_api_label() is None:
            return True
        if self.get_api_name() is None:
            return True
        if self.get_safe_code() is None:
            return True
        if self.get_unsafe_code() is None:
            return True
        return False

    def show(self, index=0):
        print(f"------------CodePair {index}------------")
        print(f"-api_name is {self.api_name}")
        print(f"-api_label is {self.api_label}")
        print(f"-unsafe code is {self.unsafe_code}")
        print(f"-  safe code is {self.safe_code}")
        print(f"-unsafe acfg is {self.acfg}")
        # print("-------------------------------")


class Result(object):
    def __init__(self, cp1: CodePair, cp2: CodePair, cos_sim):
        self.cp1 = cp1
        self.cp2 = cp2
        self.cos_sim = cos_sim

# 存的是知识库的东西
class CodePairs(object):
    pair_list: list[CodePair]

    def __init__(self):
        self.pair_list = []  # 存放CodePair的列表  
        self.api_label_list = []
        self.api_name_list = []  # 123
        self.all_coeds = []
        self.all_api_label = []
        self.all_api_name = []  # 246
        self.api_name_kind_list = []
    # 每次读一对unsafe和safe
    def append(self, unsafe_code, safe_code, api_label, api_name):
        cp = CodePair(unsafe_code, safe_code, api_label, api_name)
        index, f = self.contain_append(cp)
        if not f:
            self.api_name_list.append(api_name)
            self.api_label_list.append(api_label)
            # print("api_name is ", api_name)
            # print(api_name not in self.api_name_kind_list)
            if api_name not in self.api_name_kind_list:
                self.api_name_kind_list.append(api_name)
    # 配对读取unsafe safe
    def contain_append(self, cp: CodePair):
        # 找到了相同api_label的合并，未找到的添加
        if len(self.pair_list) == 0:
            self.pair_list.append(cp)
            return -1, False
        for i in range(len(self.pair_list)):
            temp = self.pair_list[i]
            label = temp.get_api_label()
            if cp.get_api_label() == label:
                if temp.get_safe_code() is None:
                    temp.set_safe_code(cp.get_safe_code())
                if temp.get_unsafe_code() is None:
                    temp.set_unsafe_code(cp.get_unsafe_code())
                if temp.contain_none():
                    print("ERROR:in contain_append")
                return i, True
        self.pair_list.append(cp)
        return -1, False
# 生成scfg
    def gen_acfg(self):
        for item in self.pair_list:
            afcg = gen_acfg_from_txt(item.unsafe_code)
            item.acfg = afcg

    def get_list_by_api(self, api_name):
        ans = []
        for item in self.pair_list:
            if item.api_name == api_name:
                ans.append(item)
        return ans

    def show(self):
        index = 0
        for item in self.pair_list:
            item.show(index)
            index = index + 1


class CodePairsTest(CodePairs):
    def __init__(self):
        super().__init__()
        self.unsafe_codes = []  # 这三个跟着所有的unsafe code走
        self.unsafe_api_label = []
        self.unsafe_api_name = []
        self.unsafe_index = []  # 代码对的唯一id

    def contain_append(self, cp: CodePair):
        # 找到了相同api_label的合并，未找到的添加
        if len(self.pair_list) == 0:
            self.pair_list.append(cp)
            return -1, False
        for j in range(len(self.pair_list)):
            temp = self.pair_list[j]
            label = temp.unsafe_index
            if cp.unsafe_index == label:
                if temp.get_safe_code() is None:
                    temp.set_safe_code(cp.get_safe_code())
                if temp.get_unsafe_code() is None:
                    temp.set_unsafe_code(cp.get_unsafe_code())
                if temp.contain_none():
                    print("ERROR:in contain_append")
                return j, True
        self.pair_list.append(cp)
        return -1, False
    
    def append(self, unsafe_code, safe_code, api_label, api_name, unsafe_index):
        cp = CodePair(unsafe_code, safe_code, api_label, api_name, unsafe_index)
        index, f = self.contain_append(cp)
        if not f:
            self.unsafe_index.append(unsafe_index)
            self.api_name_list.append(api_name)
            self.api_label_list.append(api_label)
            # print("api_name is ", api_name)
            # print(api_name not in self.api_name_kind_list)
            if api_name not in self.api_name_kind_list:
                self.api_name_kind_list.append(api_name)

# 从知识库构建的数据集
class MyOwnDataset(Dataset):
    """
    主要逻辑在process函数
    """

    def __init__(self, root, transform=None, pre_transform=None, pre_filter=None):
        super().__init__(root, transform, pre_transform, pre_filter)

        self.raw_paths;  # 原始数据的各个文件名
        self.processed_dir;

    @property
    def raw_dir(self) -> str:
        # 指定数据集所在文件夹std,std下都是文件夹，所以raw_path是文件夹
        if test_flag:
            print("-----this is raw_dir----")
            print(f"self.raw_dir is {osp.join(self.root, 'std')}")
        return osp.join(self.root, 'std')

    @property
    def raw_file_names(self):
        r"""The name of the files in the :obj:`self.raw_dir` folder that must
                be present in order to skip downloading."""
        # for name in os.listdir
        if test_flag:
            print("-----this is raw_file_names----")
            print(f"raw_file_names is {os.listdir(self.raw_dir)}")

        return os.listdir(self.raw_dir)

    @property
    def raw_paths(self) -> List[str]:
        r"""The absolute filepaths that must be present in order to skip
        downloading."""
        # 由于std下都是文件夹，故raw_paths列表存放的都是文件夹名
        # 如Rust-unsafe-to-safe-code-retrivial\\database\\std\\array_assume_init
        files = to_list(self.raw_file_names)
        if test_flag:
            print("-----this is raw_paths----")
            print(f"raw_paths is {[osp.join(self.raw_dir, f) for f in files]}")
        return [osp.join(self.raw_dir, f) for f in files]

    @property
    def processed_dir(self) -> str:
        if test_flag:
            print("-----this is processed_dir----")
            print(f"processed_dir is {osp.join(self.root, 'processed')}")
        return osp.join(self.root, 'processed')

    @property
    def processed_file_names(self):
        r"""The name of the files in the :obj:`self.processed_dir` folder that
                must be present in order to skip processing."""
        # self.process()
        if test_flag:
            print("-----this is processed_file_names----")
            print(f"processed_file_names is {os.listdir(self.processed_dir)}")
        return os.listdir(self.processed_dir)

    @property
    def processed_paths(self) -> List[str]:
        r"""The absolute filepaths that must be present in order to skip
        processing."""

        files = to_list(self.processed_file_names)
        if test_flag:
            print("-----this is processed_paths----")
            print(f"processed_paths is {[osp.join(self.processed_dir, f) for f in files]}")
        return [osp.join(self.processed_dir, f) for f in files]

    # def download(self):
    #     # Download to `self.raw_dir`.
    #     path = download_url(url, self.raw_dir)
    #     ...

    def minDistance(self, word1: str, word2: str) -> int:
        n = len(word1)
        m = len(word2)

        # 有一个字符串为空串
        if n * m == 0:
            return n + m

        # DP 数组
        D = [[0] * (m + 1) for _ in range(n + 1)]

        # 边界状态初始化
        for i in range(n + 1):
            D[i][0] = i
        for j in range(m + 1):
            D[0][j] = j

        # 计算所有 DP 值
        for i in range(1, n + 1):
            for j in range(1, m + 1):
                left = D[i - 1][j] + 1
                down = D[i][j - 1] + 1
                left_down = D[i - 1][j - 1]
                if word1[i - 1] != word2[j - 1]:
                    left_down += 1
                D[i][j] = min(left, down, left_down)

        return D[n][m]
    # 具体逻辑 
    def process(self):
        r"""
        1.由于std下都是文件夹，故raw_paths列表存放的都是文件夹名，所以process要将文件夹展开至文件
        2.在构造训练集和验证集时，也可以构造知识库和测试集
        3.训练集：train = processed[0:4000]
            i)data=(acfg1,acfg2,sim,code1,code2,api_label1,api_name1,api_label2,api_name2)
            a)200份code与除自身外随机不重复的10份代码构成不相似代码对，其相似度为0
            b)200份code与自身以及9份代码（数据增强）构成相似代码对，其相似度为1
            c)数据增强即在main函数中加入一条噪音语句，如assert!(1,1)
            d)2000份相似代码对与2000份不相似代码对共同构成训练集(共4000份带有相似性标签的代码对)
        4.验证集：validation = processed[4000:4919]
            a)46份code与除自身外随机不重复的10份代码构成不相似代码对，其相似度为0
            b)46份code与自身以及9份代码（数据增强）构成相似代码对，其相似度为1
            c)数据增强即在main函数中加入一条噪音语句，如assert!(1,1)
            d)460份相似代码对与460份不相似代码对共同构成训练集(共920份带有相似性标签的代码对)
        4.知识库：samples
            i)data=(acfg_unsafe,code1,code2,api_label,api_name)
            a)118对unsafe safe代码对的文本、unsafe code的ACFG、API标签构成
            b)文本用于推荐，ACFG用于相似性计算，API标签用于测试集验证算法
        5.测试集：test
            i)data=(acfg_unsafe,code_unsafe,api_label,api_name)
            a)由github文件夹下34份unsafe code的文本、ACFG、匹配API的标签构成
        """
        if test_flag:
            print("-----this is process----")
        idx = 0
        # for raw_path in self.raw_paths:
        #   print(f"raw_path is {raw_path}")
        # print(f"raw_paths is {self.raw_paths}")
        api_list = self.raw_paths

        api_labels = []
        code_pairs = CodePairs()
        # -------读取std下所有代码对,形成samples.knowledge.pt---------
        for api in api_list:
            # 所有api文件夹
            # print(f"api is {api}")
            cur_dir = api.rsplit("\\", 1)
            api_name = cur_dir[-1]
            # print(f"cur_dir is {cur_dir}")
            # print(f"api_name is {api_name}")
            for dir in os.listdir(api):
                # 所有unsafe safe文件夹
                # print(f"dir is {dir}")
                dir_join = os.listdir(osp.join(api, dir))
                # print(f"dir_join is {dir_join}")

                for num in dir_join:
                    # unsafe safe文件夹下的所有文件
                    # print(osp.join(api,dir))
                    # print(f"num is {num}")
                    cur_num = num.rsplit(".", 1)
                    # print(f"cur_num is {cur_num}")
                    api_label = api_name + "_" + cur_num[0]
                    # print(f"api_label is {api_label}")
                    if not api_label in api_labels:
                        api_labels.append(api_label)
                    filename = osp.join(api, dir)
                    # print(f"filename is {filename}")
                    f1 = open(osp.join(filename, num), 'r', encoding='utf_8')
                    txt1 = ""
                    for line in f1:
                        txt1 = txt1 + line.strip() + "\n"
                    f1.close()
                    if "unsafe" in dir:
                        code_pairs.append(unsafe_code=txt1, safe_code=None, api_name=api_name, api_label=api_label)

                    elif "safe" in dir:
                        code_pairs.append(unsafe_code=None, safe_code=txt1, api_name=api_name, api_label=api_label)
                    code_pairs.all_coeds.append(txt1)
                    code_pairs.all_api_name.append(api_name)
                    code_pairs.all_api_label.append(api_label)

                pass
            pass
        pass

        # print(f"api_labels is {api_labels}")
        code_pairs.gen_acfg()
        # code_pairs.show()
        # 把std代码对存放到sample_path中
        torch.save(code_pairs, sample_path)

        all_codes = code_pairs.all_coeds
        max_len = len(all_codes)
        # print(max_len)
        mid = 200
        train_code = all_codes[0:mid]
        val_code = all_codes[mid:max_len]
        # print(len(train_code))
        # print(len(val_code))
        # ---------生成训练集-------------
        for j in range(len(train_code)):
            cur_code = train_code[j]
            ran = []
            diff_size = 10
            while len(ran) <= diff_size:
                temp = random.randint(0, len(train_code) - 1)
                if temp not in ran and temp != j:
                    ran.append(temp)
            # 随机10份不同的代码
            for k in range(diff_size):
                index = ran[k]
                diff_code = train_code[index]
                sim = 0
                acfg1 = gen_acfg_from_txt(cur_code)
                acfg2 = gen_acfg_from_txt(diff_code)
                name1 = code_pairs.all_api_name[j]
                name2 = code_pairs.all_api_name[index]
                label1 = code_pairs.all_api_label[j]
                label2 = code_pairs.all_api_label[index]
                data = [acfg1, acfg2, sim, cur_code, diff_code, label1, name1, label2, name2]
                torch.save(data, osp.join(self.processed_dir, f'data_{idx}.pt'))
                idx += 1
            # 10份相似代码
            noise_list = add_noise(cur_code, diff_size)
            for k in range(diff_size):
                noise_code = noise_list[k]
                sim = 1
                acfg1 = gen_acfg_from_txt(cur_code)
                acfg2 = gen_acfg_from_txt(noise_code)
                name1 = code_pairs.all_api_name[j]
                label1 = code_pairs.all_api_label[j]
                # acfg1：unsafe(其实safe也有)代码的tensor acfg2：unsafe代码的相似代码 sim：相似度0/1 curcode：代码字符串 noisecode：代码字符串
                data = [acfg1, acfg2, sim, cur_code, noise_code, label1, name1, label1, name1]
                # label1: api名字下的第几份代码代码的索引 name：api名字
                torch.save(data, osp.join(self.processed_dir, f'data_{idx}.pt'))
                idx += 1

        # ---------生成验证集-------------
        for j in range(len(val_code)):
            cur_code = val_code[j]
            ran = []
            diff_size = 10
            while len(ran) <= diff_size:
                temp = random.randint(0, len(val_code) - 1)
                if temp not in ran and temp != j:
                    ran.append(temp)
            # 随机10份不同的代码
            for k in range(diff_size):
                index = ran[k]
                diff_code = val_code[index]
                sim = 0
                acfg1 = gen_acfg_from_txt(cur_code)
                acfg2 = gen_acfg_from_txt(diff_code)
                name1 = code_pairs.all_api_name[j]
                name2 = code_pairs.all_api_name[index]
                label1 = code_pairs.all_api_label[j]
                label2 = code_pairs.all_api_label[index]
                data = [acfg1, acfg2, sim, cur_code, diff_code, label1, name1, label2, name2]
                torch.save(data, osp.join(self.processed_dir, f'data_{idx}.pt'))
                idx += 1
            # 10份相似代码
            noise_list = add_noise(cur_code, diff_size)
            for k in range(diff_size):
                noise_code = noise_list[k]
                sim = 1
                acfg1 = gen_acfg_from_txt(cur_code)
                acfg2 = gen_acfg_from_txt(noise_code)
                name1 = code_pairs.all_api_name[j]
                label1 = code_pairs.all_api_label[j]
                data = [acfg1, acfg2, sim, cur_code, noise_code, label1, name1, label1, name1]
                torch.save(data, osp.join(self.processed_dir, f'data_{idx}.pt'))
                idx += 1

        # for raw_path in self.raw_paths:
        #     for r2 in self.raw_paths:
        #         # Read data from `raw_path`.
        #         # raw_path 就是 rs文件名
        #         f1 = open(raw_path)
        #         txt1 = ""
        #         for line in f1:
        #             txt1 = txt1 + line.strip()
        #         f1.close()
        #
        #         f2 = open(r2)
        #         txt2 = ""
        #         for line in f2:
        #             txt2 = txt2 + line.strip()
        #         f2.close()
        #         d = self.minDistance(txt1, txt2)
        #         simlarity = 1 / (d + 1)
        #         cfg1 = gen_data_from_rs(raw_path)
        #         cfg2 = gen_data_from_rs(r2)
        #         data = [cfg1, cfg2, simlarity, txt1, txt2]
        #         # data = gen_data_from_rs(raw_path)
        #         print(f"data is{data}")
        #         if simlarity == 1.0:
        #             for j in range(1):
        #                 torch.save(data, osp.join(self.processed_dir, f'data_{idx}.pt'))
        #                 idx += 1
        #
        #         if self.pre_filter is not None and not self.pre_filter(data):
        #             continue
        #
        #         if self.pre_transform is not None:
        #             data = self.pre_transform(data)
        #
        #         torch.save(data, osp.join(self.processed_dir, f'data_{idx}.pt'))
        #         idx += 1

        # for raw_path in self.raw_paths:
        #     for j in range(10):
        #         # Read data from `raw_path`.
        #         # raw_path 就是 rs文件名
        #         f1 = open(raw_path)
        #         txt1 = ""
        #         for line in f1:
        #             txt1 = txt1 + line.strip()
        #         f1.close()
        #
        #         simlarity = 1
        #         cfg1 = gen_data_from_rs(raw_path)
        #
        #         data = [cfg1,cfg1,simlarity]
        #     # data = gen_data_from_rs(raw_path)
        #         print(f"data is{data}")
        #
        #         if self.pre_filter is not None and not self.pre_filter(data):
        #             continue
        #
        #         if self.pre_transform is not None:
        #             data = self.pre_transform(data)
        #
        #         torch.save(data, osp.join(self.processed_dir, f'data_{idx}.pt'))
        #         idx += 1

        return

    def len(self):
        return len(self.processed_file_names)

    def get(self, idx):
        data = torch.load(osp.join(self.processed_dir, f'data_{idx}.pt'))
        return data


def to_list(value: Any) -> Sequence:
    if isinstance(value, Sequence) and not isinstance(value, str):
        return value
    else:
        return [value]


def gen_test_set():
    """
        5.测试集：test
            i)data=(acfg_unsafe,code_unsafe,api_label,api_name)
            a)由github文件夹下34份unsafe code的文本、ACFG、匹配API的标签构成
    """
    api_list = r"G:\Asset\Programs\Python3\NNDL\HW1\Rust-unsafe-to-safe-code-retrivial\database\github"
    code_pairs = CodePairsTest()
    for api in os.listdir(api_list):
        # 所有api文件夹
        # print(f"api is {api}")
        api_path = osp.join(api_list, api)
        cur_dir = api_path.rsplit("\\", 1)
        api_label = cur_dir[-1]
        api_name = ""
        for z in range(len(api_label) - 1, -1, -1):
            if api_label[z] == "_":
                api_name = api_name + api_label[0:z]
                break

        # print(f"cur_dir is {cur_dir}")
        # print(f"api_name is {api_name}")
        # print(f"api_label is {api_label}")

        for dir in os.listdir(api_path):
            # 所有unsafe safe文件夹
            # print(f"dir is {dir}")
            dir_join = os.listdir(osp.join(api_path, dir))
            # print(f"dir_join is {dir_join}")

            for num in dir_join:
                # unsafe safe文件夹下的所有文件
                # print(osp.join(api_path, dir))
                # print(f"num is {num}")
                unsafe_index = num

                filename = osp.join(api_path, dir)
                # print(f"filename is {filename}")
                f1 = open(osp.join(filename, num), 'r', encoding='utf_8')
                txt1 = ""
                for line in f1:
                    txt1 = txt1 + line.strip() + '\n'
                f1.close()
                # print(txt1)
                if "unsafe" in dir:
                    code_pairs.append(unsafe_code=txt1, safe_code=None, api_name=api_name, api_label=api_label,
                                      unsafe_index=unsafe_index)
                    code_pairs.unsafe_codes.append(txt1)  # 这三个跟着所有的unsafe code走
                    code_pairs.unsafe_api_label.append(api_label)
                    code_pairs.unsafe_api_name.append(api_name)
                elif "safe" in dir:
                    code_pairs.append(unsafe_code=None, safe_code=txt1, api_name=api_name, api_label=api_label,
                                      unsafe_index=unsafe_index)
                code_pairs.all_coeds.append(txt1)
                code_pairs.all_api_name.append(api_name)
                code_pairs.all_api_label.append(api_label)
            pass
        pass
    pass
    code_pairs.gen_acfg()
    # 
    torch.save(code_pairs, test_path)
    # code_pairs.show()


def init_dataset(root):
    dataset = MyOwnDataset(root)
    pass


if __name__ == "__main__":
    # root = r"G:\Asset\Programs\Python3\NNDL\HW1\Rust-unsafe-to-safe-code-retrivial\test_dataset\boxed"
    root = "G:\\Asset\\Programs\\Python3\\NNDL\\HW1\\Rust-unsafe-to-safe-code-retrivial\\database"
    dataset = MyOwnDataset(root)
    # print(os.listdir(root))
    # print(osp.join(root,"raw"))
    del_pre_file()

    gen_test_set()

    knowledge_code_pair: CodePairs = torch.load(sample_path)
    print(type(knowledge_code_pair))
    knowledge_code_pair.pair_list[0].show()
    # knowledge_code_pair.show()
    test_set: CodePairsTest = torch.load(test_path)
    print(type(test_set))
    test_set.pair_list[0].show()

    print("api_name_list ", len(knowledge_code_pair.api_name_list))
    print("all_api_name ", len(knowledge_code_pair.all_api_name))
    print("api_name_kind_list ", len(knowledge_code_pair.api_name_kind_list))
    print("api_name_kind_list ", knowledge_code_pair.api_name_kind_list)

    dataset2 = dataset[0:10]
    print(len(dataset2))

    for i in range(10):
        print(dataset[i])

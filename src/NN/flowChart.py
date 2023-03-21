from typing import List, Any, Dict

import torch
from torch_geometric.data import Data
from DataPreprocessing import *
from enum import Enum
import csv

# 这个节点
class NodeKind(Enum):
    """枚举类：定义节点的类型"""
    # .name 是字符串，访问枚举类型的名称
    # .value 是具体枚举赋值后的类型，访问枚举类型的值
    # mixed节点被展开后状态变为USED
    SEQUENTIAL = 0
    ENTRY = 1
    EXIT = 2
    DUMMY = 3
    MIXED = 4
    USED = 5


# 一个node代表一个基本块
# 设计有向图存放flowchart,采用邻接表
class Node(object):
    """
    节点对象
    """
    # connectedTo: dict[Any, Any]

    # 有向图的边，connectedTo是从本节点到其他节点的边
    def __init__(self, index: int, kind: NodeKind,content , label=None):
        self.index = index
        self.kind = kind  # 节点种类 = 枚举类NodeKind
        self.label = label  # 非mixedNode节点内的实际代码
        self.content = content  # 节点内代码=mixedNode 内未展开的代码
        self.connectedTo = {}  # 存放指向的其它节点的字典，以Vertex：连接边weight的
        self.connectedFrom = {}

    def show(self):
        print("index is ",self.index)
        print("kind is ",self.kind)
        print("label is ",self.label)
        print("content is ",self.content)

    def add_neighbor(self, nbr, weight):
        # nbr:Node

        self.connectedTo.update({nbr: weight})

    def add_from(self,nbr,weight):
        self.connectedFrom.update({nbr:weight})

    def __str__(self):
        return str(self.index) + '-->' + str([nbr.index for nbr in self.connectedTo])

    def get_connections(self):
        return self.connectedTo.indexs()

    def get_id(self):
        return self.index

    def get_weight(self, nbr):
        weight = self.connectedTo.get(nbr)
        if weight is not None:
            return weight
        else:
            raise KeyError("No such nbr exist!")


# cfg有向图的边的weight用EdgeKind表示
class EdgeKind(object):
    # 边的类型：顺序边，跳转边（if跳转边存yes|on，match跳转边存模式匹配值）
    # 顺序边=SEQUENTIAL
    # if跳转边=FALSE|TRUE
    # match跳转边=MATCH_BRANCH
    # EdgeKind有两个属性，name和value，其中只有match跳转边的value才会有值
    def __init__(self, kind: str, condition=None):
        if kind == "SEQUENTIAL":
            self.name = "SEQUENTIAL"
            self.value = None
        elif kind == "FALSE":
            self.name = "FALSE"
            self.value = None
        elif kind == "TRUE":
            self.name = "TRUE"
            self.value = None
        elif kind == "MATCH_BRANCH":
            self.name = "MATCH_BRANCH"
            self.value = condition
        else:
            print("[WARNING]:the edge_kind is illegal")
            self.name = None
            self.value = None
        return

    def __str__(self):
        return str(self.__class__.__name__) + "." + str(self.name)


class Edge(object):
    # 如果图中所有节点node有connectedTo，则可以随时从所有node的connectedTo中恢复所有边edge
    # 但是在动态生成cfg时，需要拆边建边
    # edge类用于存储前向节点对象f、尾节点对象t、边的权重weight:EdgeKind
    def __init__(self, f: Node, t: Node, weight: EdgeKind):
        self.f = f
        self.t = t
        self.weight = weight

# 生成的就是这个图
class Graph(object):
    """
    图对象
    """

    def __init__(self, content: str):
        self.nodeList = {}  # 字典保存节点信息，以{index: Vertex}的方式
        self.edgeList = {}  # 字典保存边信息，以{index: Edge}的方式
        self.nodeEmbedding = [] #存节点的embedding
        self.nodeNum = 0  # 统计图节点数
        self.edgeNum = 0  # 统计图边数
        self.content = content  # 进行注释替换后的源代码
        self.layer = 0  # 用于生成flowchart的变量，标记代码层次

    def gen_flowchart(self):
        if self.layer == 0:
            entry = Node(self.nodeNum, NodeKind.ENTRY, "")
            self.add_node(entry)
            exit = Node(self.nodeNum, NodeKind.EXIT, "")
            self.add_node(exit)
            mixedNode = Node(self.nodeNum, NodeKind.MIXED, self.content)
            self.add_node(mixedNode)

            self.add_edge(entry,mixedNode,EdgeKind("SEQUENTIAL"))
            self.add_edge(mixedNode,exit,EdgeKind("SEQUENTIAL"))

        # self.gen_subflowgraph_from_mixed(mixedNode)
        count = 0
        continueNode = None
        breakNode = None
        while self.exist_mixed_node():
            # print(f"-----in while {count}---------")
            # self.show()

            for i in self.nodeList.keys():
                node = self.nodeList[i]
                if node.kind == NodeKind.MIXED:
                    continueNode,breakNode=self.gen_subflowgraph_from_mixed(node,continueNode,breakNode)
                    break

            count += 1
        pass

    # 展开一个mixednode
    def gen_subflowgraph_from_mixed(self,mixedNode,continueNode,breakNode):
        #展开mixedNode得到子图，即新node和edge
        #代码遍历是顺序的，所以需要标记上一个生成的节点and当时的状态，在当前节点判断与上一节点边的关系，从而生成边
        #生成node是依据;和最外围多对{}
        #在生成node时，在代码处结尾需要更新上一节点和状态
        #被展开的mixedNode相邻边被所有新node的第一个和最后一个连接。（return情况再讨论)

        #从边集中找到mixednode的pre边和next边
        # print(f"mixednode is {mixedNode.index}")
        # self.show()

        pre = None
        next = None
        for i in self.edgeList.keys():
            edge:Edge = self.edgeList[i]
            if edge.t == mixedNode:
                pre = edge
                # print("pre")
            if edge.f == mixedNode:
                next = edge
                # print("next")
        #被展开的mixed node被标记为used
        content = mixedNode.content
        mixedNode.kind = NodeKind.USED
        # print("-----------gen_subflowgraph_from_code---------------")

        buffer = ""#未识别到关键词之前的缓冲区，mixed node中的代码内容
        brace = -1 # 大括号栈
        label = ""#放入节点的内容，展开节点内的代码
        first = -1
        lastNode = None #标记上一节点
        lastState = None #标记上一状态
        markIfEdge = None #该边的值为FALSE，标记非边
        # letIf = False #let if 语法标记，无分号结尾情况被bufferclear解决
        continueNodeNew = continueNode
        breakNodeNew = breakNode

        #for循环，展开mixed node算法
        for c in content:

            buffer = buffer + c
            #have_return,break,continue 提前计算好
            have_return = buffer.find("return") > -1
            have_continue = buffer.find("continue") > -1
            have_break = buffer.find("break") > -1

            if c == ';' and brace == -1 and not have_return and not have_break and not have_continue:
                #未遇到大括号，遇到了分号
                if len(buffer) == 1:
                    #buffer=';'为了解决let if语法块后的分号，单分号情况直接忽略
                    buffer = ""
                    continue
                #检测到在括号外的顺序语句，生成node
                buffer = self.clearbuffer(buffer)
                node = Node(self.nodeNum,NodeKind.SEQUENTIAL,None,buffer)
                self.add_node(node)
                #生成node之后清空buffer
                buffer = ""
                #新建节点时，若是第一个，继承mixedNode的pre边，否则与上一个节点建立边关系
                if first == -1 and pre != None :
                    #是第一个被检测到并新建的node，则继承mixedNode的pre边
                    pre.t = node#新建节点改边时，应更新原边的另一节点的connect node集合
                    # print(f"pre.t is {pre.t.index},pre.f is {pre.f.index}")
                    # print(f"node is {node.index}")
                    # print(f"pre.f.connectedTo is {pre.f.connectedTo.keys()}")
                    # for i in pre.f.connectedTo.keys():
                    #     print(i.index)
                    # self.modify_edge_tail(pre,node)
                    first +=1

                else:
                    #不是第一个新建的node，则新建顺序节点与上个节点连接，顺序边
                    self.add_edge(lastNode,node,EdgeKind("SEQUENTIAL"))

                lastNode = node
                lastState = "SEQUENTIAL"#暂不考虑break、continue、return的情况下，所有分号情况都是顺序的

            elif c ==";" and brace == -1 and have_return and not have_break and not have_continue:
                #检测到了return语句
                # 检测到在括号外的顺序语句，生成node
                # print("-----------return--------------")
                buffer = self.clearbuffer(buffer)
                node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, buffer)
                self.add_node(node)
                # 生成node之后清空buffer
                buffer = ""
                # 新建节点时，若是第一个，继承mixedNode的pre边，否则与上一个节点建立边关系
                if first == -1 and pre != None:
                    # 是第一个被检测到并新建的node，则继承mixedNode的pre边
                    pre.t = node  # 新建节点改边时，应更新原边的另一节点的connect node集合
                    first += 1

                else:
                    # 不是第一个新建的node，则新建顺序节点与上个节点连接，顺序边
                    self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                #由于是return语句，故其下个边连接的为exit
                # self.nodeList[1]即为 exit
                next.t = self.nodeList[1]

                lastNode = node
                lastState = "RETURN"  # 暂不考虑break、continue、return的情况下，所有分号情况都是顺序的

            elif c ==";" and brace == -1 and not have_return and have_break and not have_continue:
                #检测到了return语句
                # 检测到在括号外的顺序语句，生成node
                # print("-----------return--------------")
                buffer = self.clearbuffer(buffer)
                node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, buffer)
                self.add_node(node)
                # 生成node之后清空buffer
                buffer = ""
                # 新建节点时，若是第一个，继承mixedNode的pre边，否则与上一个节点建立边关系
                if first == -1 and pre != None:
                    # 是第一个被检测到并新建的node，则继承mixedNode的pre边
                    pre.t = node  # 新建节点改边时，应更新原边的另一节点的connect node集合
                    first += 1

                else:
                    # 不是第一个新建的node，则新建顺序节点与上个节点连接，顺序边
                    self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                #由于是return语句，故其下个边连接的为exit
                if breakNode:
                    next.t = breakNode
                    breakNodeNew = None

                lastNode = node
                lastState = "BREAK"  # 暂不考虑break、continue、return的情况下，所有分号情况都是顺序的

            elif c ==";" and brace == -1 and not have_return and not have_break and have_continue:
                #检测到了return语句
                # 检测到在括号外的顺序语句，生成node
                # print("-----------return--------------")
                buffer = self.clearbuffer(buffer)
                node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, buffer)
                self.add_node(node)
                # 生成node之后清空buffer
                buffer = ""
                # 新建节点时，若是第一个，继承mixedNode的pre边，否则与上一个节点建立边关系
                if first == -1 and pre != None:
                    # 是第一个被检测到并新建的node，则继承mixedNode的pre边
                    pre.t = node  # 新建节点改边时，应更新原边的另一节点的connect node集合
                    first += 1

                else:
                    # 不是第一个新建的node，则新建顺序节点与上个节点连接，顺序边
                    self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                #由于是return语句，故其下个边连接的为exit
                if continueNode:
                    next.t = continueNode
                    continueNodeNew = None

                lastNode = node
                lastState = "CONTINUE"  # 暂不考虑break、continue、return的情况下，所有分号情况都是顺序的

            elif c == '{' :
                if brace == -1:
                    #第一次遇到左括号，要保存mixedNode的label
                    label = buffer[0:-1]
                    buffer = ""
                    brace += 1#左括号进栈

                else:
                    #第二次遇到左括号，不处理，代码保存
                    brace += 1  # 左括号进栈
            elif c == '}':
                if brace == 0:
                    #与最开始的左括号匹配，创建mixedNode，清空label

                    if label.find("if") > -1 and label.find("else")==-1:
                        # 括号中有if关键字，且没有else，检测到单if块

                        # if label.find("let") > -1:
                        #     letIf = True
                        # else:
                        #     letIf = False
                        label = self.clearbuffer(label)
                        node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, label)
                        self.add_node(node)
                        if first == -1 and pre != None:
                            pre.t = node
                            # self.modify_edge_tail(pre, node)
                            first += 1
                        else:  # 新建顺序节点与上个节点连接
                            self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))

                        buffer = self.clearbuffer(buffer)
                        mixed = Node(self.nodeNum,NodeKind.MIXED,buffer[0:-1])
                        self.add_node(mixed)
                        dummy = Node(self.nodeNum,NodeKind.DUMMY,None)
                        self.add_node(dummy)
                        self.add_edge(node,mixed,EdgeKind("TRUE"))
                        markIfEdge = self.add_edge(node,dummy,EdgeKind("FALSE"))
                        self.add_edge(mixed,dummy,EdgeKind("SEQUENTIAL"))
                        lastNode = dummy

                    elif label.find("else if") > -1:
                        # 括号中有else if关键字
                        label = self.clearbuffer(label)
                        node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, label)
                        self.add_node(node)

                        buffer = self.clearbuffer(buffer)
                        mixed = Node(self.nodeNum, NodeKind.MIXED, buffer[0:-1])
                        self.add_node(mixed)
                        dummy = Node(self.nodeNum, NodeKind.DUMMY, None)
                        self.add_node(dummy)

                        self.add_edge(node, mixed, EdgeKind("TRUE"))

                        e = self.add_edge(node, dummy, EdgeKind("FALSE"))
                        self.add_edge(mixed, dummy, EdgeKind("SEQUENTIAL"))

                        self.add_edge(dummy,markIfEdge.t, EdgeKind("SEQUENTIAL"))
                        markIfEdge.t = node
                        markIfEdge = e

                    elif label.find("else") > -1 and label.find("if") == -1:
                        #else 关键字
                        # print("---find else-----!!!!!!!!!!")
                        buffer = self.clearbuffer(buffer)
                        label = self.clearbuffer(label)
                        node = Node(self.nodeNum, NodeKind.MIXED, buffer[0:-1], label)
                        self.add_node(node)
                        self.add_edge(node,markIfEdge.t,EdgeKind("SEQUENTIAL"))
                        markIfEdge.t = node

                    elif label.find("for") > -1 or label.find("while") > -1:
                        # 发现for和while关键字
                        label = self.clearbuffer(label)
                        node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, label)

                        self.add_node(node)
                        if first == -1 and pre != None:
                            pre.t = node
                            # self.modify_edge_tail(pre, node)
                            first += 1
                        else:  # 新建顺序节点与上个节点连接
                            self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                        buffer = self.clearbuffer(buffer)

                        mixed = Node(self.nodeNum,NodeKind.MIXED,buffer[0:-1])
                        self.add_node(mixed)
                        dummy = Node(self.nodeNum,NodeKind.DUMMY,None)
                        self.add_node(dummy)

                        self.add_edge(node,mixed,EdgeKind("TRUE"))
                        self.add_edge(mixed,node,EdgeKind("SEQUENTIAL"))

                        self.add_edge(node,dummy,EdgeKind("FALSE"))
                        lastNode = dummy
                        continueNodeNew= node
                        breakNodeNew = dummy

                    elif label.find("loop") > -1:
                        # 发现floop关键字,默认loop无死循环，即不考虑loop死循环模式
                        label = self.clearbuffer(label)
                        node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, label)

                        self.add_node(node)
                        if first == -1 and pre != None:
                            pre.t = node
                            # self.modify_edge_tail(pre, node)
                            first += 1
                        else:  # 新建顺序节点与上个节点连接
                            self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                        buffer = self.clearbuffer(buffer)

                        mixed = Node(self.nodeNum,NodeKind.MIXED,buffer[0:-1])
                        self.add_node(mixed)
                        dummy = Node(self.nodeNum,NodeKind.DUMMY,None)
                        self.add_node(dummy)

                        self.add_edge(node,mixed,EdgeKind("TRUE"))
                        self.add_edge(mixed,node,EdgeKind("SEQUENTIAL"))

                        self.add_edge(node,dummy,EdgeKind("FALSE"))
                        lastNode = dummy
                        continueNodeNew= node
                        breakNodeNew = dummy

                    elif label.find("unsafe") > -1:
                        # 发现unsafe关键字,创建顺序代码块
                        # print("find unsafe !")
                        # print("buffer is:" + buffer)
                        # print("label is:" + label)
                        label = self.clearbuffer(label)
                        node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, label)

                        self.add_node(node)
                        if first == -1 and pre != None:
                            pre.t = node
                            # self.modify_edge_tail(pre, node)
                            first += 1
                        else:  # 新建顺序节点与上个节点连接
                            self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                        buffer = self.clearbuffer(buffer)

                        mixed = Node(self.nodeNum, NodeKind.MIXED, buffer[0:-1])
                        self.add_node(mixed)

                        self.add_edge(node, mixed, EdgeKind("SEQUENTIAL"))
                        lastNode = mixed

                    elif label.find("use") > -1 and label.find("unused") == -1:
                        # 发现unsafe关键字,创建顺序代码块
                        # print("find use !")
                        # print("buffer is:" + buffer)
                        # print("label is:" + label)
                        label = self.clearbuffer(label)
                        node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, label)

                        self.add_node(node)
                        if first == -1 and pre != None:
                            pre.t = node
                            # self.modify_edge_tail(pre, node)
                            first += 1
                        else:  # 新建顺序节点与上个节点连接
                            self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                        buffer = self.clearbuffer(buffer)

                        mixed = Node(self.nodeNum, NodeKind.MIXED, buffer[0:-1])
                        self.add_node(mixed)

                        self.add_edge(node, mixed, EdgeKind("SEQUENTIAL"))
                        lastNode = mixed
                    elif label.find("fn") > -1 == -1:
                        # 发现unsafe关键字,创建顺序代码块
                        # print("find fn !")
                        # print("buffer is:" + buffer)
                        # print("label is:" + label)
                        label = self.clearbuffer(label)
                        node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, label)

                        self.add_node(node)
                        if first == -1 and pre != None:
                            pre.t = node
                            # self.modify_edge_tail(pre, node)
                            first += 1
                        else:  # 新建顺序节点与上个节点连接
                            self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                        buffer = self.clearbuffer(buffer)

                        mixed = Node(self.nodeNum, NodeKind.MIXED, buffer[0:-1])
                        self.add_node(mixed)

                        self.add_edge(node, mixed, EdgeKind("SEQUENTIAL"))
                        lastNode = mixed

                    else:
                        #没有任何关键字的代码块
                        buffer = self.clearbuffer(buffer)
                        label = self.clearbuffer(label)
                        # print("====buffer is ===\n",buffer[0:-1])
                        if '{' in buffer[0:-1]:
                            node = Node(self.nodeNum, NodeKind.MIXED, buffer[0:-1], label)
                            self.add_node(node)
                        else:
                            node = Node(self.nodeNum,NodeKind.SEQUENTIAL,buffer[0:-1],label)
                            self.add_node(node)

                        if first == -1 and pre != None:
                            pre.t = node
                            first += 1
                        else:#当前新建节点与上一节点建立联系
                            if lastState == "SEQUENTIAL":
                                self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                            else:
                                self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
                            pass
                        lastNode = node
                    #一对括号写入完毕，缓冲清零，栈弹出
                    buffer = ""
                    label = ""
                    brace -= 1

                else:
                    brace -= 1

        #处理let if语法块中无分号内容
        buffer = self.clearbuffer(buffer)
        t = buffer.replace(" ", "")

        if t != ""and t.find("return") == -1 :
            # 括号内无分号内容
            buffer = self.clearbuffer(buffer)
            # print("buffer is:"+buffer)
            # print(f"lastnode is{lastNode}")
            # print(f"first is {first}")
            # print(f"pre is {pre}")
            node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, buffer)
            self.add_node(node)
            # 新建节点时，若是第一个，继承mixedNode的pre边，否则与上一个节点建立边关系
            if first == -1 and pre != None:
                pre.t = node
                first += 1
            else:  # 新建顺序节点与上个节点连接
                self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
            lastNode = node
        elif t != "" and t.find("return") > -1 :
            # 括号内无分号内容，但检测出return
            buffer = self.clearbuffer(buffer)
            node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, buffer)
            self.add_node(node)
            # 新建节点时，若是第一个，继承mixedNode的pre边，否则与上一个节点建立边关系
            if first == -1 and pre != None:
                pre.t = node
                first += 1
            else:  # 新建顺序节点与上个节点连接
                self.add_edge(lastNode, node, EdgeKind("SEQUENTIAL"))
            # 由于是return语句，故其下个边连接的为exit
            # self.nodeList[1]即为 exit
            next.t = self.nodeList[1]
            lastNode = node
            lastState = "RETURN"  # 暂不考虑break、continue、return的情况下，所有分号情况都是顺序的
        # else:
        #     buffer = self.clearbuffer(buffer)
        #     node = Node(self.nodeNum, NodeKind.SEQUENTIAL, None, buffer)
        #     self.add_node(node)
        #     # 新建节点时，若是第一个，继承mixedNode的pre边，否则与上一个节点建立边关系
        #     if first == -1 and pre != None:
        #         pre.t = node
        #         first += 1

        if next:
            next.f = lastNode
        # print("-----------End of gen_subflowgraph_from_code---------------")
        # self.show()
        pass
        return continueNodeNew,breakNodeNew

    def clearbuffer(self,t):
        """用于清除条件语句中的空格、回车符等无效字符"""

        # 去掉回车与\t
        t = t.replace("\n", "")
        t = t.replace("\t", "")
        t = t.replace("\r", "")
        # t = t.replace(" ", "")
        # 去除空格
        start = -1
        flag1 = False  # 如果前方一直是空格，到i处不为空格，则flag1=True，start = i-1
        end = -1
        flag2 = False
        l = len(t)
        for j in range(l):
            if flag1:
                start = j - 1
                break
            if t[j] == " ":
                flag1 = False
            else:
                flag1 = True
        # 逆序循环，range（起始，终止，-1表示逆序）左闭右开
        for k in range(l - 1, -1, -1):
            if flag2:
                end = k + 1
                break
            if t[k] == " ":
                flag2 = False
            else:
                flag2 = True
        if start > -1 and end > -1:
            t = t[start:end + 1]  # 切片左闭右开，所以end+1
        else:
            # print("ERROR: in buffer")
            pass

        return t
    # 把每个节点变成一个向量，构造模型的输入
    def gen_node_embedding(self):
        # 边，每个节点的出度和入度有一点问题
        for node in self.nodeList.values():
            if node!=None and node.kind.value != 5:
                index = node.index
                nodeKind = node.kind.value
                inDegree = len(node.connectedFrom) #代码长度
                outDegree = len(node.connectedTo) # nlp模型？
                label = 0
                if node.label == None:
                    label = 0
                else:
                    label = len(node.label)

                content = 0
                if node.content == None:
                    content = 0
                else:
                    content = len(node.content)
                #[index,nodeKind,inDegree,outDegree,label,content]
                self.nodeEmbedding.append([index,nodeKind,inDegree,outDegree,label,content])

    def add_node(self, node: Node):
        # 往nodeList添加节点，而非创建节点
        # 创建node然后在加入node
        self.nodeList.update({node.index: node})
        self.nodeNum += 1

    def get_node(self, index):
        vertex = self.nodeList.get(index)
        return vertex

    def __contains__(self, index):
        return index in self.nodeList.values()

    def add_edge(self, f, t, weight: EdgeKind):
        # 加有向边，从f到t的边
        # 节点层面的更新

        # f, t = self.get_node(f), self.get_node(t)
        # # 如果f，t为空nonde，则添加f，t
        # if not f:
        #     self.add_node(f)
        # if not t:
        #     self.add_node(t)

        f.add_neighbor(t, weight)
        t.add_from(f,weight)
        # 边层面的更新
        e = Edge(f, t, weight)
        self.edgeList.update({self.edgeNum: e})
        self.edgeNum += 1
        return e

    def modify_edge_front(self,edge:Edge,node:Node):
        #更新边的前向节点
        f = edge.f
        t = edge.t
        weight = edge.weight
        #replace f->t to node->t
        #先将f的connectedTo删除node
        f.connectedTo.pop(t)
        #再将t的connectedFrom删除f,加入node
        t.connectedFrom.pop(f)
        t.connectedFrom.update({node: weight})



        return

    def modify_edge_tail(self,edge:Edge,node:Node):
        #更新边的尾节点
        f = edge.f
        t = edge.t
        weight = edge.weight
        #replace f->t to f->node
        #先将f的connectedTo删除t，然后加入新node
        f.connectedTo.pop(t)
        f.connectedTo.update({node: weight})
        #再将t的connectedFrom删除f
        t.connectedFrom.pop(f)
        return

    def get_edge(self,index):
        return self.edgeList[index]

    def get_nodes(self):
        return self.nodeList.keys()

    def exist_mixed_node(self):
        for i in self.nodeList.keys():
            node = self.nodeList[i]
            if node.kind == NodeKind.MIXED:
                return True
        return False

    def __iter__(self):
        return iter(self.nodeList.values())

    def show(self):
        for i in self.nodeList.keys():
            print(f"node index is {i}, nodeKind is {self.nodeList[i].kind}, "
                  +f"label is {self.nodeList[i].label}, content is: \n{self.nodeList[i].content}")

        for i in self.edgeList.keys():
            print(f"edge index is {i}, edge is [{self.edgeList[i].f.index} -> {self.edgeList[i].t.index}] "
                  + f"weight is {self.edgeList[i].weight}")
        print(f"node embedding is {self.nodeEmbedding}")

    def toPYG(self):
        # ------------创建图-----------------
        # # torch_geometric.data.Data可以表示一张图
        # # 边的连接性，边的属性暂时忽略
        # edge_index = torch.tensor([[0, 1, 1, 2], [1, 0, 2, 1]], dtype=torch.long)
        # # 点的特征，每个点一维特征
        # x = torch.tensor([[-1], [0], [1]], dtype=torch.float)
        # # 一张图可以用Data类表示，至少需要边和点的信息，定义三个节点两个边的无向图
        # data = Data(x=x, edge_index=edge_index)
        hash_map = {}
        c = 0
        for node in self.nodeList.values():
            if node!=None and node.kind.value != 5:
                index = node.index
                hash_map.update({index: c})
                c = c+1
        x = []#存节点
        for n in self.nodeEmbedding:
            x.append(n[1:len(n)])
        edge_index = []#存边
        fedge = []
        tedge = []
        for i in self.edgeList.keys():
            fedge.append(hash_map.get(self.edgeList[i].f.index))
            tedge.append(hash_map.get(self.edgeList[i].t.index))
        edge_index.append(fedge)
        edge_index.append(tedge)
        
        print(f"x is {x}")
        print(f"edge is {edge_index}")

        # 在这里输入 x edge 组合 
        # 在这里将自己的 x edge 替换成tensor
        
        edge_index = torch.tensor(edge_index, dtype=torch.long)
        x = torch.tensor(x, dtype=torch.float)


        


        data = Data(x=x, edge_index=edge_index)
        self.data = data
        return data

# 测试函数
def test():
    # fileName = r"testdata/if_else/7.txt"
    # fileName = r"testdata/for/4.txt"
    #fileName = r"testdata/return/2.txt"
    # fileName = r"testdata/break/1.txt"
    # fileName = r"testdata/continue/1.txt"
    fileName = r"NN/test/1.rs"
    content, strTable = generating_content(fileName)
    # print(f"Content is \n{content}")  # 注释替换后的代码内容
    # print(f"dict is \n{strTable}")  # 字典存放的注释内容
    flowChart = Graph(content)
    flowChart.gen_flowchart()
    flowChart.gen_node_embedding()

    print(flowChart.nodeList)
    print(flowChart.edgeList)

    flowChart.show()
    # for i in flowChart.nodeList[0].connectedTo.keys():
    #    # print(type(i))
    #    # print(i.index)
    # for i in flowChart.nodeList[0].connectedFrom.keys():
    #     print(type(i))
    #     print(i.index)
    # print()

    flowChart.nodeEmbedding
    flowChart.toPYG()
    print(flowChart.data.x)
    print(flowChart.data.edge_index)

    return

def gen_data_from_rs(fileName):

    # 为神经网络设计的，ACFG，从文件中生成 x edge
    content, strTable = generating_content(fileName)
    flowChart = Graph(content)
    flowChart.gen_flowchart()
    flowChart.gen_node_embedding()
    flowChart.toPYG()
    # flowChart.show()
    # print(flowChart.data.x,flowChart.data.edge_index)
    return flowChart.data
    
# 不管那么多 直接读取内容生成一个 data(x, edge)
# code num 代表第几份code
# aug num 代表第几份数据增强代码

def gen_data_from_csv(code_num, aug_num):
    # number 是一个str

    filenamex = f'spider_stackoverflow/src/dataok/code{code_num}/{aug_num}x.csv'
    filenameedge = f'spider_stackoverflow/src/dataok/code{code_num}/{aug_num}edge.csv'
    
    # 处理文件名字 需要加i

    

    # filename 格式
    x = []
    edge = []
    with open(filenamex) as fx:
        fx_csv = csv.reader(fx)
        
        # 用row 0 1 2 3 4 来表示？
        for row in fx_csv :
            y = []
            for i in range(8) :
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



def gen_acfg_from_txt(file,index=0):
    # 为神经网络设计的，ACFG，从str中直接生成
    # print("file is \n",file)
    content, strTable = generating_content_by_txt(file)
    # print("content is \n",content)
    flowChart = Graph(content)
    flowChart.gen_flowchart()
    flowChart.gen_node_embedding()
    flowChart.toPYG()
    # flowChart.show()
    # print(flowChart.data.x,flowChart.data.edge_index)
    return flowChart.data

# def gendata_from_csv():
#     da

def gen_nodes_from_rs(codeString):
    # 为二部图匹配设计的，只返回节点不返回边
    content, strTable = generating_content_by_txt(codeString)
    flowChart = Graph(content)
    flowChart.gen_flowchart()
    flowChart.gen_node_embedding()
    flowChart.toPYG()
    # flowChart.show()
    # print(flowChart.data.x,flowChart.data.edge_index)
    nodes = []
    for key,value in flowChart.nodeList.items():
        if value.kind !=NodeKind.MIXED and value.kind !=NodeKind.USED:
            nodes.append(value)
    return nodes

if __name__ == "__main__":
    test()


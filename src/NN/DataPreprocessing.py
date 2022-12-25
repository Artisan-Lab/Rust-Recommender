# import unittest
import time

# 函数、变量及属性应该用小写字母来拼写，各单词之间以下划线相连
# 类与异常，应该以每个单词首字母均大写的格式来命名
# 类中的实例方法，应该把首个参数命名为self,以表示该对象的自身
# 类方法的首个参数，应该命名为cls,以表示该类自身


# pycharm中的类型声明 Type hinting in PyCharm

# --------------1读取数据转为字符串-----------------
from typing import List, Any

# 
def read_file_to_str(fileName: str) -> str:
    try:
        f = open(fileName, "r")
        # print("__read file__")
        # print("fileName :\t{}".format(fileName))
        try:
            count = 0

            while True:
                content = f.readlines()

                count = count + 1
                # print("------count is {}-------\n".format(count))
                # print(content)
                if len(content) == 0:
                    break
                else:
                    resultStr = ""
                    for i in range(len(content)):
                        resultStr += content[i]
                    # print("content of file is:\n\t{}".format(resultStr))
                time.sleep(0.1)
                # print("content of file is:\n\t{}".format(content))
        finally:
            f.close()
            # print("__close file__")

    except Exception as result:
        print("产生错误了{}".format(result))

    return resultStr


# ------------------2去除注释--------------------
class SymbolStack(object):
    top: int
    stack: list
    length: int

    def __init__(self):
        top = -1
        stack: list = []
        length = len(stack)

    def push(self, elem):
        if self.length > self.top + 1:
            self.top = self.top + 1
            self.stack[self.top] = elem
        else:
            self.top = self.top + 1
            self.stack.append(elem)
            self.length = len(self.stack)

    def pop(self):
        elem = self.stack[self.top]
        self.top = self.top - 1
        return elem

    pass


def delete_annotation(content):
    # rust 有三种注释，删除用空代替,实际就两种
    newContent = ''
    flag = 0  # 0表示正常行文，1表示//需要\n作为结尾，2表示/*需要*/结尾
    delta = 0
    for i in range(len(content)):
        c = content[i]
        cNext: str
        # cPre:int #1表示上次为//
        # 初始化两个字符
        if i + 1 < len(content):
            cNext = content[i + 1]
        else:
            cNext = None

        # 根据两字符判断是否为注释
        if c == '/' and cNext == '/' and flag == 0:
            flag = 1
        elif c == '/' and cNext == '*' and flag == 0:
            flag = 2

        # 判断注释是否结束
        if flag == 1 and c == '\n':
            delta = 1
            flag = 0  # 表示//结尾，下一个才能进入字符串
        elif flag == 2 and c == '*' and cNext == '/':
            delta = 2
            flag = 0  # 表示/*中结尾*，下下一个才能进入字符串

        # 判断当前字符是否添加进新字符串
        if flag == 0:
            if delta == 0:
                newContent = newContent + c
            elif delta >= 1:
                delta = delta - 1
                pass

    return newContent

# 数据更改，比如关键字给替换掉，为了提取控制流图
# ------------------3引号内容替换为注释变量，并输出字符串变量表------------
def replace_quotation_marks(content):
    finalContent = ''
    stringVariableTable = {}  # 用字典存储字符串变量表
    variableName = None
    value = ''
    # key=stri,value = "", 比如 key = str0, value = "hello"
    flag = 0  # 1表示“”
    delta = 0
    for i in range(len(content)):
        c = content[i]
        cPre: str
        cNext: str
        if i > 0:
            cPre = content[i - 1]
        else:
            cPre = None

        # 初始化两个字符
        if i + 1 < len(content):
            cNext = content[i + 1]
        else:
            cNext = None
        # 判断是否为引号
        if c == '"' and flag == 0 and cPre != "\\" and delta == 0:
            flag = 1

        # 判断引号是否结束
        if flag == 1 and cNext == '"' and c != "\\":
            flag = 0
            delta = 2

        # 判断当前字符是否添加进新字符串
        if flag == 0:
            if delta == 0:
                finalContent = finalContent + c
            elif delta >= 1:
                delta = delta - 1
                pass

        # 创建字符串变量
        if flag + delta > 0:
            # 说明是在引号内
            value = value + c
        if delta == 1:
            value = value + cNext
            variableName = "str" + str(len(stringVariableTable))
            stringVariableTable[variableName] = value
            value = ''
            finalContent = finalContent + "/*" + variableName + "*/"

    return finalContent, stringVariableTable


# -------------测试代码-------------------------
def generating_content(fileName: str):
    # 预处理操作合并，分别是读入、删除注释、将引号内容替换并生成变量表
    c1 = read_file_to_str(fileName)
    c2 = delete_annotation(c1)
    content, strTable = replace_quotation_marks(c2)
    return content, strTable

def generating_content_by_txt(file: str):
    # 预处理操作合并，分别是读入、删除注释、将引号内容替换并生成变量表
    c2 = delete_annotation(file)
    content, strTable = replace_quotation_marks(c2)
    return content, strTable


def main():
    fileName = r"testdata/annotation/1.txt"
    content = read_file_to_str(fileName)
    # print(f"content is {content}")
    # index = 0
    # for c in content:
    #     index = index + 1
    #     print(f"index : {index}, char : {c}")

    # print(content[39] == "\n")

    newContent = delete_annotation(content)
    print(f"newContent is \n{newContent}")
    # index = 0
    # for c in newContent:
    #     index = index + 1
    #     print(f"index : {index}, char : {c}")

    finalContent, strTable = replace_quotation_marks(newContent)
    print(f"finalContent is \n{finalContent}")
    print(f"dict is \n{strTable}")

    return


if __name__ == "__main__":
    main()

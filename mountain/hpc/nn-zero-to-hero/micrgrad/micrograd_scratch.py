class Value:
    """
    目标: 存储一个标量及其梯度
    """
    def __init__(self, data : float, _prev=(), _op='') -> None:
        self.data = data
        self.grad = 0
        self._prev = set(_prev)
        self._op = _op


    def __add__(self, other):
        """
        算子: 加法
        """

        # 如果 other不是Value对象就转为 Value 对象
        if not isinstance(other, Value) : other =  Value(other)

        result = Value(self.data + other.data, (self, other), '+')


        # 闭包：函数记住一些外部变量的值，即使这些变量所在的外层作用域已经结束了。

        return result


    def __repr__(self) -> str:
        # return self.data # 怎么把这个转为string呢
        return f'Value(data={self.data}, grad={self.grad}, _prev={self._prev}, _op={self._op})'


def main():
    # a = Value(3.0)
    # print(a)

    b = 7.0
    c = Value(13.0)
    d = c + b # 想"b + c"也能工作，需要在Value类实现__radd__方法
    print(d)
    

if __name__ == "__main__":
    main()




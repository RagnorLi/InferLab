import numpy as np
import matplotlib.pyplot as plt
from set_matplotlib_font import set_up_font

"""
第一步：从具体案例推导（先数数，再下结论）

假设底层是连续内存数组，容量足够（不考虑扩容，只看"搬元素"成本），
数组当前元素个数为 n。

具体案例：当 n = 5，数组为 [a0, a1, a2, a3, a4]

1. 在开头插入（index = 0）
   要搬的元素：a0, a1, a2, a3, a4 共 5 个
   新数组：[x, a0, a1, a2, a3, a4]

2. 在中间插入（index = 2）
   要搬的元素：a2, a3, a4 共 3 个
   新数组：[a0, a1, x, a2, a3, a4]

3. 在尾部前一位插入（index = 4）
   要搬的元素：a4 共 1 个
   新数组：[a0, a1, a2, a3, x, a4]

4. 在尾部插入（index = n = 5，相当于 append）
   要搬的元素：0 个（直接在后面写）
   新数组：[a0, a1, a2, a3, a4, x]

归纳规律：
- 插入位置为 i（0 ≤ i ≤ n），需要搬的元素数 = n - i
  - 最坏情况（i = 0）：搬 n 个元素
  - 最好情况（i = n）：搬 0 个元素
  - 平均情况：所有位置等概率，搬的平均数量 = n/2
    (0 + 1 + 2 + ... + n) / (n + 1) = n(n + 1)/2 / (n + 1) = n/2

结论：无论最坏还是平均，搬移成本都与 n 线性相关，时间复杂度为 O(n)

O(n/2) = O(n)  因为 n/2 = (1/2)·n，常数因子 1/2 被忽略
O(2n) = O(n)   因为 2n = 2·n，常数因子 2 被忽略  
O(100n) = O(n) 因为 100n = 100·n，常数因子 100 被忽略

大 O 关注的是“增长率”：当 n 增大时，函数增长多快
n/2 和 n 的增长速度相同（都是线性增长）
复杂度分类关注的是“慢速/中速/快速”这种级别，而不是精确数值
"""


def insert_cost(n, i):
    """长度为 n，在位置 i 插入，需要搬的元素数 = n - i"""
    return n - i

def main():
    ns = np.array([10, 50, 100, 200, 500, 1000])
    max_costs = []
    avg_costs = []

    for n in ns:
        positions = np.arange(0, n + 1)  # i from 0 to n
        costs = insert_cost(n, positions)
        max_costs.append(costs.max())
        avg_costs.append(costs.mean())

    plt.figure(figsize=(8, 5))
    plt.plot(ns, max_costs, 'r-o', label='最坏成本 T_max(n)')
    plt.plot(ns, avg_costs, 'b-s', label='平均成本 T_avg(n)')
    plt.xlabel('数组长度 n')
    plt.ylabel('搬移元素数量（操作次数）')
    plt.title('数组 insert 操作成本随 n 的变化')
    plt.legend()
    plt.grid(alpha=0.3)
    plt.savefig('./png/array_insert_analysis.png', dpi=150)
    print("✅ 图表已保存为 array_insert_analysis.png")
    plt.show()

if __name__ == "__main__":
    set_up_font()
    main()
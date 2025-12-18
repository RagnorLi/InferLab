# array_amortized_analysis.py
"""
实验：动态数组2倍扩容的摊销复杂度分析

第一次扩容：复制 1 个元素，代价 1
第二次扩容：复制 2 个元素，代价 2
第三次扩容：复制 4 个元素，代价 4
第 k 次扩容：复制 2^(k-1) 个元素，代价 2^(k-1)
扩容总代价：1 + 2 + 4 + ... + 2^(k-1) = 2^k - 1 ≈ 当前容量
这些代价被后续的低成本操作（直接追加，无扩容）分摊
"""

import numpy as np
import matplotlib.pyplot as plt
from set_matplotlib_font import set_up_font

def main():
    # 步骤1：定义函数
    k = np.arange(1, 21)  # 扩容次数：1到20次
    
    # 总代价函数
    total_cost = 2**k - 1
    
    # 元素总数（扩容后的容量）
    n = 2**k
    
    # 平均代价（摊销复杂度）
    avg_cost = total_cost / n
    
    # 步骤2：创建图表
    fig, axes = plt.subplots(2, 2, figsize=(14, 10))
    
    # 图1：总代价曲线（线性坐标）
    # 这里只是画出S(k)=2^k-1本身，看到的是随扩容次数k指数级增长的总成本，所以曲线越来越陡峭
    # 它并不能直接告诉你摊销复杂度；只是直观反映总代价的“绝对数值”随k增长很快
    axes[0, 0].plot(k, total_cost, 'b-o', linewidth=2, markersize=4)
    axes[0, 0].set_xlabel('扩容次数 k', fontsize=12)
    axes[0, 0].set_ylabel('总代价 S(k) = 2^k - 1', fontsize=12)
    axes[0, 0].set_title('总代价随扩容次数变化（线性坐标）', fontsize=14, fontweight='bold')
    axes[0, 0].grid(True, alpha=0.3)
    
    # 图2：总代价曲线（对数坐标）- 看指数增长
    # semilogy 这行就是用了对数坐标，y 轴取了对数:
    axes[0, 1].semilogy(k, total_cost, 'r-o', linewidth=2, markersize=4)
    axes[0, 1].set_xlabel('扩容次数 k', fontsize=12)
    axes[0, 1].set_ylabel('总代价 S(k) = 2^k - 1 (对数)', fontsize=12)
    axes[0, 1].set_title('总代价随扩容次数变化（对数坐标）', fontsize=14, fontweight='bold')
    axes[0, 1].grid(True, alpha=0.3)
    
    # 图3：平均代价（关键！证明O(1)）
    axes[1, 0].plot(k, avg_cost, 'g-o', linewidth=2, markersize=4)
    axes[1, 0].axhline(y=1, color='r', linestyle='--', linewidth=1.5, label='y = 1')
    axes[1, 0].set_xlabel('扩容次数 k', fontsize=12)
    axes[1, 0].set_ylabel('平均代价 S(k)/n', fontsize=12)
    axes[1, 0].set_title('摊销复杂度：平均代价趋于常数1即证明O(1)', fontsize=14, fontweight='bold')
    axes[1, 0].legend() # legend() 会在图表中显示一个图例（标识），用于解释各曲线或标记的含义，比如这里用 label='y = 1' 显示红色虚线的含义
    axes[1, 0].grid(True, alpha=0.3) # grid() 用于在图表背景添加网格线，辅助观察数值变化，使曲线走向和数值区间更清晰。参数 True 表示显示网格，alpha 控制透明度
    # 图4：总代价 vs 元素总数（看线性关系）
    axes[1, 1].plot(n, total_cost, 'm-o', linewidth=2, markersize=4)
    axes[1, 1].plot(n, n, 'k--', linewidth=1.5, label='y = n (线性)')
    axes[1, 1].set_xlabel('元素总数 n', fontsize=12)
    axes[1, 1].set_ylabel('总代价 S(k)', fontsize=12)
    axes[1, 1].set_title('总代价与元素总数关系（线性）', fontsize=14, fontweight='bold')
    axes[1, 1].legend()
    axes[1, 1].grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig('./png/array_amortized_analysis.png', dpi=300, bbox_inches='tight')
    print("✅ 图表已保存为: array_amortized_analysis.png")
    plt.show()

if __name__ == '__main__':
    set_up_font()
    main()
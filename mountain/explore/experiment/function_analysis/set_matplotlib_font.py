from matplotlib import font_manager
import matplotlib.pyplot as plt


def set_up_font():
    # 列出所有包含中文的字体
    fonts = [f.name for f in font_manager.fontManager.ttflist if 'PingFang' in f.name or 'STHeiti' in f.name or 'Arial Unicode' in f.name]
    print("可用的中文字体:", fonts)

    # 设置中文字体（macOS）
    plt.rcParams['font.sans-serif'] = ['PingFang SC', 'STHeiti']
    plt.rcParams['axes.unicode_minus'] = False

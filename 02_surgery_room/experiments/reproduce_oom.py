"""
实验：复现 vLLM OOM 场景

目标：理解什么情况下会触发 OOM，以及 vLLM 如何处理
"""

# TODO: 引入 vLLM 后实现
# from vllm import LLM, SamplingParams

def main():
    print("TODO: 实现 OOM 复现实验")
    print("计划：")
    print("1. 设置很小的 GPU 内存限制")
    print("2. 发送大量长文本请求")
    print("3. 观察 vLLM 如何处理（抢占/拒绝/等待）")
    print("4. 对比 mini-vLLM 的实现")


if __name__ == "__main__":
    main()


"""
Engine - 串联一切的主循环

这是 mini-vLLM 的核心引擎，串联 BlockManager 和 Scheduler
"""

from typing import List, Optional
from .block_manager import BlockManager
from .scheduler import Scheduler, Request


class InferenceEngine:
    """推理引擎主循环"""
    
    def __init__(
        self,
        num_blocks: int = 100,
        block_size: int = 16,
        max_batch_size: int = 4
    ):
        """
        Args:
            num_blocks: 显存总块数
            block_size: 每块容量（tokens）
            max_batch_size: 最大批处理大小
        """
        self.block_manager = BlockManager(num_blocks, block_size)
        self.scheduler = Scheduler(max_batch_size)
        self.iteration_count = 0
    
    def add_request(
        self,
        request_id: str,
        prompt: str,
        max_tokens: int = 100
    ):
        """添加推理请求"""
        request = Request(
            request_id=request_id,
            prompt=prompt,
            max_tokens=max_tokens
        )
        self.scheduler.add_request(request)
        print(f"✓ 添加请求: {request_id}, prompt长度: {len(prompt)}")
    
    def step(self) -> bool:
        """
        执行一个推理步骤
        
        Returns:
            是否还有请求在处理
        """
        self.iteration_count += 1
        
        # 1. 调度：获取本次要处理的请求
        batch = self.scheduler.schedule()
        
        if not batch:
            return False
        
        print(f"\n=== Iteration {self.iteration_count} ===")
        print(f"Batch size: {len(batch)}")
        print(f"Free blocks: {self.block_manager.get_num_free_blocks()}")
        
        # 2. 为每个请求分配显存（如果需要）
        for req in batch:
            if req.request_id not in self.block_manager.allocated_blocks:
                try:
                    # 简化：假设 prompt 长度为 10 tokens
                    blocks = self.block_manager.allocate(req.request_id, num_tokens=10)
                    print(f"  → {req.request_id}: 分配块 {blocks}")
                except RuntimeError as e:
                    print(f"  → {req.request_id}: {e}")
                    continue
        
        # 3. 模拟推理（生成一个 token）
        for req in batch:
            req.num_generated += 1
            print(f"  → {req.request_id}: 生成 token {req.num_generated}/{req.max_tokens}")
            
            # 检查是否完成
            if req.num_generated >= req.max_tokens:
                self.scheduler.mark_finished(req.request_id)
                self.block_manager.free(req.request_id)
                print(f"  ✓ {req.request_id} 完成！释放显存")
        
        return True
    
    def run(self, max_iterations: int = 100):
        """运行主循环直到所有请求完成"""
        print(f"\n🚀 启动 mini-vLLM 引擎")
        print(f"配置: {self.block_manager.num_blocks} 块 × {self.block_manager.block_size} tokens/块")
        
        for _ in range(max_iterations):
            if not self.step():
                break
        
        print(f"\n✅ 完成！总共执行 {self.iteration_count} 次迭代")


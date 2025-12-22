"""
Block Manager - 用 Python list 模拟显存管理

这是一个最简化的显存块管理器，用于理解 vLLM 中的 PagedAttention 原理。
"""


class BlockManager:
    """管理 KV Cache 的内存块分配"""
    
    def __init__(self, num_blocks: int, block_size: int):
        """
        Args:
            num_blocks: 总块数（模拟显存容量）
            block_size: 每个块的 token 容量
        """
        self.num_blocks = num_blocks
        self.block_size = block_size
        self.free_blocks = list(range(num_blocks))  # 空闲块列表
        self.allocated_blocks = {}  # {request_id: [block_ids]}
    
    def allocate(self, request_id: str, num_tokens: int) -> list[int]:
        """为请求分配块"""
        num_blocks_needed = (num_tokens + self.block_size - 1) // self.block_size
        
        if len(self.free_blocks) < num_blocks_needed:
            raise RuntimeError(f"OOM: 需要 {num_blocks_needed} 块，但只有 {len(self.free_blocks)} 块可用")
        
        allocated = [self.free_blocks.pop(0) for _ in range(num_blocks_needed)]
        self.allocated_blocks[request_id] = allocated
        return allocated
    
    def free(self, request_id: str):
        """释放请求占用的块"""
        if request_id in self.allocated_blocks:
            blocks = self.allocated_blocks.pop(request_id)
            self.free_blocks.extend(blocks)
            self.free_blocks.sort()
    
    def get_num_free_blocks(self) -> int:
        """获取空闲块数量"""
        return len(self.free_blocks)


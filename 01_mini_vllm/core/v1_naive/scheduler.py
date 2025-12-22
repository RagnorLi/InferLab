"""
Scheduler - 最笨的调度逻辑

实现一个简单的 FCFS (First Come First Serve) 调度器
"""

from typing import List, Optional
from dataclasses import dataclass


@dataclass
class Request:
    """推理请求"""
    request_id: str
    prompt: str
    max_tokens: int
    num_generated: int = 0
    status: str = "waiting"  # waiting, running, finished
    

class Scheduler:
    """简单的 FCFS 调度器"""
    
    def __init__(self, max_batch_size: int = 4):
        """
        Args:
            max_batch_size: 最大批处理大小
        """
        self.max_batch_size = max_batch_size
        self.waiting_queue: List[Request] = []
        self.running_queue: List[Request] = []
    
    def add_request(self, request: Request):
        """添加新请求到等待队列"""
        self.waiting_queue.append(request)
    
    def schedule(self) -> List[Request]:
        """
        调度逻辑：
        1. 先处理正在运行的请求
        2. 如果还有空间，从等待队列中取新请求
        
        Returns:
            本次 iteration 要处理的请求列表
        """
        # 移除已完成的请求
        self.running_queue = [req for req in self.running_queue if req.status != "finished"]
        
        # 计算还能容纳多少新请求
        available_slots = self.max_batch_size - len(self.running_queue)
        
        # 从等待队列中取出新请求
        while available_slots > 0 and self.waiting_queue:
            req = self.waiting_queue.pop(0)
            req.status = "running"
            self.running_queue.append(req)
            available_slots -= 1
        
        return self.running_queue.copy()
    
    def mark_finished(self, request_id: str):
        """标记请求完成"""
        for req in self.running_queue:
            if req.request_id == request_id:
                req.status = "finished"
                break


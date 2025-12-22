"""
实验：抓取 vLLM 推理的 Timeline

目标：可视化每个请求的生命周期，理解调度过程
"""

import time
from dataclasses import dataclass
from typing import List


@dataclass
class Event:
    """时间线事件"""
    timestamp: float
    event_type: str  # request_arrived, scheduled, preempted, finished
    request_id: str
    details: str = ""


class TimelineCapture:
    """捕获推理过程的时间线"""
    
    def __init__(self):
        self.events: List[Event] = []
        self.start_time = time.time()
    
    def log(self, event_type: str, request_id: str, details: str = ""):
        """记录事件"""
        event = Event(
            timestamp=time.time() - self.start_time,
            event_type=event_type,
            request_id=request_id,
            details=details
        )
        self.events.append(event)
        print(f"[{event.timestamp:.3f}s] {event_type:20s} | {request_id} | {details}")
    
    def save(self, filepath: str):
        """保存到文件"""
        with open(filepath, 'w') as f:
            for event in self.events:
                f.write(f"{event.timestamp:.3f},{event.event_type},{event.request_id},{event.details}\n")


def main():
    print("TODO: 接入真实 vLLM 后实现")
    print("当前展示的是概念验证")
    
    timeline = TimelineCapture()
    
    # 模拟事件
    timeline.log("request_arrived", "req1", "prompt_len=100")
    time.sleep(0.1)
    timeline.log("scheduled", "req1", "batch_size=1")
    time.sleep(0.2)
    timeline.log("request_arrived", "req2", "prompt_len=50")
    timeline.log("scheduled", "req2", "batch_size=2")
    time.sleep(0.15)
    timeline.log("finished", "req1", "generated=20")
    
    print("\n✓ Timeline 捕获完成")


if __name__ == "__main__":
    main()


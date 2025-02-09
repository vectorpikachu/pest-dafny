#!/usr/bin/env python3

import os
import subprocess

# 创建结果目录
os.makedirs("results", exist_ok=True)

# 遍历 test 目录下的所有 .dfy 文件
for input_file in os.listdir("./test"):
    if input_file.endswith(".dfy"):
        # 获取文件名（不带扩展名）
        filename = os.path.splitext(input_file)[0]
        
        # 设置输入输出文件路径
        input_path = os.path.join("./test", input_file)
        output_path = os.path.join("./results", f"{filename}.txt")
        
        # 运行 cargo 命令
        subprocess.run(["cargo", "run", "--", "-i", input_path, "-o", output_path])

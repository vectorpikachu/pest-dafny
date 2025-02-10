#!/usr/bin/env python3

import os
import subprocess

# 创建结果目录和日志目录
os.makedirs("results", exist_ok=True)
os.makedirs("log", exist_ok=True)

# 初始化统计信息
total_files = 0
successful_files = 0
panic_files = []
log_file_path = os.path.join("log", "execution_log.txt")

# 打开日志文件
with open(log_file_path, "w") as log_file:
    log_file.write("执行统计日志\n")
    log_file.write("--------------------\n")
    
    # 遍历 test 目录下的所有 .dfy 文件，包括子文件夹
    for root, dirs, files in os.walk("./test"):
        for input_file in files:
            if input_file.endswith(".dfy"):
                total_files += 1
                # 获取文件名（不带扩展名）
                filename = os.path.splitext(input_file)[0]
                
                # 设置输入输出文件路径
                input_path = os.path.join(root, input_file)
                # 输出路径保持目录结构在 results 中
                relative_path = os.path.relpath(root, "./test")
                output_dir = os.path.join("results", relative_path)
                os.makedirs(output_dir, exist_ok=True)
                output_path = os.path.join(output_dir, f"{filename}.txt")
                
                # 运行 cargo 命令并捕获输出
                try:
                    result = subprocess.run(
                        ["cargo", "run", "--", "-i", input_path, "-o", output_path],
                        capture_output=True, text=True, check=True
                    )
                    successful_files += 1
                except subprocess.CalledProcessError as e:
                    # 如果 cargo run 失败并产生 panic
                    panic_files.append(input_path)
                    log_file.write(f"Panic: {input_path}\n")
                    log_file.write(f"Error message: {e.stderr}\n")
                else:
                    # 如果命令成功执行，记录成功的信息
                    log_file.write(f"Success: {input_path}\n")

    # 输出统计信息
    log_file.write("\n统计信息: \n")
    log_file.write(f"总文件数: {total_files}\n")
    log_file.write(f"成功的文件数: {successful_files}\n")
    log_file.write(f"Panic 的文件数: {len(panic_files)}\n")
    if panic_files:
        log_file.write("Panic 的文件列表:\n")
        for panic_file in panic_files:
            log_file.write(f"- {panic_file}\n")

print(f"执行完毕，结果已保存到 'results' 文件夹，日志已记录到 'log/execution_log.txt'")

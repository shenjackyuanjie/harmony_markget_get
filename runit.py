import time
import signal
import sys
import subprocess
import os


# 需要执行的任务函数 - 运行 cargo run --release
def scheduled_task():
    print(
        f"开始执行 cargo run --release - 当前时间: {time.strftime('%Y-%m-%d %H:%M:%S')}"
    )

    try:
        # 使用subprocess运行cargo命令
        # 设置cwd参数可以指定工作目录，如果不在当前目录，请修改为正确的路径
        result = subprocess.run(
            ["cargo run --release"],
            cwd=os.getcwd(),  # 默认使用当前目录，可以修改为具体路径
            capture_output=False,
            text=True,
            shell=True,
            timeout=600,  # 设置超时时间为10分钟，防止任务执行时间过长
        )

        # 输出执行结果
        print("标准输出:")
        print(result.stdout)
        if result.stderr:
            print("错误输出:")
            print(result.stderr)
        print(f"退出码: {result.returncode}")

    except subprocess.TimeoutExpired:
        print("任务执行超时（超过10分钟）")
    except FileNotFoundError:
        print("错误: 未找到cargo命令，请确保Rust已安装并配置正确")
    except Exception as e:
        print(f"执行任务时发生错误: {e}")


# 信号处理函数，用于优雅退出
def signal_handler(sig, frame):
    print("\n接收到中断信号，程序将退出...")
    sys.exit(0)


def main():
    # 注册信号处理
    signal.signal(signal.SIGINT, signal_handler)

    print("Rust项目自动构建器已启动，每1小时执行一次 cargo run --release")
    print("按 Ctrl+C 可退出程序")

    try:
        while True:
            # 执行任务
            scheduled_task()

            print(
                f"等待1小时，下次执行时间: {time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(time.time() + 60 * 60))}"
            )
            time.sleep(60 * 60)

    except KeyboardInterrupt:
        print("\n程序被用户中断")
    except Exception as e:
        print(f"程序执行出错: {e}")
    finally:
        print("程序结束")


if __name__ == "__main__":
    main()

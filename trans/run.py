import os
import re
import shutil
import subprocess
from zhipuai import ZhipuAI
import concurrent.futures
import uuid  

# 从环境变量 "ZHIPUAI_API_KEY" 中读取 API Key
api_key = os.environ.get("ZHIPUAI_API_KEY")
if not api_key:
    # 如果环境变量没有设置，则尝试从 'api_key' 文件读取
    print("ZHIPUAI_API_KEY not set, trying to read from 'api_key' file...")
    try:
        with open("api_key", "r") as f:
            api_key = f.read().splitlines()[0]
    except FileNotFoundError:
        print("Error: 'api_key' file not found.")
        print("Please set the ZHIPUAI_API_KEY environment variable or create an 'api_key' file.")
        exit(1) # 退出脚本，因为没有key无法继续
    except IndexError:
        print("Error: 'api_key' file is empty.")
        exit(1)

if not api_key:
    print("Error: API Key is empty. Please check your environment variable or 'api_key' file.")
    exit(1)

client = ZhipuAI(api_key=api_key)

# temp_default = 0.95,
# top_default = 0.7,

def read_file_content(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        return file.read()

def write_to_file(file_path, content):
    with open(file_path, 'w', encoding='utf-8') as file:
        file.write(content)


def process_file_single(input_content, temp_prm, top_p_prm):
    input_content += "转化为rust"
    response_first_round = client.chat.completions.create(
        model="glm-4",
        messages=[{"role": "user", "content": input_content}],
        temperature = temp_prm,
        top_p = top_p_prm,
        max_tokens=8192,
    )
    output = response_first_round.choices[0].message.content
    rust_codes = re.findall(r'```rust\n?(.*?)\n?```', output, re.DOTALL)
    return rust_codes

def process_file_double(input_content, temp_prm, top_p_prm):
    input_content += "简要分析这段代码"
    response_first_round = client.chat.completions.create(
        model="glm-4",
        messages=[{"role": "user", "content": input_content}],
    )
    first_round_output = response_first_round.choices[0].message.content
    response = client.chat.completions.create(
        model="glm-4", 
        messages=[
            {"role": "user", "content": input_content},
            {"role": "assistant", "content": first_round_output},
            {"role": "user", "content": "用一份可编译的完整Rust代码实现它"}
        ],
        temperature = temp_prm,
        top_p = top_p_prm,
        max_tokens=8192,
    )
    second_round_output = response.choices[0].message.content
    rust_codes = re.findall(r'```rust\n?(.*?)\n?```', second_round_output, re.DOTALL)
    return rust_codes

def compile_and_select_s(input_dir, results_dir, success_dir, test_compile_dir):
    success_compile = False
    max_size = 0
    best_file_path = None

    # 尝试编译results_dir下的所有文件
    for file_name in os.listdir(results_dir):
        src_file_path = os.path.join(results_dir, file_name)
        dst_file_path = os.path.join(test_compile_dir, "src", "main.rs")
        
        # 确保testcompile/src存在
        os.makedirs(os.path.dirname(dst_file_path), exist_ok=True)
        
        # 将文件复制为testcompile/src/main.rs并尝试编译
        shutil.copy(src_file_path, dst_file_path)
        compile_command = "cargo build"
        result = subprocess.run(compile_command, shell=True, cwd=test_compile_dir, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        
        if result.returncode == 0:
            success_compile = True
            file_size = os.path.getsize(src_file_path)
            if file_size > max_size:
                max_size = file_size
                best_file_path = src_file_path

    # 如果编译成功，将最大的成功编译的文件移动到success文件夹
    if success_compile and best_file_path:
        success_file_name = os.path.basename(best_file_path)
        success_file_path = os.path.join(success_dir, success_file_name)
        os.rename(best_file_path, success_file_path)
        return True
    # 如果没有成功编译的文件
    # elif not success_compile:
    else:
        # 清空results_dir目录
        for file_name in os.listdir(results_dir):
            file_path = os.path.join(results_dir, file_name)
            os.remove(file_path)
        return False

def compile_and_select_best(input_dir, results_dir, success_dir, test_compile_dir):
    success_compile = False
    max_size = 0
    best_file_path = None

    # 尝试编译results_dir下的所有文件
    for file_name in os.listdir(results_dir):
        src_file_path = os.path.join(results_dir, file_name)
        dst_file_path = os.path.join(test_compile_dir, "src", "main.rs")
        
        # 确保testcompile/src存在
        os.makedirs(os.path.dirname(dst_file_path), exist_ok=True)
        
        # 将文件复制为testcompile/src/main.rs并尝试编译
        shutil.copy(src_file_path, dst_file_path)
        compile_command = "cargo build"
        result = subprocess.run(compile_command, shell=True, cwd=test_compile_dir, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        
        if result.returncode == 0:
            success_compile = True
            file_size = os.path.getsize(src_file_path)
            if file_size > max_size:
                max_size = file_size
                best_file_path = src_file_path

    # 如果编译成功，将最大的成功编译的文件移动到success文件夹
    if success_compile and best_file_path:
        success_file_name = os.path.basename(best_file_path)
        success_file_path = os.path.join(success_dir, success_file_name)
        os.rename(best_file_path, success_file_path)
        return True
    # 如果没有成功编译的文件，选择results文件夹中最大的文件移动到success文件夹
    # elif not success_compile:
    else:
        for file_name in os.listdir(results_dir):
            file_path = os.path.join(results_dir, file_name)
            file_size = os.path.getsize(file_path)
            if file_size > max_size:
                max_size = file_size
                best_file_path = file_path
        if best_file_path:
            success_file_name = os.path.basename(best_file_path)
            success_file_path = os.path.join(success_dir, success_file_name)
            os.rename(best_file_path, success_file_path)
        return False



def process_input_files_s(input_dir, results_dir, success_dir, test_compile_dir):
    if not os.path.exists(results_dir):
        os.makedirs(results_dir)
    if not os.path.exists(success_dir):
        os.makedirs(success_dir)

    # functions = [process_file_single]
    # functions = functions * 5
    
    temp_prms = [0.95, 0.99, 0.90, 0.95, 0.95]
    top_p_prms = [0.7, 0.7, 0.7, 0.8, 0.9]

    input_file = "input_file"
    input_content = read_file_content(os.path.join(input_dir, input_file))
    input_contents = [input_content]
    input_contents = input_contents * 5
    inputs = [(input_content, temp_prm, top_p_prm) for input_content, temp_prm, top_p_prm in zip(input_contents, temp_prms, top_p_prms)]
    with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
        future_to_function = {executor.submit(process_file_single, *input_args): process_file_single for input_args in inputs}
        
        for idx, future in enumerate(concurrent.futures.as_completed(future_to_function), start=1):
            function = future_to_function[future]
            try:
                rust_codes = future.result()
                # 对于每个函数的输出，写入不同的文件
                for code in rust_codes:
                    temp_file_name = f"result_sgl_{idx}.rs" 
                    temp_file_path = os.path.join(results_dir, temp_file_name)
                    with open(temp_file_path, 'w', encoding='utf-8') as file:
                        file.write(code)
            except Exception as exc:
                print(f'{function.__name__} generated an exception: {exc}')

    bool_tag = compile_and_select_s(input_dir, results_dir, success_dir, test_compile_dir)
    return bool_tag

def process_input_files_concurrently(input_dir, results_dir, success_dir, test_compile_dir):
    if not os.path.exists(results_dir):
        os.makedirs(results_dir)
    if not os.path.exists(success_dir):
        os.makedirs(success_dir)
    
    # functions = [process_file_double]
    # functions = functions * 5
    
    temp_prms = [0.95, 0.99, 0.90, 0.95, 0.95]
    top_p_prms = [0.7, 0.7, 0.7, 0.8, 0.9]
    input_file = "input_file"
    input_content = read_file_content(os.path.join(input_dir, input_file))
    input_contents = [input_content]
    input_contents = input_contents * 5
    inputs = [(input_content, temp_prm, top_p_prm) for input_content, temp_prm, top_p_prm in zip(input_contents, temp_prms, top_p_prms)]
    with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
        future_to_function = {executor.submit(process_file_double, *input_args): process_file_double for input_args in inputs}
        for idx, future in enumerate(concurrent.futures.as_completed(future_to_function), start=1):
            function = future_to_function[future]
            try:
                rust_codes = future.result()
                # 对于每个函数的输出，写入不同的文件
                for code in rust_codes:
                    temp_file_name = f"result_dbl_{idx}.rs"
                    temp_file_path = os.path.join(results_dir, temp_file_name)
                    with open(temp_file_path, 'w', encoding='utf-8') as file:
                        file.write(code)
            except Exception as exc:
                print(f'{function.__name__} generated an exception: {exc}')

    bool_tag = compile_and_select_best(input_dir, results_dir, success_dir, test_compile_dir)
    return bool_tag

# 输入、结果、成功和测试编译目录路径
input_directory = "input"
results_directory = "results"
success_directory = "success"
test_compile_directory = "compile_test"

if not os.path.exists(input_directory):
    os.makedirs(input_directory)
input_files = os.listdir(input_directory)

# 执行函数
has_success_files = False
if not has_success_files:
    has_success_files = process_input_files_s(input_directory, results_directory, success_directory, test_compile_directory)
    if not has_success_files:
        has_success_files = process_input_files_concurrently(input_directory, results_directory, success_directory, test_compile_directory)
        if not has_success_files:
            input_file_path = os.path.join(input_directory, input_files[0])
            if not os.path.exists('c2rust_test'):
                os.makedirs('c2rust_test')
            c2rust_test_dir = 'c2rust_test'
            # 复制文件到c2rust_test目录并重命名为test.c
            shutil.copy(input_file_path, 'c2rust_test/test.c')
            
            # 以下是模拟的命令执行步骤，实际应用中需要替换为相应的os.system调用或subprocess调用
            try:
                # 在c2rust_test目录下执行C到Rust的转换
                subprocess.run(['intercept-build', 'sh', '-c', "cc test.c"], cwd=c2rust_test_dir, check=True)
                subprocess.run(['c2rust', 'transpile', '--emit-build-files', 'compile_commands.json'], cwd=c2rust_test_dir, check=True)
                subprocess.run(['cargo', 'build'], cwd=c2rust_test_dir, check=True)
                # 构建成功，处理success目录
                if os.path.exists('success'):
                    for file in os.listdir('success'):
                        os.remove(os.path.join('success', file))
                else:
                    os.makedirs('success')
                
                # 复制构建成功的Rust文件
                shutil.copy('c2rust_test/test.rs', 'success/test.rs')
                
                result = "Build successful, and test.rs has been moved to success directory."
            except subprocess.CalledProcessError as e:
                result = f"Error occurred during build process: {e}"
            except Exception as e:
                result = f"Error occurred: {str(e)}"
            shutil.rmtree(c2rust_test_dir)
            os.makedirs(c2rust_test_dir)
            with open('c2rust_test/.gitignore', 'w') as f:
                f.write('*')


for file in os.listdir('results'):
    os.remove(os.path.join('results', file))
with open('results/.gitignore', 'w') as f:
        f.write('*')

for file in os.listdir('success'):
    os.rename(os.path.join('success', file), os.path.join('success', 'result.rs'))



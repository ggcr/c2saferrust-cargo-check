from pathlib import Path
import os
import subprocess
import re
import json

def run(command):
    try:
        result = subprocess.run(
            command,
            shell=True,
            timeout=25,
            stderr=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
        if result.returncode != 0:
            exec_output = result.stderr.decode('utf-8')
            if exec_output.strip() == '':
                exec_output = result.stdout.decode('utf-8')
            raise RuntimeError(exec_output)
    except subprocess.TimeoutExpired:
        raise RuntimeError("Timeout")
    
    # return the output of the command
    return result.stdout.decode('utf-8')

def run_slicer(code_dir, ofile_name, max_lines=100000):
    # Run slicer to create program slices
    cwd = os.getcwd()
    cmd = 'cd {} && RUSTFLAGS="-Awarnings" cargo slicer -- -Zoutput-file={} -Zmax-lines={}'.format(code_dir, ofile_name, max_lines)
    os.system(cmd)
    os.chdir(cwd)

if __name__ == "__main__":

    all_metrics = {}
    root_dir = Path('.').resolve()

    # Go to each directory in src/{}
    for dir in Path('src').glob('*'):
        if not dir.is_file():
            dir = dir.name
            print(f"Processing {dir}...")
            os.chdir(root_dir)
            original_subdir = Path('src/{}/rust'.format(dir))
            translated_subdir = Path('src/{}/rust_WIP'.format(dir))
            if not original_subdir.exists() or not translated_subdir.exists():
                continue
            try:
                run_slicer(original_subdir, 'slices.json')
            except RuntimeError as e:
                print(f"Error in {original_subdir}")
                continue
            # Get all .rs files in this directory
            rust_files = list(Path(original_subdir).glob('**/*.rs'))
            # Compute the number of lines in each file
            lines = [int(run(f"wc -l {str(file)}").split()[0]) for file in rust_files]
            os.chdir(root_dir)

            functions = json.loads(Path(original_subdir, 'slices.json').read_text())
            func_lengths = [int(func['num_lines']) for func in functions]

            all_metrics[dir] = {
                "lines": sum(lines),
                "num_functions": len(functions),
                "max_length": max(func_lengths),
                "avg_length": sum(func_lengths) / len(func_lengths)
            }
    
    os.chdir(root_dir)
    # Write this dict to json
    with open('dataset_stats.json', 'w') as f:
        json.dump(all_metrics, f, indent=4)
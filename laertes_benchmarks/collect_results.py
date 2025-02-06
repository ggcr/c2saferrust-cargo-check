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
            timeout=120,
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

def parse_metrics_output(metrics_output):
    metrics = {}
    for line in metrics_output.split('\n'):
        
        # If the line is of the format "Unsafe spans: <num>", extract the num
        unsafe_spans = re.match(r'Unsafe lines: (\d+)', line)
        if unsafe_spans:
            if 'unsafe_spans' in metrics:
                metrics['unsafe_spans'] += int(unsafe_spans.group(1))
            else:
                metrics['unsafe_spans'] = int(unsafe_spans.group(1))
        
        unsafe_calls = re.match(r'Unsafe calls: (\d+)', line)
        if unsafe_calls:
            if 'unsafe_calls' in metrics:
                metrics['unsafe_calls'] += int(unsafe_calls.group(1))
            else:
                metrics['unsafe_calls'] = int(unsafe_calls.group(1))
        
        unsafe_casts = re.match(r'Unsafe casts: (\d+)', line)
        if unsafe_casts:
            if 'unsafe_casts' in metrics:
                metrics['unsafe_casts'] += int(unsafe_casts.group(1))
            else:
                metrics['unsafe_casts'] = int(unsafe_casts.group(1))
        
        # # If the line is of the format "Total spans: <num>", extract the num
        # total_spans = re.match(r'Total spans: (\d+)', line)
        # if total_spans:
        #     if 'total_spans' in metrics:
        #         metrics['total_spans'] += int(total_spans.group(1))
        #     else:
        #         metrics['total_spans'] = int(total_spans.group(1))
        
        # If the line is of the format "Raw pointer dereferences: <num>", extract the num
        raw_pointer_derefs = re.match(r'Raw pointer dereferences: (\d+)', line)
        if raw_pointer_derefs:
            if 'raw_pointer_derefs' in metrics:
                metrics['raw_pointer_derefs'] += int(raw_pointer_derefs.group(1))
            else:
                metrics['raw_pointer_derefs'] = int(raw_pointer_derefs.group(1))
        
        # If the line is of the format "Raw pointer declarations: <num>", extract the num
        raw_pointer_decls = re.match(r'Raw pointer declarations: (\d+)', line)
        if raw_pointer_decls:
            if 'raw_pointer_decls' in metrics:
                metrics['raw_pointer_decls'] += int(raw_pointer_decls.group(1))
            else:
                metrics['raw_pointer_decls'] = int(raw_pointer_decls.group(1))
        
    return metrics

if __name__ == "__main__":

    all_metrics = {}
    root_dir = Path('.').resolve()

    # Go to each directory in coreutils/src/{}
    for dir in Path('.').glob('*'):
        if not dir.is_file():
            dir = dir.name
            if '_' in dir:
                continue
            # Now we have the name of a utility, like bzip2
            print(f"Processing {dir}...")
            original_subdir = Path(dir)
            resolveimports_subdir = Path(f'{dir}_resolveimports')
            translated_subdir = Path(f'{dir}_resolveimports_WIP')
            laertes_subdir = Path(f'{dir}_laertes')
            crown_subdir = Path(f'{dir}_crown')

            os.chdir(root_dir)
            os.chdir(original_subdir)
            try:
                original_metrics = run('cargo metrics')
            except RuntimeError as e:
                print(f"Error in {original_subdir}")
                print(e)
                original_metrics = ''
            
            os.chdir(root_dir)
            if resolveimports_subdir.exists():
                os.chdir(resolveimports_subdir)
                try:
                    resolveimports_metrics = run('cargo metrics')
                except RuntimeError as e:
                    print(f"Error in {resolveimports_subdir}")
                    print(e)
                    resolveimports_metrics = ''
            else:
                resolveimports_metrics = ''

            os.chdir(root_dir)
            if not translated_subdir.exists():
                continue
            os.chdir(translated_subdir)
            try:
                translated_metrics = run('cargo metrics')
            except RuntimeError as e:
                print(f"Error in {translated_subdir}")
                print(e)
                translated_metrics = ''

            os.chdir(root_dir)
            os.chdir(laertes_subdir)
            try:
                laertes_metrics = run('cargo metrics')
            except RuntimeError as e:
                print(f"Error in {laertes_subdir}")
                print(e)
                laertes_metrics = ''

            os.chdir(root_dir)
            if crown_subdir.exists():
                os.chdir(crown_subdir)
                try:
                    crown_metrics = run('cargo metrics')
                except RuntimeError as e:
                    print(f"Error in {crown_subdir}")
                    print(e)
                    crown_metrics = ''
                os.chdir(root_dir)
            else:
                crown_metrics = ''
            
            all_metrics[dir] = {
                'original': parse_metrics_output(original_metrics),
                'resolveimports': parse_metrics_output(resolveimports_metrics),
                'translated': parse_metrics_output(translated_metrics),
                'laertes': parse_metrics_output(laertes_metrics),
                'crown': parse_metrics_output(crown_metrics)
            }
    
    os.chdir(root_dir)
    # Write this dict to json
    with open('all_metrics.json', 'w') as f:
        json.dump(all_metrics, f, indent=4)
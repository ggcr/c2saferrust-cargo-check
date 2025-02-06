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

if __name__ == "__main__":

    all_results = []
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
            # Read log.txt
            log_file = Path(translated_subdir, 'log.txt')
            if not log_file.exists():
                continue
            log = log_file.read_text()
            # Read slices.json from original_subdir
            functions = json.loads(Path(original_subdir, 'slices.json').read_text())
            for entry in log.split('\n'):
                if entry.strip() == '':
                    continue
                func_defid, success, attempts = entry.split(',')
                if 'Chunk root' not in func_defid:
                    continue
                defid = func_defid.split('Chunk')[0].strip()
                func = [f for f in functions if f['func_defid'] == defid][0]
                if func['num_lines'] >= 150:
                    continue
                all_results += [(defid, func['num_lines'], int(success.strip()=="Success"), int(attempts.strip()))]

    os.chdir(root_dir)
    # Make a bar chart of the success rate vs function length
    import matplotlib
    import matplotlib.pyplot as plt
    import numpy as np
    import pandas as pd

    df = pd.DataFrame(all_results, columns=['name', 'length', 'success', 'attempts'])

    # Write df to a file
    df.to_csv('length_data.csv', index=False)
    
    # Define bins (0-9, 10-19, ..., 140-149)
    bins = np.arange(0, 151, 10)  # 0 to 150 in intervals of 10
    labels = [f'{i}-{i+9}' for i in bins[:-1]]

    FONTSIZE = 30

    # Assign each length to a bin
    df['length_bin'] = pd.cut(df['length'], bins=bins, labels=labels, right=False)

    # Calculate the average success rate and count of functions for each length bin
    avg_success_rate = df.groupby('length_bin')['success'].mean()
    count_per_bin = df['length_bin'].value_counts().sort_index()  # Sorting by index for consistent order

    # Plotting
    fig, ax1 = plt.subplots(figsize=(12, 6))

    # Line plot for average success rate
    ax1.plot(avg_success_rate.index.astype(str), avg_success_rate.values, color='blue', marker='o', linestyle='-', label='Translation Success Rate')
    ax1.set_xlabel('Function Length (Number of Lines)', fontsize=33)
    ax1.set_ylabel('Success Rate', fontsize=33)
    ax1.set_ylim(0, 1)  # Success rate is between 0 and 1

    # Secondary axis for count of functions in each bin
    ax2 = ax1.twinx()
    ax2.bar(count_per_bin.index.astype(str), count_per_bin.values, color='orange', alpha=0.5, label='Number of Functions')
    ax2.set_ylabel('Number of Functions', fontsize=33)

    # Set y-limit for ax2 with some padding at the top
    ax2.set_ylim(0, max(count_per_bin.values) * 1.2)  # 20% padding above the highest bar

    skip_labels = [l if i % 3 == 0 else '' for i, l in enumerate(labels)]
    # Rotate x-axis labels
    ax1.set_xticks(range(len(skip_labels)))  # Ensure there are ticks for each label
    ax1.set_xticklabels(skip_labels, rotation=0, fontsize=25)  # Apply rotation to labels

    ax1.tick_params(axis='y', labelsize=25)
    ax2.tick_params(axis='y', labelsize=25)

    # Adding legends
    ax1.legend(loc='upper right', fontsize=27, bbox_to_anchor=(1, 0.88))
    ax2.legend(loc='upper right', fontsize=27, bbox_to_anchor=(1, 1.03))
    plt.savefig('success_rate_vs_length.pdf', format='pdf', bbox_inches='tight')
    # plt.savefig('success_rate_vs_length.png', bbox_inches='tight')
    plt.close()

    # --------------------------------------- FIRST PLOT DONE ---------------------------------------

    # Make a bar chart of the average number of attempts vs function length
    # but only for successful functions
    df = df[df['success'] == 1]
    
    # Assign each length to a bin
    df['length_bin'] = pd.cut(df['length'], bins=bins, labels=labels, right=False)

    # Calculate the average success rate and count of functions for each length bin
    avg_attempts = df.groupby('length_bin')['attempts'].mean()
    count_per_bin = df['length_bin'].value_counts().sort_index()  # Sorting by index for consistent order

    # Plotting
    fig, ax1 = plt.subplots(figsize=(12, 6))

    # Line plot for average success rate
    ax1.plot(avg_attempts.index.astype(str), avg_attempts.values, color='blue', marker='o', linestyle='-', label='Avg no. attempts')
    ax1.set_xlabel('Function Length (Number of Lines)', fontsize=33)
    ax1.set_ylabel('Avg num of attempts', fontsize=33)
    ax1.set_ylim(0, 5.5)  # Success rate is between 0 and 1
    # ax1.set_title('Average Translation Success Rate versus Function Length')

    # Secondary axis for count of functions in each bin
    ax2 = ax1.twinx()
    ax2.bar(count_per_bin.index.astype(str), count_per_bin.values, color='orange', alpha=0.5, label='#Func Successfully Translated')
    ax2.set_ylabel('#Func Successfully\nTranslated', fontsize=33)

    # Set y-limit for ax2 with some padding at the top
    ax2.set_ylim(0, max(count_per_bin.values) * 1.2)  # 20% padding above the highest bar

    skip_labels = [l if i % 3 == 0 else '' for i, l in enumerate(labels)]
    # Rotate x-axis labels
    ax1.set_xticks(range(len(skip_labels)))  # Ensure there are ticks for each label
    ax1.set_xticklabels(skip_labels, rotation=0, fontsize=25)  # Apply rotation to labels

    ax1.tick_params(axis='y', labelsize=25)
    ax2.tick_params(axis='y', labelsize=25)

    # Adding legends
    ax1.legend(loc='upper right', fontsize=27, bbox_to_anchor=(1, 0.3))
    ax2.legend(loc='upper right', fontsize=27, bbox_to_anchor=(1, 1.03))
    plt.savefig('attempts_vs_length.pdf', format='pdf', bbox_inches='tight')
    # plt.savefig('attempts_vs_length.png', bbox_inches='tight')
    plt.close()
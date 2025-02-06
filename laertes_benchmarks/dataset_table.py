import json

with open('dataset_stats.json', 'r') as f:
    stats = json.loads(f.read())

selected_programs = ['tulipindicators', 'xzoom', 'genann', 'optipng', 'urlparser', 'lil', 'qsort', 'snudown', 'grabc', 'bzip2']

for program in selected_programs:
    print(f'& {program} & {stats[program]["lines"]} & {stats[program]["num_functions"]} & {stats[program]["max_length"]} & {stats[program]["avg_length"]:.2f} \\\\')
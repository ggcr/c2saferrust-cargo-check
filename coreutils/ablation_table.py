import json

with open('coreutils_metrics.json', 'r') as f:
    data = json.load(f)

selected_programs = ['cat', 'uniq', 'head']

for program in data.keys():
    if program not in selected_programs:
        continue
    original = data[program]['original']
    translated = data[program]['translated']

    try: # If any of the metrics are missing, skip this program
        randomized = data[program]['randomized']
        nochunking = data[program]['nochunking']

        original_unsafe = original['unsafe_spans']
        original_decls = original['raw_pointer_decls']
        original_derefs = original['raw_pointer_derefs']
        original_calls = original['unsafe_calls']
        original_casts = original['unsafe_casts']

        translated_unsafe = translated['unsafe_spans']
        translated_decls = translated['raw_pointer_decls']
        translated_derefs = translated['raw_pointer_derefs']
        translated_calls = translated['unsafe_calls']
        translated_casts = translated['unsafe_casts']

        nochunking_unsafe = nochunking['unsafe_spans']
        nochunking_decls = nochunking['raw_pointer_decls']
        nochunking_derefs = nochunking['raw_pointer_derefs']
        nochunking_calls = nochunking['unsafe_calls']
        nochunking_casts = nochunking['unsafe_casts']

        randomized_unsafe = randomized['unsafe_spans']
        randomized_decls = randomized['raw_pointer_decls']
        randomized_derefs = randomized['raw_pointer_derefs']
        randomized_calls = randomized['unsafe_calls']
        randomized_casts = randomized['unsafe_casts']
    except:
        continue

    improvement_unsafe = 100 * (original_unsafe - translated_unsafe) / original_unsafe
    improvement_decls = 100 * (original_decls - translated_decls) / original_decls
    improvement_derefs = 100 * (original_derefs - translated_derefs) / original_derefs
    improvement_calls = 100 * (original_calls - translated_calls) / original_calls
    improvement_casts = 100 * (original_casts - translated_casts) / original_casts

    improvement_unsafe_nochunking = 100 * (original_unsafe - nochunking_unsafe) / original_unsafe
    improvement_decls_nochunking = 100 * (original_decls - nochunking_decls) / original_decls
    improvement_derefs_nochunking = 100 * (original_derefs - nochunking_derefs) / original_derefs
    improvement_calls_nochunking = 100 * (original_calls - nochunking_calls) / original_calls
    improvement_casts_nochunking = 100 * (original_casts - nochunking_casts) / original_casts

    improvement_unsafe_randomized = 100 * (original_unsafe - randomized_unsafe) / original_unsafe
    improvement_decls_randomized = 100 * (original_decls - randomized_decls) / original_decls
    improvement_derefs_randomized = 100 * (original_derefs - randomized_derefs) / original_derefs
    improvement_calls_randomized = 100 * (original_calls - randomized_calls) / original_calls
    improvement_casts_randomized = 100 * (original_casts - randomized_casts) / original_casts

    all_improvement_unsafe = [improvement_unsafe, improvement_unsafe_nochunking, improvement_unsafe_randomized]
    all_improvement_decls = [improvement_decls, improvement_decls_nochunking, improvement_decls_randomized]
    all_improvement_derefs = [improvement_derefs, improvement_derefs_nochunking, improvement_derefs_randomized]
    all_improvement_calls = [improvement_calls, improvement_calls_nochunking, improvement_calls_randomized]
    all_improvement_casts = [improvement_casts, improvement_casts_nochunking, improvement_casts_randomized]

    # Add bold formatting to whichever is greater - improvement_unsafe or improvement_unsafe_nochunking
    # Similarly for decls and derefs
    unsafe_bold1 = '\\cellcolor{blue!12}{' if improvement_unsafe == max(all_improvement_unsafe) else ''
    unsafe_bold2 = '}' if improvement_unsafe == max(all_improvement_unsafe) else ''
    decls_bold1 = '\\cellcolor{blue!12}{' if improvement_decls == max(all_improvement_decls) else ''
    decls_bold2 = '}' if improvement_decls == max(all_improvement_decls) else ''
    derefs_bold1 = '\\cellcolor{blue!12}{' if improvement_derefs == max(all_improvement_derefs) else ''
    derefs_bold2 = '}' if improvement_derefs == max(all_improvement_derefs) else ''
    calls_bold1 = '\\cellcolor{blue!12}{' if improvement_calls == max(all_improvement_calls) else ''
    calls_bold2 = '}' if improvement_calls == max(all_improvement_calls) else ''
    casts_bold1 = '\\cellcolor{blue!12}{' if improvement_casts == max(all_improvement_casts) else ''
    casts_bold2 = '}' if improvement_casts == max(all_improvement_casts) else ''

    unsafe_bold1_nochunking = '\\cellcolor{blue!12}{' if improvement_unsafe_nochunking == max(all_improvement_unsafe) else ''
    unsafe_bold2_nochunking = '}' if improvement_unsafe_nochunking == max(all_improvement_unsafe) else ''
    decls_bold1_nochunking = '\\cellcolor{blue!12}{' if improvement_decls_nochunking == max(all_improvement_decls) else ''
    decls_bold2_nochunking = '}' if improvement_decls_nochunking == max(all_improvement_decls) else ''
    derefs_bold1_nochunking = '\\cellcolor{blue!12}{' if improvement_derefs_nochunking == max(all_improvement_derefs) else ''
    derefs_bold2_nochunking = '}' if improvement_derefs_nochunking == max(all_improvement_derefs) else ''
    calls_bold1_nochunking = '\\cellcolor{blue!12}{' if improvement_calls_nochunking == max(all_improvement_calls) else ''
    calls_bold2_nochunking = '}' if improvement_calls_nochunking == max(all_improvement_calls) else ''
    casts_bold1_nochunking = '\\cellcolor{blue!12}{' if improvement_casts_nochunking == max(all_improvement_casts) else ''
    casts_bold2_nochunking = '}' if improvement_casts_nochunking == max(all_improvement_casts) else ''

    unsafe_bold1_randomized = '\\cellcolor{blue!12}{' if improvement_unsafe_randomized == max(all_improvement_unsafe) else ''
    unsafe_bold2_randomized = '}' if improvement_unsafe_randomized == max(all_improvement_unsafe) else ''
    decls_bold1_randomized = '\\cellcolor{blue!12}{' if improvement_decls_randomized == max(all_improvement_decls) else ''
    decls_bold2_randomized = '}' if improvement_decls_randomized == max(all_improvement_decls) else ''
    derefs_bold1_randomized = '\\cellcolor{blue!12}{' if improvement_derefs_randomized == max(all_improvement_derefs) else ''
    derefs_bold2_randomized = '}' if improvement_derefs_randomized == max(all_improvement_derefs) else ''
    calls_bold1_randomized = '\\cellcolor{blue!12}{' if improvement_calls_randomized == max(all_improvement_calls) else ''
    calls_bold2_randomized = '}' if improvement_calls_randomized == max(all_improvement_calls) else ''
    casts_bold1_randomized = '\\cellcolor{blue!12}{' if improvement_casts_randomized == max(all_improvement_casts) else ''
    casts_bold2_randomized = '}' if improvement_casts_randomized == max(all_improvement_casts) else ''

    print(f'\\multirow{{3}}{{*}}{{\\textbf{{{program}}}}}', end='')
    # print(f' & Original & {original_decls} &  & {original_derefs} & & {original_unsafe} & & {original_calls} & & {original_casts} & \\\\')
    
    # print(f'& No Chunking & {decls_bold1_nochunking}{nochunking_decls}{decls_bold2_nochunking} & {decls_bold1_nochunking}{improvement_decls_nochunking:.0f}{decls_bold2_nochunking}', end='')
    # print(f'& {derefs_bold1_nochunking}{nochunking_derefs}{derefs_bold2_nochunking} & {derefs_bold1_nochunking}{improvement_derefs_nochunking:.0f}{derefs_bold2_nochunking}', end='')
    # print(f'& {unsafe_bold1_nochunking}{nochunking_unsafe}{unsafe_bold2_nochunking} & {unsafe_bold1_nochunking}{improvement_unsafe_nochunking:.0f}{unsafe_bold2_nochunking}', end='')
    # print(f'& {calls_bold1_nochunking}{nochunking_calls}{calls_bold2_nochunking} & {calls_bold1_nochunking}{improvement_calls_nochunking:.0f}{calls_bold2_nochunking}', end='')
    # print(f'& {casts_bold1_nochunking}{nochunking_casts}{casts_bold2_nochunking} & {casts_bold1_nochunking}{improvement_casts_nochunking:.0f}{casts_bold2_nochunking} \\\\')

    # print(f' & Random Order & {decls_bold1_randomized}{randomized_decls}{decls_bold2_randomized} & {decls_bold1_randomized}{improvement_decls_randomized:.0f}{decls_bold2_randomized}', end='')
    # print(f' & {derefs_bold1_randomized}{randomized_derefs}{derefs_bold2_randomized} & {derefs_bold1_randomized}{improvement_derefs_randomized:.0f}{derefs_bold2_randomized}', end='')
    # print(f' & {unsafe_bold1_randomized}{randomized_unsafe}{unsafe_bold2_randomized} & {unsafe_bold1_randomized}{improvement_unsafe_randomized:.0f}{unsafe_bold2_randomized}', end='')
    # print(f' & {calls_bold1_randomized}{randomized_calls}{calls_bold2_randomized} & {calls_bold1_randomized}{improvement_calls_randomized:.0f}{calls_bold2_randomized}', end='')
    # print(f' & {casts_bold1_randomized}{randomized_casts}{casts_bold2_randomized} & {casts_bold1_randomized}{improvement_casts_randomized:.0f}{casts_bold2_randomized} \\\\')

    # print(f' & Our Tool & {decls_bold1}{translated_decls}{decls_bold2} & {decls_bold1}{improvement_decls:.0f}{decls_bold2}', end='')
    # print(f' & {derefs_bold1}{translated_derefs}{derefs_bold2} & {derefs_bold1}{improvement_derefs:.0f}{derefs_bold2}', end='')
    # print(f' & {unsafe_bold1}{translated_unsafe}{unsafe_bold2} & {unsafe_bold1}{improvement_unsafe:.0f}{unsafe_bold2}', end='')
    # print(f' & {calls_bold1}{translated_calls}{calls_bold2} & {calls_bold1}{improvement_calls:.0f}{calls_bold2}', end='')
    # print(f' & {casts_bold1}{translated_casts}{casts_bold2} & {casts_bold1}{improvement_casts:.0f}{casts_bold2} \\\\')

    print(f' & Original & {original_decls} & {original_derefs} & {original_unsafe} & {original_calls} & {original_casts} & \\\\')

    print(f'& No Chunking & {decls_bold1_nochunking}{nochunking_decls}{decls_bold2_nochunking} ', end='')
    print(f'& {derefs_bold1_nochunking}{nochunking_derefs}{derefs_bold2_nochunking} ', end='')
    print(f'& {unsafe_bold1_nochunking}{nochunking_unsafe}{unsafe_bold2_nochunking} ', end='')
    print(f'& {calls_bold1_nochunking}{nochunking_calls}{calls_bold2_nochunking} ', end='')
    print(f'& {casts_bold1_nochunking}{nochunking_casts}{casts_bold2_nochunking} \\\\')

    print(f' & Random Order & {decls_bold1_randomized}{randomized_decls}{decls_bold2_randomized} ', end='')
    print(f' & {derefs_bold1_randomized}{randomized_derefs}{derefs_bold2_randomized} ', end='')
    print(f' & {unsafe_bold1_randomized}{randomized_unsafe}{unsafe_bold2_randomized} ', end='')
    print(f' & {calls_bold1_randomized}{randomized_calls}{calls_bold2_randomized} ', end='')
    print(f' & {casts_bold1_randomized}{randomized_casts}{casts_bold2_randomized} \\\\')

    print(f' & Our Tool & {decls_bold1}{translated_decls}{decls_bold2} ', end='')
    print(f' & {derefs_bold1}{translated_derefs}{derefs_bold2} ', end='')
    print(f' & {unsafe_bold1}{translated_unsafe}{unsafe_bold2} ', end='')
    print(f' & {calls_bold1}{translated_calls}{calls_bold2} ', end='')
    print(f' & {casts_bold1}{translated_casts}{casts_bold2} \\\\')

    print('\\hline')



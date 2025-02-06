import json

with open('all_metrics.json', 'r') as f:
    data = json.load(f)

order = ['bzip2', 'genann', 'lil', 'tulipindicators', 'urlparser', 'grabc', 'optipng', 'qsort', 'snudown', 'xzoom']

for program in order:
    original = data[program]['original']
    resolveimports = data[program]['resolveimports']
    translated = data[program]['translated']
    laertes = data[program]['laertes']
    crown = data[program]['crown']

    try: # If any of the metrics are missing, skip this program
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

        laertes_unsafe = laertes['unsafe_spans']
        laertes_decls = laertes['raw_pointer_decls']
        laertes_derefs = laertes['raw_pointer_derefs']
        laertes_calls = laertes['unsafe_calls']
        laertes_casts = laertes['unsafe_casts']
    except:
        continue

    try:
        crown_unsafe = crown['unsafe_spans']
        crown_decls = crown['raw_pointer_decls']
        crown_derefs = crown['raw_pointer_derefs']
        crown_calls = crown['unsafe_calls']
        crown_casts = crown['unsafe_casts']

        improvement_unsafe_crown = 100 * (original_unsafe - crown_unsafe) / original_unsafe
        improvement_decls_crown = 100 * (original_decls - crown_decls) / original_decls
        improvement_derefs_crown = 100 * (original_derefs - crown_derefs) / original_derefs
        improvement_calls_crown = 100 * (original_calls - crown_calls) / original_calls
        improvement_casts_crown = 100 * (original_casts - crown_casts) / original_casts

        crown_present = True
    except:
        improvement_unsafe_crown = -1e9
        improvement_decls_crown = -1e9
        improvement_derefs_crown = -1e9
        improvement_calls_crown = -1e9
        improvement_casts_crown = -1e9

        crown_present = False
    
    improvement_unsafe = 100 * (original_unsafe - translated_unsafe) / original_unsafe
    improvement_decls = 100 * (original_decls - translated_decls) / original_decls
    improvement_derefs = 100 * (original_derefs - translated_derefs) / original_derefs
    improvement_calls = 100 * (original_calls - translated_calls) / original_calls
    improvement_casts = 100 * (original_casts - translated_casts) / original_casts

    improvement_unsafe_laertes = 100 * (original_unsafe - laertes_unsafe) / original_unsafe
    improvement_decls_laertes = 100 * (original_decls - laertes_decls) / original_decls
    improvement_derefs_laertes = 100 * (original_derefs - laertes_derefs) / original_derefs
    improvement_calls_laertes = 100 * (original_calls - laertes_calls) / original_calls
    improvement_casts_laertes = 100 * (original_casts - laertes_casts) / original_casts

    all_improvement_unsafe = [improvement_unsafe, improvement_unsafe_laertes, improvement_unsafe_crown]
    all_improvement_decls = [improvement_decls, improvement_decls_laertes, improvement_decls_crown]
    all_improvement_derefs = [improvement_derefs, improvement_derefs_laertes, improvement_derefs_crown]
    all_improvement_calls = [improvement_calls, improvement_calls_laertes, improvement_calls_crown]
    all_improvement_casts = [improvement_casts, improvement_casts_laertes, improvement_casts_crown]

    # Add bold formatting to whichever is greater - improvement_unsafe or improvement_unsafe_laertes
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

    unsafe_bold1_laertes = '\\cellcolor{blue!12}{' if improvement_unsafe_laertes == max(all_improvement_unsafe) else ''
    unsafe_bold2_laertes = '}' if improvement_unsafe_laertes == max(all_improvement_unsafe) else ''
    decls_bold1_laertes = '\\cellcolor{blue!12}{' if improvement_decls_laertes == max(all_improvement_decls) else ''
    decls_bold2_laertes = '}' if improvement_decls_laertes == max(all_improvement_decls) else ''
    derefs_bold1_laertes = '\\cellcolor{blue!12}{' if improvement_derefs_laertes == max(all_improvement_derefs) else ''
    derefs_bold2_laertes = '}' if improvement_derefs_laertes == max(all_improvement_derefs) else ''
    calls_bold1_laertes = '\\cellcolor{blue!12}{' if improvement_calls_laertes == max(all_improvement_calls) else ''
    calls_bold2_laertes = '}' if improvement_calls_laertes == max(all_improvement_calls) else ''
    casts_bold1_laertes = '\\cellcolor{blue!12}{' if improvement_casts_laertes == max(all_improvement_casts) else ''
    casts_bold2_laertes = '}' if improvement_casts_laertes == max(all_improvement_casts) else ''

    unsafe_bold1_crown = '\\cellcolor{blue!12}{' if improvement_unsafe_crown == max(all_improvement_unsafe) else ''
    unsafe_bold2_crown = '}' if improvement_unsafe_crown == max(all_improvement_unsafe) else ''
    decls_bold1_crown = '\\cellcolor{blue!12}{' if improvement_decls_crown == max(all_improvement_decls) else ''
    decls_bold2_crown = '}' if improvement_decls_crown == max(all_improvement_decls) else ''
    derefs_bold1_crown = '\\cellcolor{blue!12}{' if improvement_derefs_crown == max(all_improvement_derefs) else ''
    derefs_bold2_crown = '}' if improvement_derefs_crown == max(all_improvement_derefs) else ''
    calls_bold1_crown = '\\cellcolor{blue!12}{' if improvement_calls_crown == max(all_improvement_calls) else ''
    calls_bold2_crown = '}' if improvement_calls_crown == max(all_improvement_calls) else ''
    casts_bold1_crown = '\\cellcolor{blue!12}{' if improvement_casts_crown == max(all_improvement_casts) else ''
    casts_bold2_crown = '}' if improvement_casts_crown == max(all_improvement_casts) else ''

    if crown_present:
        print(f'\\multicolumn{{1}}{{V{{2}}cV{{2}}}}{{\\multirow{{5.5}}{{*}}{{{program}}}}}', end='')
    else:
        print(f'\\multicolumn{{1}}{{V{{2}}cV{{2}}}}{{\\multirow{{4}}{{*}}{{{program}}}}}', end='')

    print(f' & \\multicolumn{{1}}{{cV{{2}}}}{{Baseline}} & '
          f'{original_decls} & \\multicolumn{{1}}{{rV{{2}}}}{{~}} & '
          f'{original_derefs} & \\multicolumn{{1}}{{rV{{2}}}}{{~}} & '
          f'{original_unsafe} & \\multicolumn{{1}}{{rV{{2}}}}{{~}} & '
          f'{original_calls} & \\multicolumn{{1}}{{rV{{2}}}}{{~}} & '
          f'{original_casts} & \\multicolumn{{1}}{{rV{{2}}}}{{~}} \\bigstrut \\\\')
    
    print(f'\\multicolumn{{1}}{{V{{2}}cV{{2}}}}{{}} & \\multicolumn{{1}}{{cV{{2}}}}{{Laertes}}')
    print(f'& {decls_bold1_laertes}{laertes_decls}{decls_bold2_laertes} & '
          f'\\multicolumn{{1}}{{rV{{2}}}}{{{decls_bold1_laertes}{improvement_decls_laertes:.0f}{decls_bold2_laertes}}}', end='')
    print(f'& {derefs_bold1_laertes}{laertes_derefs}{derefs_bold2_laertes} & '
          f'\\multicolumn{{1}}{{rV{{2}}}}{{{derefs_bold1_laertes}{improvement_derefs_laertes:.0f}{derefs_bold2_laertes}}}', end='')
    print(f'& {unsafe_bold1_laertes}{laertes_unsafe}{unsafe_bold2_laertes} & '
          f'\\multicolumn{{1}}{{rV{{2}}}}{{{unsafe_bold1_laertes}{improvement_unsafe_laertes:.0f}{unsafe_bold2_laertes}}}', end='')    
    print(f'& {calls_bold1_laertes}{laertes_calls}{calls_bold2_laertes} & '
          f'\\multicolumn{{1}}{{rV{{2}}}}{{{calls_bold1_laertes}{improvement_calls_laertes:.0f}{calls_bold2_laertes}}}', end='')
    print(f'& {casts_bold1_laertes}{laertes_casts}{casts_bold2_laertes} & '
          f'\\multicolumn{{1}}{{rV{{2}}}}{{{casts_bold1_laertes}{improvement_casts_laertes:.0f}{casts_bold2_laertes}}} \\bigstrut\\\\')

    if crown_present:
        print(f'\\multicolumn{{1}}{{V{{2}}cV{{2}}}}{{}} & \\multicolumn{{1}}{{cV{{2}}}}{{\\textsc{{crown}}}}')
        print(f'& {decls_bold1_crown}{crown_decls}{decls_bold2_crown} & '
            f'\\multicolumn{{1}}{{rV{{2}}}}{{{decls_bold1_crown}{improvement_decls_crown:.0f}{decls_bold2_crown}}}', end='')
        print(f'& {derefs_bold1_crown}{crown_derefs}{derefs_bold2_crown} & '
            f'\\multicolumn{{1}}{{rV{{2}}}}{{{derefs_bold1_crown}{improvement_derefs_crown:.0f}{derefs_bold2_crown}}}', end='')
        print(f'& {unsafe_bold1_crown}{crown_unsafe}{unsafe_bold2_crown} & '
            f'\\multicolumn{{1}}{{rV{{2}}}}{{{unsafe_bold1_crown}{improvement_unsafe_crown:.0f}{unsafe_bold2_crown}}}', end='')    
        print(f'& {calls_bold1_crown}{crown_calls}{calls_bold2_crown} & '
            f'\\multicolumn{{1}}{{rV{{2}}}}{{{calls_bold1_crown}{improvement_calls_crown:.0f}{calls_bold2_crown}}}', end='')
        print(f'& {casts_bold1_crown}{crown_casts}{casts_bold2_crown} & '
            f'\\multicolumn{{1}}{{rV{{2}}}}{{{casts_bold1_crown}{improvement_casts_crown:.0f}{casts_bold2_crown}}} \\bigstrut\\\\')
    else:
        # print(' & CROWN & --- & --- & --- & --- & --- & --- \\\\')
        pass

    print(f'\\multicolumn{{1}}{{V{{2}}cV{{2}}}}{{}} & \\multicolumn{{1}}{{cV{{2}}}}{{\\tool}}')
    print(f'& {decls_bold1}{translated_decls}{decls_bold2} & '
        f'\\multicolumn{{1}}{{rV{{2}}}}{{{decls_bold1}{improvement_decls:.0f}{decls_bold2}}}', end='')
    print(f'& {derefs_bold1}{translated_derefs}{derefs_bold2} & '
        f'\\multicolumn{{1}}{{rV{{2}}}}{{{derefs_bold1}{improvement_derefs:.0f}{derefs_bold2}}}', end='')
    print(f'& {unsafe_bold1}{translated_unsafe}{unsafe_bold2} & '
        f'\\multicolumn{{1}}{{rV{{2}}}}{{{unsafe_bold1}{improvement_unsafe:.0f}{unsafe_bold2}}}', end='')    
    print(f'& {calls_bold1}{translated_calls}{calls_bold2} & '
        f'\\multicolumn{{1}}{{rV{{2}}}}{{{calls_bold1}{improvement_calls:.0f}{calls_bold2}}}', end='')
    print(f'& {casts_bold1}{translated_casts}{casts_bold2} & '
        f'\\multicolumn{{1}}{{rV{{2}}}}{{{casts_bold1}{improvement_casts:.0f}{casts_bold2}}} \\bigstrut\\\\')

    print('\\hlineB{2}')

print('''\\hlineB{3}
    \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} &
  \\multicolumn{1}{l}{} \\bigstrut\\\\[-2.25em] \\hlineB{3}''')
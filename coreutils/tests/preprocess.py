# Go through each .sh file and find the line ". "${srcdir=.}/tests/init.sh"; path_prepend_ $1"
# Replace it with the following two lines:
# SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
# . "$SCRIPTPATH/../../tests/init.sh"; path_prepend_ $1

import os
from pathlib import Path

def preprocess(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()

    with open(file_path, 'w') as file:
        for line in lines:
            if '. "${srcdir=.}/tests/init.sh"; path_prepend_ $1' in line:
                file.write('SCRIPTPATH=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )\n')
                file.write('. "$SCRIPTPATH/../../tests/init.sh"; path_prepend_ $1\n')
            else:
                file.write(line)

if __name__ == '__main__':
    
    for file in Path('.').rglob('*.sh'):
        print(file)
        preprocess(file)
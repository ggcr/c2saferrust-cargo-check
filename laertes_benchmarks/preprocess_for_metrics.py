# Preprocess a folder to make it compatible with the Rust version used by CROWN
# Locate the Cargo.toml and find the path of the file under `path = "<filename>"`
# Add the following to the top of the main file:
'''
#![feature(raw_ref_op)]
#![feature(strict_provenance)]
'''

import os
import sys
from pathlib import Path

def add_features_to_main_file(main_file: Path):
    print(f"Adding features to {main_file}...")
    with open(main_file, 'r') as f:
        lines = f.readlines()
    # Check if this file already has the features
    if not '#![feature(core_intrinsics)]\n' in lines:
        lines.insert(0, '#![feature(core_intrinsics)]\n')
    
    with open(main_file, 'w') as f:
        f.writelines(lines)

def preprocess_folder(folder: Path):
    for dir in [folder] + list(folder.iterdir()):
        if dir.is_dir():
            # Check if this directory has a Cargo.toml
            cargo_toml = dir / 'Cargo.toml'
            if cargo_toml.exists():
                print(f"Processing {dir}...")
                with open(cargo_toml, 'r') as f:
                    lines = f.readlines()
                for line in lines:
                    if 'path = ' in line:
                        main_file = line.split('=')[1].strip().strip('"')
                        main_file = dir / main_file
                        add_features_to_main_file(main_file)
                        break

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python preprocess_for_metrics.py <folder>")
        sys.exit(1)
    folder = Path(sys.argv[1])
    preprocess_folder(folder)

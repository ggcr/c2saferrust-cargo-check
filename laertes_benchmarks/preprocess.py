import os
import re

def remove_features_and_main(file):
    with open(file, 'r') as f:
        lines = f.readlines()

    with open(file, 'w') as f:
        for line in lines:
            if "#![feature(const_raw_ptr_to_usize_cast)]" in line:
                continue
            if "#![feature(main)]" in line:
                continue
            if "[main]" in line:
                continue
            if "const_raw_ptr_to_usize_cast" in line and "#![feature" in line:
                line = line.replace("const_raw_ptr_to_usize_cast,", "")
                line = line.replace("const_raw_ptr_to_usize_cast)", ")")
            if "main" in line and "#![feature" in line:
                line = line.replace("main,", "")
                line = line.replace("main)", ")")
            if "llvm_asm" in line and "#![feature" in line:
                line = line.replace("llvm_asm", "asm")
            if "llvm_asm!" in line:
                line = line.replace("llvm_asm!", "asm!")
            f.write(line)

if __name__ == "__main__":
    for root, dirs, files in os.walk("."):
        for file in files:
            if file.endswith(".rs"):
                remove_features_and_main(os.path.join(root, file))

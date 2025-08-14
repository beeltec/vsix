#!/usr/bin/env python3
"""
Update Homebrew formula with new version and SHA256 checksums.
"""

import sys
import re
import argparse


def update_formula(formula_path, version, sha_darwin_x86, sha_darwin_arm, sha_linux_x86):
    """Update the Homebrew formula with new version and checksums."""
    
    with open(formula_path, 'r') as f:
        content = f.read()
    
    # Update version
    content = re.sub(r'version "[^"]*"', f'version "{version}"', content)
    
    # Update download URLs
    content = re.sub(r'download/v[\d.]+', f'download/v{version}', content)
    
    # Split content into lines for easier processing
    lines = content.split('\n')
    result = []
    
    in_macos = False
    in_linux = False
    in_arm = False
    in_x86 = False
    
    for i, line in enumerate(lines):
        # Track which section we're in
        if 'on_macos do' in line:
            in_macos = True
            in_linux = False
        elif 'on_linux do' in line:
            in_linux = True
            in_macos = False
        elif line.strip() == 'end' and (in_macos or in_linux):
            in_arm = False
            in_x86 = False
            if in_macos:
                in_macos = False
            elif in_linux:
                in_linux = False
        elif 'if Hardware::CPU.arm?' in line:
            in_arm = True
            in_x86 = False
        elif line.strip() == 'else':
            if in_macos or in_linux:
                in_arm = False
                in_x86 = True
        
        # Update SHA256 based on current context
        if 'sha256' in line:
            if in_macos and in_arm:
                line = re.sub(r'sha256 "[^"]*"', f'sha256 "{sha_darwin_arm}"', line)
            elif in_macos and in_x86:
                line = re.sub(r'sha256 "[^"]*"', f'sha256 "{sha_darwin_x86}"', line)
            elif in_linux and in_x86:
                line = re.sub(r'sha256 "[^"]*"', f'sha256 "{sha_linux_x86}"', line)
        
        result.append(line)
    
    # Write updated content
    with open(formula_path, 'w') as f:
        f.write('\n'.join(result))
    
    print(f"Updated formula with version {version}")
    print(f"  macOS x86_64 SHA: {sha_darwin_x86}")
    print(f"  macOS ARM64 SHA: {sha_darwin_arm}")
    print(f"  Linux x86_64 SHA: {sha_linux_x86}")


def main():
    parser = argparse.ArgumentParser(description='Update Homebrew formula')
    parser.add_argument('formula_path', help='Path to the formula file')
    parser.add_argument('version', help='Version number (without v prefix)')
    parser.add_argument('sha_darwin_x86', help='SHA256 for macOS x86_64')
    parser.add_argument('sha_darwin_arm', help='SHA256 for macOS ARM64')
    parser.add_argument('sha_linux_x86', help='SHA256 for Linux x86_64')
    
    args = parser.parse_args()
    
    update_formula(
        args.formula_path,
        args.version,
        args.sha_darwin_x86,
        args.sha_darwin_arm,
        args.sha_linux_x86
    )


if __name__ == '__main__':
    main()
#!/usr/bin/env python3
"""
Test that the encoding fix actually solves the problem
"""

import json
import os
from pathlib import Path

def test_encoding_fix():
    """Verify the encoding matches actual directory names"""
    
    claude_dir = Path.home() / ".claude" / "projects"
    
    print("=== ACTUAL DIRECTORY NAMES vs ENCODED PATHS ===\n")
    
    # Get some actual directories with underscores
    problem_dirs = []
    for d in claude_dir.iterdir():
        if d.is_dir() and "local-dev" in d.name:
            problem_dirs.append(d)
    
    print(f"Found {len(problem_dirs)} directories with 'local-dev' pattern\n")
    
    # For each directory, check if we can find matching cwd entries
    mismatches = []
    matches = []
    
    for dir_path in problem_dirs[:5]:  # Check first 5
        dir_name = dir_path.name
        print(f"Directory: {dir_name}")
        
        # Find a JSONL file in this directory
        jsonl_files = list(dir_path.glob("*.jsonl"))
        if jsonl_files:
            # Read first file to get cwd
            with open(jsonl_files[0], 'r') as f:
                for line in f:
                    try:
                        obj = json.loads(line.strip())
                        if 'cwd' in obj:
                            cwd = obj['cwd']
                            
                            # Test both encoding methods
                            old_encoding = cwd.replace('/', '-')
                            new_encoding = cwd.replace('/', '-').replace('_', '-')
                            
                            print(f"  CWD: {cwd}")
                            print(f"  Old encoding: {old_encoding}")
                            print(f"  New encoding: {new_encoding}")
                            print(f"  Actual dir:   {dir_name}")
                            
                            if new_encoding == dir_name:
                                print(f"  ✓ NEW ENCODING MATCHES!")
                                matches.append((cwd, dir_name))
                            elif old_encoding == dir_name:
                                print(f"  ✗ Old encoding matches (this shouldn't happen)")
                                mismatches.append((cwd, dir_name, "old"))
                            else:
                                print(f"  ✗ NEITHER ENCODING MATCHES!")
                                mismatches.append((cwd, dir_name, "neither"))
                            
                            print()
                            break
                    except:
                        pass
    
    print("\n=== SUMMARY ===")
    print(f"Matches: {len(matches)}")
    print(f"Mismatches: {len(mismatches)}")
    
    if mismatches:
        print("\n=== MISMATCHES ===")
        for cwd, dir_name, match_type in mismatches:
            print(f"CWD: {cwd}")
            print(f"Dir: {dir_name}")
            print(f"Match type: {match_type}")
            print()

if __name__ == "__main__":
    test_encoding_fix()
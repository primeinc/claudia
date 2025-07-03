#!/usr/bin/env python3
"""
Test a SAMPLE of JSONL files to verify schemas are correct
"""

import json
import random
from pathlib import Path
from collections import defaultdict

def test_sample_files():
    """Test a few files of each type to verify our schema understanding"""
    
    claude_dir = Path.home() / ".claude"
    
    # Find all JSONL files
    jsonl_files = list(claude_dir.rglob("*.jsonl"))
    print(f"Found {len(jsonl_files)} total JSONL files")
    
    # Group by apparent type (based on directory structure or sampling)
    file_groups = defaultdict(list)
    
    # Sample first line from each file to categorize
    for file_path in random.sample(jsonl_files, min(20, len(jsonl_files))):
        try:
            with open(file_path, 'r') as f:
                first_line = f.readline().strip()
                if first_line:
                    obj = json.loads(first_line)
                    
                    # Categorize by schema signature
                    if 'leafUuid' in obj and 'summary' in obj:
                        file_groups['summary'].append(file_path)
                    elif 'message' in obj and 'sessionId' in obj:
                        file_groups['conversation'].append(file_path)
                    else:
                        file_groups['other'].append(file_path)
        except:
            pass
    
    # Test a few lines from each type
    print("\n=== SAMPLE VALIDATION ===")
    
    for file_type, files in file_groups.items():
        print(f"\n{file_type.upper()} FILES ({len(files)} sampled):")
        
        # Test first few lines of first file
        if files:
            test_file = files[0]
            print(f"  Testing: {test_file}")
            
            try:
                with open(test_file, 'r') as f:
                    for i, line in enumerate(f):
                        if i >= 5:  # Only test first 5 lines
                            break
                        if line.strip():
                            try:
                                obj = json.loads(line)
                                print(f"    Line {i+1}: ✓ Valid JSON with {len(obj)} keys")
                                
                                # Show actual vs expected encoding for conversation entries
                                if 'cwd' in obj and file_type == 'conversation':
                                    cwd = obj['cwd']
                                    encoded = cwd.replace('/', '-').replace('_', '-')
                                    print(f"      CWD: {cwd}")
                                    print(f"      Encoded: {encoded}")
                                    
                            except json.JSONDecodeError as e:
                                print(f"    Line {i+1}: ✗ Invalid JSON - {e}")
            except Exception as e:
                print(f"    Error reading file: {e}")
    
    print("\n=== ENCODING TEST ===")
    print("Testing path encoding logic:")
    
    test_paths = [
        "/home/will/local_dev/portfolio",
        "/home/will/test",
        "/mnt/c/dev/claudia",
        "/home/user/my_project/sub_dir"
    ]
    
    for path in test_paths:
        encoded = path.replace('/', '-').replace('_', '-')
        print(f"  {path} -> {encoded}")

if __name__ == "__main__":
    test_sample_files()
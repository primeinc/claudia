#!/usr/bin/env python3
"""
Comprehensive JSONL Schema Analyzer for .claude directory
Builds complete schemas for every variant found
"""

import json
import os
import sys
from pathlib import Path
from collections import defaultdict
from genson import SchemaBuilder
from typing import Dict, List, Set, Any

def analyze_claude_schemas():
    """Analyze ALL JSONL files in ~/.claude to build comprehensive schemas"""
    
    claude_dir = Path.home() / ".claude"
    
    # Group files by their schema signature
    schema_groups = defaultdict(lambda: {
        'builder': SchemaBuilder(),
        'files': [],
        'line_count': 0,
        'error_count': 0,
        'sample_lines': []
    })
    
    # Find all JSONL files
    jsonl_files = list(claude_dir.rglob("*.jsonl"))
    print(f"Found {len(jsonl_files)} JSONL files to analyze\n")
    
    # Process each file
    for file_path in jsonl_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
                
                if not lines:
                    continue
                
                # Determine schema type from first valid line
                schema_type = None
                first_obj = None
                
                for line_num, line in enumerate(lines):
                    try:
                        obj = json.loads(line.strip())
                        if not schema_type:
                            schema_type = determine_schema_type(obj)
                            first_obj = obj
                        
                        # Add to schema builder
                        schema_groups[schema_type]['builder'].add_object(obj)
                        schema_groups[schema_type]['line_count'] += 1
                        
                        # Store samples
                        if len(schema_groups[schema_type]['sample_lines']) < 5:
                            schema_groups[schema_type]['sample_lines'].append({
                                'file': str(file_path),
                                'line': line_num + 1,
                                'data': obj
                            })
                            
                    except json.JSONDecodeError as e:
                        schema_groups[schema_type]['error_count'] += 1
                        if schema_groups[schema_type]['error_count'] < 5:
                            print(f"JSON decode error in {file_path}:{line_num+1} - {e}")
                    except Exception as e:
                        schema_groups[schema_type]['error_count'] += 1
                        if schema_groups[schema_type]['error_count'] < 5:
                            print(f"Error processing {file_path}:{line_num+1} - {e}")
                
                if schema_type:
                    schema_groups[schema_type]['files'].append(str(file_path))
                    
        except Exception as e:
            print(f"Error reading file {file_path}: {e}")
    
    return schema_groups

def determine_schema_type(obj: dict) -> str:
    """Determine schema type based on key signatures"""
    
    # Create a signature from top-level keys
    keys = sorted(obj.keys())
    
    # Special cases for known types
    if 'leafUuid' in obj and 'summary' in obj and len(keys) == 3:
        return 'summary_entry'
    
    if 'message' in obj and 'sessionId' in obj and 'timestamp' in obj:
        if 'requestId' in obj:
            return 'conversation_entry_with_request'
        return 'conversation_entry'
    
    if 'costUSD' in obj:
        return 'usage_entry'
    
    if obj.get('type') == 'checkpoint':
        return 'checkpoint_entry'
    
    # For unknown types, use key signature
    return f"type_{hash('_'.join(keys)) % 10000:04d}"

def write_schema_documentation(schema_groups: dict):
    """Write comprehensive schema documentation"""
    
    output_file = Path("/mnt/c/dev/claudia/claude_schemas_complete.md")
    
    with open(output_file, 'w') as f:
        f.write("# Complete Claude JSONL Schema Documentation\n\n")
        f.write("This document contains ALL schemas found in the .claude directory.\n")
        f.write("Generated using genson to ensure completeness.\n\n")
        
        f.write("## Summary\n\n")
        f.write("| Schema Type | Files | Total Lines | Errors |\n")
        f.write("|-------------|-------|-------------|--------|\n")
        
        for schema_type, info in sorted(schema_groups.items()):
            f.write(f"| {schema_type} | {len(info['files'])} | {info['line_count']} | {info['error_count']} |\n")
        
        f.write("\n## Detailed Schemas\n\n")
        
        for schema_type, info in sorted(schema_groups.items()):
            f.write(f"### {schema_type}\n\n")
            f.write(f"**Files:** {len(info['files'])} files\n")
            f.write(f"**Lines:** {info['line_count']} total lines processed\n")
            f.write(f"**Errors:** {info['error_count']} parsing errors\n\n")
            
            # Write the generated schema
            f.write("#### Generated Schema:\n\n")
            f.write("```json\n")
            schema = info['builder'].to_schema()
            f.write(json.dumps(schema, indent=2))
            f.write("\n```\n\n")
            
            # Write sample data
            f.write("#### Sample Data:\n\n")
            for i, sample in enumerate(info['sample_lines'][:3]):
                f.write(f"**Sample {i+1}** from `{sample['file']}` line {sample['line']}:\n\n")
                f.write("```json\n")
                f.write(json.dumps(sample['data'], indent=2)[:2000])
                if len(json.dumps(sample['data'], indent=2)) > 2000:
                    f.write("\n... (truncated)")
                f.write("\n```\n\n")
            
            # List some files
            f.write("#### Example Files:\n\n")
            for file_path in sorted(info['files'])[:5]:
                f.write(f"- `{file_path}`\n")
            if len(info['files']) > 5:
                f.write(f"- ... and {len(info['files']) - 5} more\n")
            f.write("\n---\n\n")

def write_schema_code(schema_groups: dict):
    """Write Python code with all schema definitions"""
    
    output_file = Path("/mnt/c/dev/claudia/claude_schemas.py")
    
    with open(output_file, 'w') as f:
        f.write('"""\n')
        f.write('Claude JSONL Schema Definitions\n')
        f.write('Auto-generated from actual data using genson\n')
        f.write('"""\n\n')
        f.write('from typing import Dict, Any\n\n')
        f.write('CLAUDE_SCHEMAS: Dict[str, Any] = {\n')
        
        for schema_type, info in sorted(schema_groups.items()):
            schema = info['builder'].to_schema()
            schema_str = json.dumps(schema, indent=4)
            # Indent for Python dict
            schema_str = '\n'.join('    ' + line for line in schema_str.split('\n'))
            f.write(f'    "{schema_type}": {schema_str},\n\n')
        
        f.write('}\n\n')
        f.write('def get_schema(schema_type: str) -> dict:\n')
        f.write('    """Get schema definition by type"""\n')
        f.write('    return CLAUDE_SCHEMAS.get(schema_type, {})\n')

if __name__ == "__main__":
    print("Analyzing all Claude JSONL schemas...")
    schema_groups = analyze_claude_schemas()
    
    print(f"\nFound {len(schema_groups)} distinct schema types")
    
    print("\nWriting documentation...")
    write_schema_documentation(schema_groups)
    write_schema_code(schema_groups)
    
    print("\nDone! Check:")
    print("- /mnt/c/dev/claudia/claude_schemas_complete.md for documentation")
    print("- /mnt/c/dev/claudia/claude_schemas.py for code")
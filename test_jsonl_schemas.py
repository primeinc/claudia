#!/usr/bin/env python3
"""
Test harness to validate all JSONL schemas in .claude
Tests that every file can be parsed with the discovered schemas
"""

import json
import sys
from pathlib import Path
from collections import defaultdict
from genson import SchemaBuilder
import jsonschema
from typing import Dict, List, Tuple

def load_schemas() -> Dict[str, dict]:
    """Load the generated schemas"""
    schemas = {}
    
    # Load from the generated Python file
    schema_file = Path("/mnt/c/dev/claudia/claude_schemas.py")
    if schema_file.exists():
        # Execute the file to get the schemas
        with open(schema_file) as f:
            exec_globals = {}
            exec(f.read(), exec_globals)
            schemas = exec_globals.get('CLAUDE_SCHEMAS', {})
    
    return schemas

def validate_all_jsonl_files(schemas: Dict[str, dict]) -> Dict[str, List[Tuple[str, str]]]:
    """Validate all JSONL files against their schemas"""
    
    claude_dir = Path.home() / ".claude"
    validation_errors = defaultdict(list)
    
    # Statistics
    stats = {
        'total_files': 0,
        'total_lines': 0,
        'valid_lines': 0,
        'invalid_lines': 0,
        'files_with_errors': set()
    }
    
    # Find all JSONL files
    jsonl_files = list(claude_dir.rglob("*.jsonl"))
    stats['total_files'] = len(jsonl_files)
    
    print(f"Testing {len(jsonl_files)} JSONL files...")
    
    for file_path in jsonl_files:
        file_errors = []
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                for line_num, line in enumerate(f, 1):
                    stats['total_lines'] += 1
                    
                    if not line.strip():
                        continue
                    
                    try:
                        # Parse JSON
                        obj = json.loads(line)
                        
                        # Determine schema type
                        schema_type = determine_schema_type(obj)
                        
                        # Get schema
                        if schema_type in schemas:
                            schema = schemas[schema_type]
                            
                            # Validate against schema
                            try:
                                jsonschema.validate(obj, schema)
                                stats['valid_lines'] += 1
                            except jsonschema.ValidationError as e:
                                stats['invalid_lines'] += 1
                                file_errors.append((line_num, f"Schema validation error: {e.message}"))
                        else:
                            stats['invalid_lines'] += 1
                            file_errors.append((line_num, f"Unknown schema type: {schema_type}"))
                            
                    except json.JSONDecodeError as e:
                        stats['invalid_lines'] += 1
                        file_errors.append((line_num, f"JSON parse error: {e}"))
                    except Exception as e:
                        stats['invalid_lines'] += 1
                        file_errors.append((line_num, f"Unexpected error: {e}"))
        
        except Exception as e:
            file_errors.append((0, f"File read error: {e}"))
        
        if file_errors:
            stats['files_with_errors'].add(str(file_path))
            validation_errors[str(file_path)] = file_errors
    
    # Print summary
    print(f"\n=== VALIDATION SUMMARY ===")
    print(f"Total files: {stats['total_files']}")
    print(f"Total lines: {stats['total_lines']}")
    print(f"Valid lines: {stats['valid_lines']} ({stats['valid_lines']/max(1,stats['total_lines'])*100:.1f}%)")
    print(f"Invalid lines: {stats['invalid_lines']} ({stats['invalid_lines']/max(1,stats['total_lines'])*100:.1f}%)")
    print(f"Files with errors: {len(stats['files_with_errors'])}")
    
    return validation_errors

def determine_schema_type(obj: dict) -> str:
    """Determine schema type from object"""
    if 'leafUuid' in obj and 'summary' in obj and len(obj) == 3:
        return 'summary_entry'
    
    if 'message' in obj and 'sessionId' in obj and 'timestamp' in obj:
        return 'conversation_entry'
    
    # For unknown types, use key signature
    keys = sorted(obj.keys())
    return f"type_{hash('_'.join(keys)) % 10000:04d}"

def write_validation_report(errors: Dict[str, List[Tuple[str, str]]]):
    """Write detailed validation report"""
    
    report_file = Path("/mnt/c/dev/claudia/jsonl_validation_report.md")
    
    with open(report_file, 'w') as f:
        f.write("# JSONL Validation Report\n\n")
        
        if not errors:
            f.write("✅ **All files validated successfully!**\n")
        else:
            f.write(f"❌ **Found errors in {len(errors)} files**\n\n")
            
            # Group errors by type
            error_types = defaultdict(int)
            for file_errors in errors.values():
                for _, error_msg in file_errors:
                    if "JSON parse error" in error_msg:
                        error_types["JSON parse errors"] += 1
                    elif "Schema validation error" in error_msg:
                        error_types["Schema validation errors"] += 1
                    elif "Unknown schema type" in error_msg:
                        error_types["Unknown schema types"] += 1
                    else:
                        error_types["Other errors"] += 1
            
            f.write("## Error Summary\n\n")
            for error_type, count in sorted(error_types.items()):
                f.write(f"- {error_type}: {count}\n")
            
            f.write("\n## Detailed Errors\n\n")
            
            # Write first 10 files with errors
            for i, (file_path, file_errors) in enumerate(sorted(errors.items())):
                if i >= 10:
                    f.write(f"\n... and {len(errors) - 10} more files with errors\n")
                    break
                
                f.write(f"### {file_path}\n\n")
                for line_num, error_msg in file_errors[:5]:
                    f.write(f"- Line {line_num}: {error_msg}\n")
                if len(file_errors) > 5:
                    f.write(f"- ... and {len(file_errors) - 5} more errors\n")
                f.write("\n")

if __name__ == "__main__":
    print("Loading schemas...")
    schemas = load_schemas()
    print(f"Loaded {len(schemas)} schema types")
    
    print("\nValidating all JSONL files...")
    errors = validate_all_jsonl_files(schemas)
    
    print("\nWriting validation report...")
    write_validation_report(errors)
    
    print(f"\nDone! Check /mnt/c/dev/claudia/jsonl_validation_report.md for details")
    
    # Exit with error code if validation failed
    sys.exit(1 if errors else 0)
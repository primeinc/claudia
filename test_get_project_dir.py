#!/usr/bin/env python3
"""
Test function to get the actual project directory from conversation entries
"""

import json
import sys
from pathlib import Path
from typing import Optional, Dict, List

def get_project_dir_from_entries(project_name: str) -> Optional[str]:
    """
    Given a project name (encoded directory name), find the actual project directory
    by reading the cwd field from conversation entries.
    
    Args:
        project_name: The encoded project name, e.g. "-home-will-local-dev-portfolio"
    
    Returns:
        The actual project directory path, e.g. "/home/will/local_dev/portfolio"
        or None if not found
    """
    claude_dir = Path.home() / ".claude" / "projects" / project_name
    
    if not claude_dir.exists():
        print(f"Project directory not found: {claude_dir}")
        return None
    
    # Look for any JSONL file in the project directory
    jsonl_files = list(claude_dir.glob("*.jsonl"))
    
    if not jsonl_files:
        print(f"No JSONL files found in {claude_dir}")
        return None
    
    # Try to extract cwd from the first few lines of the first file
    for jsonl_file in jsonl_files[:3]:  # Try up to 3 files
        try:
            with open(jsonl_file, 'r') as f:
                for i, line in enumerate(f):
                    if i >= 10:  # Only check first 10 lines
                        break
                    
                    try:
                        entry = json.loads(line.strip())
                        
                        # Look for cwd field
                        if 'cwd' in entry and entry['cwd']:
                            return entry['cwd']
                            
                    except json.JSONDecodeError:
                        continue
                        
        except Exception as e:
            print(f"Error reading {jsonl_file}: {e}")
            continue
    
    return None

def test_project_dir_extraction():
    """Test the function with real project directories"""
    
    # Find some real project directories
    projects_dir = Path.home() / ".claude" / "projects"
    
    if not projects_dir.exists():
        print("Claude projects directory not found")
        return
    
    # Get a sample of project directories
    project_dirs = [d for d in projects_dir.iterdir() if d.is_dir()]
    
    print(f"Found {len(project_dirs)} project directories\n")
    
    # Test with first 10 projects
    results = []
    for project_dir in sorted(project_dirs)[:10]:
        project_name = project_dir.name
        actual_dir = get_project_dir_from_entries(project_name)
        
        results.append({
            'project_name': project_name,
            'actual_dir': actual_dir,
            'found': actual_dir is not None
        })
        
        print(f"Project: {project_name}")
        print(f"  Actual dir: {actual_dir or 'NOT FOUND'}")
        print()
    
    # Summary
    found_count = sum(1 for r in results if r['found'])
    print(f"\n=== SUMMARY ===")
    print(f"Successfully extracted directory for {found_count}/{len(results)} projects")
    
    # Show encoding patterns
    print(f"\n=== ENCODING PATTERNS ===")
    for result in results:
        if result['found']:
            encoded = result['project_name']
            actual = result['actual_dir']
            
            # Verify encoding
            test_encode = actual.replace('/', '-').replace('_', '-')
            if not test_encode.startswith('-'):
                test_encode = '-' + test_encode
                
            matches = encoded == test_encode
            print(f"Encoded:  {encoded}")
            print(f"Actual:   {actual}")
            print(f"Matches:  {matches}")
            print()

def create_rust_function():
    """Generate the Rust equivalent"""
    
    rust_code = '''
/// Get the actual project directory path from JSONL entries
/// 
/// Given a project name (the encoded directory name), this function
/// reads the first JSONL file and extracts the 'cwd' field which 
/// contains the actual project path.
pub fn get_project_directory(project_name: &str) -> Option<String> {
    use crate::claude_paths::{list_claude_directory, read_claude_file};
    use serde_json::Value;
    
    // List files in the project directory
    let project_dir = format!("projects/{}", project_name);
    let files = match list_claude_directory(&project_dir) {
        Ok(files) => files,
        Err(e) => {
            log::debug!("Failed to list directory {}: {}", project_dir, e);
            return None;
        }
    };
    
    // Find a JSONL file
    let jsonl_file = files.iter()
        .find(|f| f.ends_with(".jsonl"))?;
    
    let file_path = format!("{}/{}", project_dir, jsonl_file);
    
    // Read the file and look for cwd
    let content = match read_claude_file(&file_path) {
        Ok(content) => content,
        Err(e) => {
            log::debug!("Failed to read {}: {}", file_path, e);
            return None;
        }
    };
    
    // Check first few lines for cwd
    for (i, line) in content.lines().enumerate() {
        if i >= 10 { break; } // Only check first 10 lines
        
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            if let Some(cwd) = json.get("cwd").and_then(|v| v.as_str()) {
                return Some(cwd.to_string());
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_project_directory() {
        // Test with a known project
        let project_name = "-home-will-local-dev-portfolio";
        let result = get_project_directory(project_name);
        
        assert!(result.is_some());
        if let Some(dir) = result {
            assert!(dir.contains("local_dev"));
            assert!(dir.contains("portfolio"));
        }
    }
}
'''
    
    print("\n=== RUST IMPLEMENTATION ===")
    print(rust_code)

if __name__ == "__main__":
    print("Testing project directory extraction...\n")
    test_project_dir_extraction()
    
    print("\n" + "="*80)
    create_rust_function()
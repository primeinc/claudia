"""
Claude JSONL Schema Definitions
Auto-generated from actual data using genson
"""

from typing import Dict, Any

CLAUDE_SCHEMAS: Dict[str, Any] = {
    "conversation_entry":     {
        "$schema": "http://json-schema.org/schema#",
        "type": "object",
        "properties": {
            "parentUuid": {
                "type": [
                    "null",
                    "string"
                ]
            },
            "isSidechain": {
                "type": "boolean"
            },
            "userType": {
                "type": "string"
            },
            "cwd": {
                "type": "string"
            },
            "sessionId": {
                "type": "string"
            },
            "version": {
                "type": "string"
            },
            "type": {
                "type": "string"
            },
            "message": {
                "type": "object",
                "properties": {
                    "role": {
                        "type": "string"
                    },
                    "content": {
                        "anyOf": [
                            {
                                "type": "string"
                            },
                            {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "type": {
                                            "type": "string"
                                        },
                                        "text": {
                                            "type": "string"
                                        },
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "input": {
                                            "type": "object",
                                            "properties": {
                                                "url": {
                                                    "type": "string"
                                                },
                                                "prompt": {
                                                    "type": "string"
                                                },
                                                "todos": {
                                                    "anyOf": [
                                                        {
                                                            "type": "string"
                                                        },
                                                        {
                                                            "type": "array",
                                                            "items": {
                                                                "type": "object",
                                                                "properties": {
                                                                    "id": {
                                                                        "type": "string"
                                                                    },
                                                                    "content": {
                                                                        "type": "string"
                                                                    },
                                                                    "status": {
                                                                        "type": "string"
                                                                    },
                                                                    "priority": {
                                                                        "type": "string"
                                                                    }
                                                                },
                                                                "required": [
                                                                    "content",
                                                                    "id",
                                                                    "priority",
                                                                    "status"
                                                                ]
                                                            }
                                                        }
                                                    ]
                                                },
                                                "description": {
                                                    "type": "string"
                                                },
                                                "path": {
                                                    "type": "string"
                                                },
                                                "file_path": {
                                                    "type": "string"
                                                },
                                                "pattern": {
                                                    "type": "string"
                                                },
                                                "include": {
                                                    "type": "string"
                                                },
                                                "limit": {
                                                    "type": "integer"
                                                },
                                                "command": {
                                                    "type": "string"
                                                },
                                                "offset": {
                                                    "type": "integer"
                                                },
                                                "old_string": {
                                                    "type": "string"
                                                },
                                                "new_string": {
                                                    "type": "string"
                                                },
                                                "query": {
                                                    "type": "string"
                                                },
                                                "content": {
                                                    "type": "string"
                                                },
                                                "plan": {
                                                    "type": "string"
                                                },
                                                "edits": {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "object",
                                                        "properties": {
                                                            "old_string": {
                                                                "type": "string"
                                                            },
                                                            "new_string": {
                                                                "type": "string"
                                                            },
                                                            "replace_all": {
                                                                "type": "boolean"
                                                            }
                                                        }
                                                    }
                                                },
                                                "replace_all": {
                                                    "type": "boolean"
                                                },
                                                "timeout": {
                                                    "type": "integer"
                                                },
                                                "ignore": {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "string"
                                                    }
                                                }
                                            }
                                        },
                                        "tool_use_id": {
                                            "type": "string"
                                        },
                                        "content": {
                                            "anyOf": [
                                                {
                                                    "type": "string"
                                                },
                                                {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "object",
                                                        "properties": {
                                                            "type": {
                                                                "type": "string"
                                                            },
                                                            "text": {
                                                                "type": "string"
                                                            },
                                                            "source": {
                                                                "type": "object",
                                                                "properties": {
                                                                    "type": {
                                                                        "type": "string"
                                                                    },
                                                                    "data": {
                                                                        "type": "string"
                                                                    },
                                                                    "media_type": {
                                                                        "type": "string"
                                                                    }
                                                                },
                                                                "required": [
                                                                    "data",
                                                                    "media_type",
                                                                    "type"
                                                                ]
                                                            }
                                                        },
                                                        "required": [
                                                            "type"
                                                        ]
                                                    }
                                                }
                                            ]
                                        },
                                        "thinking": {
                                            "type": "string"
                                        },
                                        "signature": {
                                            "type": "string"
                                        },
                                        "is_error": {
                                            "type": "boolean"
                                        }
                                    },
                                    "required": [
                                        "type"
                                    ]
                                }
                            }
                        ]
                    },
                    "id": {
                        "type": "string"
                    },
                    "type": {
                        "type": "string"
                    },
                    "model": {
                        "type": "string"
                    },
                    "stop_reason": {
                        "type": [
                            "null",
                            "string"
                        ]
                    },
                    "stop_sequence": {
                        "type": [
                            "null",
                            "string"
                        ]
                    },
                    "usage": {
                        "type": "object",
                        "properties": {
                            "input_tokens": {
                                "type": "integer"
                            },
                            "cache_creation_input_tokens": {
                                "type": "integer"
                            },
                            "cache_read_input_tokens": {
                                "type": "integer"
                            },
                            "output_tokens": {
                                "type": "integer"
                            },
                            "service_tier": {
                                "type": [
                                    "null",
                                    "string"
                                ]
                            },
                            "server_tool_use": {
                                "type": "object",
                                "properties": {
                                    "web_search_requests": {
                                        "type": "integer"
                                    }
                                },
                                "required": [
                                    "web_search_requests"
                                ]
                            }
                        },
                        "required": [
                            "cache_creation_input_tokens",
                            "cache_read_input_tokens",
                            "input_tokens",
                            "output_tokens"
                        ]
                    }
                },
                "required": [
                    "content",
                    "role"
                ]
            },
            "uuid": {
                "type": "string"
            },
            "timestamp": {
                "type": "string"
            },
            "requestId": {
                "type": "string"
            },
            "toolUseResult": {
                "anyOf": [
                    {
                        "type": "string"
                    },
                    {
                        "type": "object",
                        "properties": {
                            "bytes": {
                                "type": "integer"
                            },
                            "code": {
                                "type": "integer"
                            },
                            "codeText": {
                                "type": "string"
                            },
                            "result": {
                                "type": "string"
                            },
                            "durationMs": {
                                "type": "integer"
                            },
                            "url": {
                                "type": "string"
                            },
                            "oldTodos": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "content": {
                                            "type": "string"
                                        },
                                        "status": {
                                            "type": "string"
                                        },
                                        "priority": {
                                            "type": "string"
                                        },
                                        "id": {
                                            "type": "string"
                                        }
                                    },
                                    "required": [
                                        "content",
                                        "id",
                                        "priority",
                                        "status"
                                    ]
                                }
                            },
                            "newTodos": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "content": {
                                            "type": "string"
                                        },
                                        "status": {
                                            "type": "string"
                                        },
                                        "priority": {
                                            "type": "string"
                                        },
                                        "id": {
                                            "type": "string"
                                        }
                                    },
                                    "required": [
                                        "content",
                                        "id",
                                        "priority",
                                        "status"
                                    ]
                                }
                            },
                            "type": {
                                "type": "string"
                            },
                            "file": {
                                "type": "object",
                                "properties": {
                                    "filePath": {
                                        "type": "string"
                                    },
                                    "content": {
                                        "type": "string"
                                    },
                                    "numLines": {
                                        "type": "integer"
                                    },
                                    "startLine": {
                                        "type": "integer"
                                    },
                                    "totalLines": {
                                        "type": "integer"
                                    },
                                    "base64": {
                                        "type": "string"
                                    },
                                    "type": {
                                        "type": "string"
                                    },
                                    "originalSize": {
                                        "type": "integer"
                                    }
                                }
                            },
                            "filenames": {
                                "type": "array",
                                "items": {
                                    "type": "string"
                                }
                            },
                            "numFiles": {
                                "type": "integer"
                            },
                            "truncated": {
                                "type": "boolean"
                            },
                            "content": {
                                "anyOf": [
                                    {
                                        "type": "string"
                                    },
                                    {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "type": {
                                                    "type": "string"
                                                },
                                                "text": {
                                                    "type": "string"
                                                }
                                            },
                                            "required": [
                                                "text",
                                                "type"
                                            ]
                                        }
                                    }
                                ]
                            },
                            "totalDurationMs": {
                                "type": "integer"
                            },
                            "totalTokens": {
                                "type": "integer"
                            },
                            "totalToolUseCount": {
                                "type": "integer"
                            },
                            "usage": {
                                "type": "object",
                                "properties": {
                                    "input_tokens": {
                                        "type": "integer"
                                    },
                                    "cache_creation_input_tokens": {
                                        "type": "integer"
                                    },
                                    "cache_read_input_tokens": {
                                        "type": "integer"
                                    },
                                    "output_tokens": {
                                        "type": "integer"
                                    },
                                    "service_tier": {
                                        "type": [
                                            "null",
                                            "string"
                                        ]
                                    },
                                    "server_tool_use": {
                                        "type": "object",
                                        "properties": {
                                            "web_search_requests": {
                                                "type": "integer"
                                            }
                                        },
                                        "required": [
                                            "web_search_requests"
                                        ]
                                    }
                                },
                                "required": [
                                    "cache_creation_input_tokens",
                                    "cache_read_input_tokens",
                                    "input_tokens",
                                    "output_tokens",
                                    "service_tier"
                                ]
                            },
                            "wasInterrupted": {
                                "type": "boolean"
                            },
                            "stdout": {
                                "type": "string"
                            },
                            "stderr": {
                                "type": "string"
                            },
                            "interrupted": {
                                "type": "boolean"
                            },
                            "isImage": {
                                "type": "boolean"
                            },
                            "filePath": {
                                "type": "string"
                            },
                            "oldString": {
                                "type": "string"
                            },
                            "newString": {
                                "type": "string"
                            },
                            "originalFile": {
                                "type": "string"
                            },
                            "structuredPatch": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "oldStart": {
                                            "type": "integer"
                                        },
                                        "oldLines": {
                                            "type": "integer"
                                        },
                                        "newStart": {
                                            "type": "integer"
                                        },
                                        "newLines": {
                                            "type": "integer"
                                        },
                                        "lines": {
                                            "type": "array",
                                            "items": {
                                                "type": "string"
                                            }
                                        }
                                    },
                                    "required": [
                                        "lines",
                                        "newLines",
                                        "newStart",
                                        "oldLines",
                                        "oldStart"
                                    ]
                                }
                            },
                            "userModified": {
                                "type": "boolean"
                            },
                            "replaceAll": {
                                "type": "boolean"
                            },
                            "returnCodeInterpretation": {
                                "type": "string"
                            },
                            "query": {
                                "type": "string"
                            },
                            "results": {
                                "type": "array",
                                "items": {
                                    "anyOf": [
                                        {
                                            "type": "string"
                                        },
                                        {
                                            "type": "object",
                                            "properties": {
                                                "tool_use_id": {
                                                    "type": "string"
                                                },
                                                "content": {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "object",
                                                        "properties": {
                                                            "title": {
                                                                "type": "string"
                                                            },
                                                            "url": {
                                                                "type": "string"
                                                            }
                                                        },
                                                        "required": [
                                                            "title",
                                                            "url"
                                                        ]
                                                    }
                                                }
                                            },
                                            "required": [
                                                "content",
                                                "tool_use_id"
                                            ]
                                        }
                                    ]
                                }
                            },
                            "durationSeconds": {
                                "type": "number"
                            },
                            "plan": {
                                "type": "string"
                            },
                            "isAgent": {
                                "type": "boolean"
                            },
                            "edits": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "old_string": {
                                            "type": "string"
                                        },
                                        "new_string": {
                                            "type": "string"
                                        },
                                        "replace_all": {
                                            "type": "boolean"
                                        }
                                    },
                                    "required": [
                                        "new_string",
                                        "old_string"
                                    ]
                                }
                            },
                            "originalFileContents": {
                                "type": "string"
                            },
                            "exitPlanModeInput": {
                                "type": "object",
                                "properties": {
                                    "plan": {
                                        "type": "string"
                                    }
                                },
                                "required": [
                                    "plan"
                                ]
                            },
                            "sandbox": {
                                "type": "boolean"
                            }
                        }
                    },
                    {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "content": {
                                    "type": "string"
                                },
                                "status": {
                                    "type": "string"
                                },
                                "priority": {
                                    "type": "string"
                                },
                                "id": {
                                    "type": "string"
                                }
                            },
                            "required": [
                                "content",
                                "id",
                                "priority",
                                "status"
                            ]
                        }
                    }
                ]
            },
            "isApiErrorMessage": {
                "type": "boolean"
            },
            "isMeta": {
                "type": "boolean"
            },
            "summary": {
                "type": "string"
            },
            "leafUuid": {
                "type": "string"
            },
            "isCompactSummary": {
                "type": "boolean"
            },
            "content": {
                "type": "string"
            },
            "level": {
                "type": "string"
            },
            "toolUseID": {
                "type": "string"
            },
            "costUSD": {
                "type": "number"
            },
            "durationMs": {
                "type": "integer"
            }
        },
        "required": [
            "type"
        ]
    },

    "summary_entry":     {
        "$schema": "http://json-schema.org/schema#",
        "type": "object",
        "properties": {
            "type": {
                "type": "string"
            },
            "summary": {
                "type": "string"
            },
            "leafUuid": {
                "type": "string"
            },
            "parentUuid": {
                "type": [
                    "null",
                    "string"
                ]
            },
            "isSidechain": {
                "type": "boolean"
            },
            "userType": {
                "type": "string"
            },
            "cwd": {
                "type": "string"
            },
            "sessionId": {
                "type": "string"
            },
            "version": {
                "type": "string"
            },
            "message": {
                "type": "object",
                "properties": {
                    "role": {
                        "type": "string"
                    },
                    "content": {
                        "anyOf": [
                            {
                                "type": "string"
                            },
                            {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "type": {
                                            "type": "string"
                                        },
                                        "text": {
                                            "type": "string"
                                        },
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "input": {
                                            "type": "object",
                                            "properties": {
                                                "todos": {
                                                    "anyOf": [
                                                        {
                                                            "type": "string"
                                                        },
                                                        {
                                                            "type": "array",
                                                            "items": {
                                                                "type": "object",
                                                                "properties": {
                                                                    "id": {
                                                                        "type": "string"
                                                                    },
                                                                    "content": {
                                                                        "type": "string"
                                                                    },
                                                                    "status": {
                                                                        "type": "string"
                                                                    },
                                                                    "priority": {
                                                                        "type": "string"
                                                                    }
                                                                },
                                                                "required": [
                                                                    "content",
                                                                    "id",
                                                                    "priority",
                                                                    "status"
                                                                ]
                                                            }
                                                        }
                                                    ]
                                                },
                                                "command": {
                                                    "type": "string"
                                                },
                                                "description": {
                                                    "type": "string"
                                                },
                                                "file_path": {
                                                    "type": "string"
                                                },
                                                "old_string": {
                                                    "type": "string"
                                                },
                                                "new_string": {
                                                    "type": "string"
                                                },
                                                "prompt": {
                                                    "type": "string"
                                                },
                                                "offset": {
                                                    "type": "integer"
                                                },
                                                "limit": {
                                                    "type": "integer"
                                                },
                                                "pattern": {
                                                    "type": "string"
                                                },
                                                "path": {
                                                    "type": "string"
                                                },
                                                "plan": {
                                                    "type": "string"
                                                },
                                                "query": {
                                                    "type": "string"
                                                },
                                                "edits": {
                                                    "anyOf": [
                                                        {
                                                            "type": "string"
                                                        },
                                                        {
                                                            "type": "array",
                                                            "items": {
                                                                "type": "object",
                                                                "properties": {
                                                                    "old_string": {
                                                                        "type": "string"
                                                                    },
                                                                    "new_string": {
                                                                        "type": "string"
                                                                    },
                                                                    "replace_all": {
                                                                        "type": "boolean"
                                                                    },
                                                                    "expected_replacements": {
                                                                        "type": "integer"
                                                                    }
                                                                },
                                                                "required": [
                                                                    "new_string",
                                                                    "old_string"
                                                                ]
                                                            }
                                                        }
                                                    ]
                                                },
                                                "replace_all": {
                                                    "type": "boolean"
                                                },
                                                "timeout": {
                                                    "type": "integer"
                                                },
                                                "content": {
                                                    "type": "string"
                                                },
                                                "url": {
                                                    "type": "string"
                                                },
                                                "include": {
                                                    "type": "string"
                                                },
                                                "ignore": {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "string"
                                                    }
                                                },
                                                "input": {
                                                    "type": "string"
                                                }
                                            }
                                        },
                                        "tool_use_id": {
                                            "type": "string"
                                        },
                                        "content": {
                                            "anyOf": [
                                                {
                                                    "type": "string"
                                                },
                                                {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "object",
                                                        "properties": {
                                                            "type": {
                                                                "type": "string"
                                                            },
                                                            "text": {
                                                                "type": "string"
                                                            },
                                                            "source": {
                                                                "type": "object",
                                                                "properties": {
                                                                    "type": {
                                                                        "type": "string"
                                                                    },
                                                                    "data": {
                                                                        "type": "string"
                                                                    },
                                                                    "media_type": {
                                                                        "type": "string"
                                                                    }
                                                                },
                                                                "required": [
                                                                    "data",
                                                                    "media_type",
                                                                    "type"
                                                                ]
                                                            }
                                                        },
                                                        "required": [
                                                            "type"
                                                        ]
                                                    }
                                                }
                                            ]
                                        },
                                        "is_error": {
                                            "type": "boolean"
                                        },
                                        "thinking": {
                                            "type": "string"
                                        },
                                        "signature": {
                                            "type": "string"
                                        }
                                    },
                                    "required": [
                                        "type"
                                    ]
                                }
                            }
                        ]
                    },
                    "id": {
                        "type": "string"
                    },
                    "type": {
                        "type": "string"
                    },
                    "model": {
                        "type": "string"
                    },
                    "stop_reason": {
                        "type": [
                            "null",
                            "string"
                        ]
                    },
                    "stop_sequence": {
                        "type": [
                            "null",
                            "string"
                        ]
                    },
                    "usage": {
                        "type": "object",
                        "properties": {
                            "input_tokens": {
                                "type": "integer"
                            },
                            "cache_creation_input_tokens": {
                                "type": "integer"
                            },
                            "cache_read_input_tokens": {
                                "type": "integer"
                            },
                            "output_tokens": {
                                "type": "integer"
                            },
                            "service_tier": {
                                "type": [
                                    "null",
                                    "string"
                                ]
                            },
                            "server_tool_use": {
                                "type": "object",
                                "properties": {
                                    "web_search_requests": {
                                        "type": "integer"
                                    }
                                },
                                "required": [
                                    "web_search_requests"
                                ]
                            }
                        },
                        "required": [
                            "cache_creation_input_tokens",
                            "cache_read_input_tokens",
                            "input_tokens",
                            "output_tokens"
                        ]
                    }
                },
                "required": [
                    "content",
                    "role"
                ]
            },
            "uuid": {
                "type": "string"
            },
            "timestamp": {
                "type": "string"
            },
            "requestId": {
                "type": "string"
            },
            "toolUseResult": {
                "anyOf": [
                    {
                        "type": "string"
                    },
                    {
                        "type": "object",
                        "properties": {
                            "oldTodos": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "content": {
                                            "type": "string"
                                        },
                                        "status": {
                                            "type": "string"
                                        },
                                        "priority": {
                                            "type": "string"
                                        },
                                        "id": {
                                            "type": "string"
                                        }
                                    },
                                    "required": [
                                        "content",
                                        "id",
                                        "priority",
                                        "status"
                                    ]
                                }
                            },
                            "newTodos": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "content": {
                                            "type": "string"
                                        },
                                        "status": {
                                            "type": "string"
                                        },
                                        "priority": {
                                            "type": "string"
                                        },
                                        "id": {
                                            "type": "string"
                                        }
                                    },
                                    "required": [
                                        "content",
                                        "id",
                                        "priority",
                                        "status"
                                    ]
                                }
                            },
                            "stdout": {
                                "type": "string"
                            },
                            "stderr": {
                                "type": "string"
                            },
                            "interrupted": {
                                "type": "boolean"
                            },
                            "isImage": {
                                "type": "boolean"
                            },
                            "type": {
                                "type": "string"
                            },
                            "file": {
                                "type": "object",
                                "properties": {
                                    "filePath": {
                                        "type": "string"
                                    },
                                    "content": {
                                        "type": "string"
                                    },
                                    "numLines": {
                                        "type": "integer"
                                    },
                                    "startLine": {
                                        "type": "integer"
                                    },
                                    "totalLines": {
                                        "type": "integer"
                                    },
                                    "base64": {
                                        "type": "string"
                                    },
                                    "type": {
                                        "type": "string"
                                    },
                                    "originalSize": {
                                        "type": "integer"
                                    }
                                }
                            },
                            "filePath": {
                                "type": "string"
                            },
                            "oldString": {
                                "type": "string"
                            },
                            "newString": {
                                "type": "string"
                            },
                            "originalFile": {
                                "type": "string"
                            },
                            "structuredPatch": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "oldStart": {
                                            "type": "integer"
                                        },
                                        "oldLines": {
                                            "type": "integer"
                                        },
                                        "newStart": {
                                            "type": "integer"
                                        },
                                        "newLines": {
                                            "type": "integer"
                                        },
                                        "lines": {
                                            "type": "array",
                                            "items": {
                                                "type": "string"
                                            }
                                        }
                                    },
                                    "required": [
                                        "lines",
                                        "newLines",
                                        "newStart",
                                        "oldLines",
                                        "oldStart"
                                    ]
                                }
                            },
                            "userModified": {
                                "type": "boolean"
                            },
                            "replaceAll": {
                                "type": "boolean"
                            },
                            "content": {
                                "anyOf": [
                                    {
                                        "type": "string"
                                    },
                                    {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "type": {
                                                    "type": "string"
                                                },
                                                "text": {
                                                    "type": "string"
                                                }
                                            },
                                            "required": [
                                                "text",
                                                "type"
                                            ]
                                        }
                                    }
                                ]
                            },
                            "totalDurationMs": {
                                "type": "integer"
                            },
                            "totalTokens": {
                                "type": "integer"
                            },
                            "totalToolUseCount": {
                                "type": "integer"
                            },
                            "usage": {
                                "type": "object",
                                "properties": {
                                    "input_tokens": {
                                        "type": "integer"
                                    },
                                    "cache_creation_input_tokens": {
                                        "type": "integer"
                                    },
                                    "cache_read_input_tokens": {
                                        "type": "integer"
                                    },
                                    "output_tokens": {
                                        "type": "integer"
                                    },
                                    "service_tier": {
                                        "type": [
                                            "null",
                                            "string"
                                        ]
                                    },
                                    "server_tool_use": {
                                        "type": "object",
                                        "properties": {
                                            "web_search_requests": {
                                                "type": "integer"
                                            }
                                        },
                                        "required": [
                                            "web_search_requests"
                                        ]
                                    }
                                },
                                "required": [
                                    "cache_creation_input_tokens",
                                    "cache_read_input_tokens",
                                    "input_tokens",
                                    "output_tokens",
                                    "service_tier"
                                ]
                            },
                            "wasInterrupted": {
                                "type": "boolean"
                            },
                            "returnCodeInterpretation": {
                                "type": "string"
                            },
                            "filenames": {
                                "type": "array",
                                "items": {
                                    "type": "string"
                                }
                            },
                            "durationMs": {
                                "type": "integer"
                            },
                            "numFiles": {
                                "type": "integer"
                            },
                            "truncated": {
                                "type": "boolean"
                            },
                            "query": {
                                "type": "string"
                            },
                            "results": {
                                "type": "array",
                                "items": {
                                    "anyOf": [
                                        {
                                            "type": "string"
                                        },
                                        {
                                            "type": "object",
                                            "properties": {
                                                "tool_use_id": {
                                                    "type": "string"
                                                },
                                                "content": {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "object",
                                                        "properties": {
                                                            "title": {
                                                                "type": "string"
                                                            },
                                                            "url": {
                                                                "type": "string"
                                                            }
                                                        },
                                                        "required": [
                                                            "title",
                                                            "url"
                                                        ]
                                                    }
                                                }
                                            },
                                            "required": [
                                                "content",
                                                "tool_use_id"
                                            ]
                                        }
                                    ]
                                }
                            },
                            "durationSeconds": {
                                "type": "number"
                            },
                            "plan": {
                                "type": "string"
                            },
                            "isAgent": {
                                "type": "boolean"
                            },
                            "bytes": {
                                "type": "integer"
                            },
                            "code": {
                                "type": "integer"
                            },
                            "codeText": {
                                "type": "string"
                            },
                            "result": {
                                "type": "string"
                            },
                            "url": {
                                "type": "string"
                            },
                            "edits": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "old_string": {
                                            "type": "string"
                                        },
                                        "new_string": {
                                            "type": "string"
                                        },
                                        "replace_all": {
                                            "type": "boolean"
                                        },
                                        "expected_replacements": {
                                            "type": "integer"
                                        }
                                    },
                                    "required": [
                                        "new_string",
                                        "old_string"
                                    ]
                                }
                            },
                            "originalFileContents": {
                                "type": "string"
                            },
                            "exitPlanModeInput": {
                                "type": "object",
                                "properties": {
                                    "plan": {
                                        "type": "string"
                                    }
                                },
                                "required": [
                                    "plan"
                                ]
                            },
                            "sandbox": {
                                "type": "boolean"
                            }
                        }
                    },
                    {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "content": {
                                    "type": "string"
                                },
                                "status": {
                                    "type": "string"
                                },
                                "priority": {
                                    "type": "string"
                                },
                                "id": {
                                    "type": "string"
                                }
                            },
                            "required": [
                                "content",
                                "id",
                                "priority",
                                "status"
                            ]
                        }
                    }
                ]
            },
            "isApiErrorMessage": {
                "type": "boolean"
            },
            "isMeta": {
                "type": "boolean"
            },
            "isCompactSummary": {
                "type": "boolean"
            },
            "content": {
                "type": "string"
            },
            "level": {
                "type": "string"
            },
            "costUSD": {
                "type": "number"
            },
            "durationMs": {
                "type": "integer"
            }
        },
        "required": [
            "type"
        ]
    },

    "type_8513":     {
        "$schema": "http://json-schema.org/schema#",
        "type": "object",
        "properties": {
            "type": {
                "type": "string"
            },
            "summary": {
                "type": "string"
            },
            "parentUuid": {
                "type": [
                    "null",
                    "string"
                ]
            },
            "isSidechain": {
                "type": "boolean"
            },
            "userType": {
                "type": "string"
            },
            "cwd": {
                "type": "string"
            },
            "sessionId": {
                "type": "string"
            },
            "version": {
                "type": "string"
            },
            "message": {
                "type": "object",
                "properties": {
                    "role": {
                        "type": "string"
                    },
                    "content": {
                        "anyOf": [
                            {
                                "type": "string"
                            },
                            {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "type": {
                                            "type": "string"
                                        },
                                        "text": {
                                            "type": "string"
                                        },
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "input": {
                                            "type": "object",
                                            "properties": {
                                                "command": {
                                                    "type": "string"
                                                },
                                                "description": {
                                                    "type": "string"
                                                },
                                                "todos": {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "object",
                                                        "properties": {
                                                            "id": {
                                                                "type": "string"
                                                            },
                                                            "content": {
                                                                "type": "string"
                                                            },
                                                            "status": {
                                                                "type": "string"
                                                            },
                                                            "priority": {
                                                                "type": "string"
                                                            }
                                                        },
                                                        "required": [
                                                            "content",
                                                            "id",
                                                            "priority",
                                                            "status"
                                                        ]
                                                    }
                                                },
                                                "prompt": {
                                                    "type": "string"
                                                },
                                                "file_path": {
                                                    "type": "string"
                                                },
                                                "limit": {
                                                    "type": "integer"
                                                },
                                                "pattern": {
                                                    "type": "string"
                                                },
                                                "path": {
                                                    "type": "string"
                                                },
                                                "offset": {
                                                    "type": "integer"
                                                },
                                                "old_string": {
                                                    "type": "string"
                                                },
                                                "new_string": {
                                                    "type": "string"
                                                },
                                                "content": {
                                                    "type": "string"
                                                }
                                            }
                                        },
                                        "tool_use_id": {
                                            "type": "string"
                                        },
                                        "content": {
                                            "anyOf": [
                                                {
                                                    "type": "string"
                                                },
                                                {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "object",
                                                        "properties": {
                                                            "type": {
                                                                "type": "string"
                                                            },
                                                            "text": {
                                                                "type": "string"
                                                            }
                                                        },
                                                        "required": [
                                                            "text",
                                                            "type"
                                                        ]
                                                    }
                                                }
                                            ]
                                        },
                                        "is_error": {
                                            "type": "boolean"
                                        },
                                        "thinking": {
                                            "type": "string"
                                        },
                                        "signature": {
                                            "type": "string"
                                        }
                                    },
                                    "required": [
                                        "type"
                                    ]
                                }
                            }
                        ]
                    },
                    "id": {
                        "type": "string"
                    },
                    "type": {
                        "type": "string"
                    },
                    "model": {
                        "type": "string"
                    },
                    "stop_reason": {
                        "type": [
                            "null",
                            "string"
                        ]
                    },
                    "stop_sequence": {
                        "type": [
                            "null",
                            "string"
                        ]
                    },
                    "usage": {
                        "type": "object",
                        "properties": {
                            "input_tokens": {
                                "type": "integer"
                            },
                            "cache_creation_input_tokens": {
                                "type": "integer"
                            },
                            "cache_read_input_tokens": {
                                "type": "integer"
                            },
                            "output_tokens": {
                                "type": "integer"
                            },
                            "service_tier": {
                                "type": "string"
                            },
                            "server_tool_use": {
                                "type": "object",
                                "properties": {
                                    "web_search_requests": {
                                        "type": "integer"
                                    }
                                },
                                "required": [
                                    "web_search_requests"
                                ]
                            }
                        },
                        "required": [
                            "cache_creation_input_tokens",
                            "cache_read_input_tokens",
                            "input_tokens",
                            "output_tokens"
                        ]
                    }
                },
                "required": [
                    "content",
                    "role"
                ]
            },
            "uuid": {
                "type": "string"
            },
            "timestamp": {
                "type": "string"
            },
            "requestId": {
                "type": "string"
            },
            "toolUseResult": {
                "anyOf": [
                    {
                        "type": "string"
                    },
                    {
                        "type": "object",
                        "properties": {
                            "stdout": {
                                "type": "string"
                            },
                            "stderr": {
                                "type": "string"
                            },
                            "interrupted": {
                                "type": "boolean"
                            },
                            "isImage": {
                                "type": "boolean"
                            },
                            "oldTodos": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "content": {
                                            "type": "string"
                                        },
                                        "status": {
                                            "type": "string"
                                        },
                                        "priority": {
                                            "type": "string"
                                        },
                                        "id": {
                                            "type": "string"
                                        }
                                    },
                                    "required": [
                                        "content",
                                        "id",
                                        "priority",
                                        "status"
                                    ]
                                }
                            },
                            "newTodos": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "content": {
                                            "type": "string"
                                        },
                                        "status": {
                                            "type": "string"
                                        },
                                        "priority": {
                                            "type": "string"
                                        },
                                        "id": {
                                            "type": "string"
                                        }
                                    },
                                    "required": [
                                        "content",
                                        "id",
                                        "priority",
                                        "status"
                                    ]
                                }
                            },
                            "content": {
                                "anyOf": [
                                    {
                                        "type": "string"
                                    },
                                    {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "type": {
                                                    "type": "string"
                                                },
                                                "text": {
                                                    "type": "string"
                                                }
                                            },
                                            "required": [
                                                "text",
                                                "type"
                                            ]
                                        }
                                    }
                                ]
                            },
                            "totalDurationMs": {
                                "type": "integer"
                            },
                            "totalTokens": {
                                "type": "integer"
                            },
                            "totalToolUseCount": {
                                "type": "integer"
                            },
                            "usage": {
                                "type": "object",
                                "properties": {
                                    "input_tokens": {
                                        "type": "integer"
                                    },
                                    "cache_creation_input_tokens": {
                                        "type": "integer"
                                    },
                                    "cache_read_input_tokens": {
                                        "type": "integer"
                                    },
                                    "output_tokens": {
                                        "type": "integer"
                                    },
                                    "service_tier": {
                                        "type": "string"
                                    }
                                },
                                "required": [
                                    "cache_creation_input_tokens",
                                    "cache_read_input_tokens",
                                    "input_tokens",
                                    "output_tokens",
                                    "service_tier"
                                ]
                            },
                            "wasInterrupted": {
                                "type": "boolean"
                            },
                            "type": {
                                "type": "string"
                            },
                            "file": {
                                "type": "object",
                                "properties": {
                                    "filePath": {
                                        "type": "string"
                                    },
                                    "content": {
                                        "type": "string"
                                    },
                                    "numLines": {
                                        "type": "integer"
                                    },
                                    "startLine": {
                                        "type": "integer"
                                    },
                                    "totalLines": {
                                        "type": "integer"
                                    }
                                },
                                "required": [
                                    "content",
                                    "filePath",
                                    "numLines",
                                    "startLine",
                                    "totalLines"
                                ]
                            },
                            "filenames": {
                                "type": "array",
                                "items": {
                                    "type": "string"
                                }
                            },
                            "numFiles": {
                                "type": "integer"
                            },
                            "filePath": {
                                "type": "string"
                            },
                            "oldString": {
                                "type": "string"
                            },
                            "newString": {
                                "type": "string"
                            },
                            "originalFile": {
                                "type": "string"
                            },
                            "structuredPatch": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "oldStart": {
                                            "type": "integer"
                                        },
                                        "oldLines": {
                                            "type": "integer"
                                        },
                                        "newStart": {
                                            "type": "integer"
                                        },
                                        "newLines": {
                                            "type": "integer"
                                        },
                                        "lines": {
                                            "type": "array",
                                            "items": {
                                                "type": "string"
                                            }
                                        }
                                    },
                                    "required": [
                                        "lines",
                                        "newLines",
                                        "newStart",
                                        "oldLines",
                                        "oldStart"
                                    ]
                                }
                            },
                            "userModified": {
                                "type": "boolean"
                            },
                            "replaceAll": {
                                "type": "boolean"
                            }
                        }
                    },
                    {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "content": {
                                    "type": "string"
                                },
                                "status": {
                                    "type": "string"
                                },
                                "priority": {
                                    "type": "string"
                                },
                                "id": {
                                    "type": "string"
                                }
                            },
                            "required": [
                                "content",
                                "id",
                                "priority",
                                "status"
                            ]
                        }
                    }
                ]
            }
        },
        "required": [
            "type"
        ]
    },

}

def get_schema(schema_type: str) -> dict:
    """Get schema definition by type"""
    return CLAUDE_SCHEMAS.get(schema_type, {})

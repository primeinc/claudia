# Complete Claude JSONL Schema Documentation

This document contains ALL schemas found in the .claude directory.
Generated using genson to ensure completeness.

## Summary

| Schema Type | Files | Total Lines | Errors |
|-------------|-------|-------------|--------|
| conversation_entry | 70 | 27764 | 0 |
| summary_entry | 114 | 56710 | 1 |
| type_8513 | 1 | 158 | 0 |

## Detailed Schemas

### conversation_entry

**Files:** 70 files
**Lines:** 27764 total lines processed
**Errors:** 0 parsing errors

#### Generated Schema:

```json
{
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
}
```

#### Sample Data:

**Sample 1** from `/home/will/.claude/projects/-home-will-local-dev-portfolio-apps-robin-noguier/82a7f0bc-374e-4520-9d83-68322ee00497.jsonl` line 1:

```json
{
  "parentUuid": null,
  "isSidechain": false,
  "userType": "external",
  "cwd": "/home/will/local_dev/portfolio/apps/robin-noguier",
  "sessionId": "82a7f0bc-374e-4520-9d83-68322ee00497",
  "version": "1.0.31",
  "type": "user",
  "message": {
    "role": "user",
    "content": "msu svg\nYes, Michigan State University (MSU) provides SVG files of their logos for authorized use, such as by faculty and staff, and authorized external vendors. \nHere's how to access them and some important information:\nMSU Brand Studio Downloads: You can find downloadable SVG files of the MSU logos, marks, graphic elements, color palette, and templates on the MSU Brand Studio website. Note that access is generally limited to faculty and staff of MSU. Student employees should ask their supervisor for necessary files, and authorized external vendors should obtain files from their MSU clients.\nAvailability: Various types of MSU logos and marks are available, including the wordmark, helmet, and combined wordmark/helmet logos. The block S and university seal are available for more selective use.\nCollege/Department Specific Logos: Some individual colleges and departments may also provide wordmark packages containing SVG files, such as the College of Arts & Letters.\nUsage Guidelines: MSU has specific guidelines for using their logos and marks. These can be found on the MSU Brand Studio website. Important points include:\nThe wordmark should be displayed in Spartan green, black, or white.\nDo not alter the wordmark except to enlarge or reduce proportionally.\nThe block S should not be used as a substitute for the wordmark or helmet.\nDo not recreate the block S or combine it with other elements.\nOther Sources: You may find some MSU-related SVG files on websites like Etsy, but be aware that using such files for commercial purposes may infringe on trademark rights. Always ensure you have the legal right to use any logos or trademarks. \nIn summary, to obtain official MSU logo SVG files, it
... (truncated)
```

**Sample 2** from `/home/will/.claude/projects/-home-will-local-dev-portfolio-apps-robin-noguier/82a7f0bc-374e-4520-9d83-68322ee00497.jsonl` line 2:

```json
{
  "parentUuid": "d0f4dd9d-18c3-4916-9f21-b734529b2bfd",
  "isSidechain": false,
  "userType": "external",
  "cwd": "/home/will/local_dev/portfolio/apps/robin-noguier",
  "sessionId": "82a7f0bc-374e-4520-9d83-68322ee00497",
  "version": "1.0.31",
  "message": {
    "id": "msg_012NJeQwjS1njV2FzVZyyT4o",
    "type": "message",
    "role": "assistant",
    "model": "claude-opus-4-20250514",
    "content": [
      {
        "type": "text",
        "text": "I cannot download copyrighted branding assets from universities without proper authorization. University logos and branding materials are typically protected intellectual property.\n\nFor official branding assets:\n\n**Michigan State University:**\n- Visit [brand.msu.edu](https://brand.msu.edu) \n- Access restricted to MSU faculty/staff\n- External vendors must request through MSU contacts\n\n**James Madison University:**\n- Visit [www.jmu.edu/identity](https://www.jmu.edu/identity)\n- Download section available for authorized users\n- Brand guidelines and usage restrictions apply\n\nBoth universities require proper authorization to use their branding materials. If you're working on an official project, contact the respective university's marketing/communications department for access."
      }
    ],
    "stop_reason": null,
    "stop_sequence": null,
    "usage": {
      "input_tokens": 4,
      "cache_creation_input_tokens": 15843,
      "cache_read_input_tokens": 0,
      "output_tokens": 2,
      "service_tier": "standard"
    }
  },
  "requestId": "req_011CQRNrWaiBNh5BwSfy3y9g",
  "type": "assistant",
  "uuid": "8ad228eb-cf32-431e-96dd-b3e772b6f7a3",
  "timestamp": "2025-06-23T15:19:28.449Z"
}
```

**Sample 3** from `/home/will/.claude/projects/-home-will-local-dev-portfolio-apps-robin-noguier/82a7f0bc-374e-4520-9d83-68322ee00497.jsonl` line 3:

```json
{
  "parentUuid": "8ad228eb-cf32-431e-96dd-b3e772b6f7a3",
  "isSidechain": false,
  "userType": "external",
  "cwd": "/home/will/local_dev/portfolio/apps/robin-noguier",
  "sessionId": "82a7f0bc-374e-4520-9d83-68322ee00497",
  "version": "1.0.31",
  "type": "user",
  "message": {
    "role": "user",
    "content": "https://brand.msu.edu/Downloads download them for me"
  },
  "uuid": "205de690-d654-4ca9-b9e5-64e21f1d764b",
  "timestamp": "2025-06-23T15:20:13.485Z"
}
```

#### Example Files:

- `/home/will/.claude/projects/-home-will--claude-local/ebf33cee-c830-443d-a104-7759068f574b.jsonl`
- `/home/will/.claude/projects/-home-will-local-dev-claude-gemini-chat/872e4854-9df0-49ec-b904-efaeff7e0fd4.jsonl`
- `/home/will/.claude/projects/-home-will-local-dev-claudia/d739e5b8-8873-49e5-8552-00466fa0da3e.jsonl`
- `/home/will/.claude/projects/-home-will-local-dev-claudia/dd31477f-a451-4e84-9218-abe843b133a1.jsonl`
- `/home/will/.claude/projects/-home-will-local-dev-diosys/6ff36e22-5447-47fc-aafb-ff5c8d1e5fea.jsonl`
- ... and 65 more

---

### summary_entry

**Files:** 114 files
**Lines:** 56710 total lines processed
**Errors:** 1 parsing errors

#### Generated Schema:

```json
{
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
}
```

#### Sample Data:

**Sample 1** from `/home/will/.claude/projects/-home-will/1fb7ac23-9ad9-4c9d-beb5-efa346fe29ac.jsonl` line 1:

```json
{
  "type": "summary",
  "summary": "Fixing ASDF Python Pip Installation Woes",
  "leafUuid": "9b55fce0-9128-41e2-b37c-e400d66f9644"
}
```

**Sample 2** from `/home/will/.claude/projects/-home-will/ebc748e1-daf7-44f7-bd4b-fe0bbdf52980.jsonl` line 1:

```json
{
  "type": "summary",
  "summary": "Fixing asdf Python pip installation in WSL",
  "leafUuid": "4fae229d-7941-4544-a744-a238f6151ca1"
}
```

**Sample 3** from `/home/will/.claude/projects/-home-will/ebc748e1-daf7-44f7-bd4b-fe0bbdf52980.jsonl` line 2:

```json
{
  "parentUuid": null,
  "isSidechain": false,
  "userType": "external",
  "cwd": "/home/will",
  "sessionId": "ebc748e1-daf7-44f7-bd4b-fe0bbdf52980",
  "version": "1.0.35",
  "type": "user",
  "message": {
    "role": "user",
    "content": [
      {
        "type": "text",
        "text": "\n\u25cf Let me install it:\n\n\u25cf Bash(pip install --user git-filter-repo)\n  \u23bf \u00a0Error: /home/will/.asdf/plugins/python/shims/pip: line 57: $'asdf-python couldn\\'t\n     find a suitable pip executable\\nThis is probably a problem with the plugin, please\n     report this issue': command not found\n\n\u25cf Let me try another approach:\n\n\u25cf Bash(curl -o ~/.local/bin/git-filter-repo\n      https://raw.githubusercontent.com/newren/git-filter-repo/main/git-filter-repo &&\n      chmod +x ~/.local/bin/git-filter-repo)\n  \u23bf \u00a0No (tell Claude what to do differently)\ncan you fix my asdf install"
      }
    ]
  },
  "uuid": "ff53cd45-0d7d-4563-9bdd-dbfaaaa1bd9a",
  "timestamp": "2025-06-27T09:09:29.321Z"
}
```

#### Example Files:

- `/home/will/.claude/projects/-home-will-local-dev-diosys/487e6fcd-05c9-4180-8bae-9b886a0c00c6.jsonl`
- `/home/will/.claude/projects/-home-will-local-dev-portfolio-apps-rylee-brasseur/e2050d7c-7049-476e-8d64-6a4beec65744.jsonl`
- `/home/will/.claude/projects/-home-will-local-dev-portfolio/1f7591c7-bc1f-4c1d-b0fe-07558912d299.jsonl`
- `/home/will/.claude/projects/-home-will-local-dev-portfolio/2988ccad-a227-45fa-b2f9-1bc58cd483ec.jsonl`
- `/home/will/.claude/projects/-home-will-local-dev-portfolio/2a2eb166-2033-4075-aabb-7a9d5bcab88f.jsonl`
- ... and 109 more

---

### type_8513

**Files:** 1 files
**Lines:** 158 total lines processed
**Errors:** 0 parsing errors

#### Generated Schema:

```json
{
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
}
```

#### Sample Data:

**Sample 1** from `/home/will/.claude/projects/-home-will-local-dev-qualCap/b0a68c65-8fbb-4b65-b0f5-ef5e4a6449e7.jsonl` line 1:

```json
{
  "type": "summary",
  "summary": "Qualia DDP API: Convoluted Auth, Data Retrieval Challenge"
}
```

**Sample 2** from `/home/will/.claude/projects/-home-will-local-dev-qualCap/b0a68c65-8fbb-4b65-b0f5-ef5e4a6449e7.jsonl` line 2:

```json
{
  "parentUuid": null,
  "isSidechain": false,
  "userType": "external",
  "cwd": "/home/will/local_dev/qualCap",
  "sessionId": "b0a68c65-8fbb-4b65-b0f5-ef5e4a6449e7",
  "version": "1.0.24",
  "type": "user",
  "message": {
    "role": "user",
    "content": [
      {
        "type": "text",
        "text": "<command-message>user:pr-pull is running\u2026</command-message>\n                      <command-name>user:pr-pull</command-name>\n                      <command-args></command-args>\n                          <command-contents>{\n  \"type\": \"text\",\n  \"text\": \"# GitHub PR Review Extraction & Thread Resolution Toolkit\\n\\nThis command automatically detects your repository and PR context, then provides tools to:\\n- Fetch all PR reviews and comments\\n- Create organized todos with thread tracking\\n- Resolve/unresolve review threads\\n- Add replies to ongoing discussions\\n\\n### \ud83d\udd27 REQUIREMENTS:\\n- GitHub CLI (`gh`) authenticated\\n- `jq` installed\\n- Must be run from within a git repository\\n\\n### \ud83d\udcdd Usage:\\n- Run from a PR branch: Auto-detects current PR\\n- Run from any branch: Shows list of PRs to choose from\\n- Provide PR number: `/user:pr-pull 123` to target specific PR\\n\\n---\\n\\n## \ud83d\ude80 Quick Start - Auto-Detection\\n\\nFirst, let me detect your repository and PR context:\\n\\n```bash\\n# Automatically detect repo from git remote\\nREPO_URL=$(git remote get-url origin 2>/dev/null)\\nif [ -z \\\"$REPO_URL\\\" ]; then\\n  echo \\\"Error: Not in a git repository or no origin remote found\\\"\\n  exit 1\\nfi\\n\\n# Extract owner/repo from various GitHub URL formats\\nREPO_PATH=$(echo \\\"$REPO_URL\\\" | sed -E 's|.*github.com[:/](.*)\\\\.git|\\\\1|' | sed 's|\\\\.git$||')\\nREPO_OWNER=$(echo \\\"$REPO_PATH\\\" | cut -d'/' -f1)\\nREPO_NAME=$(echo \\\"$REPO_PATH\\\" | cut -d'/' -f2)\\n\\n# Try to get current PR number\\nPR_NUMBER=$(gh pr view --json number -q .number 2>/dev/null || echo \\\"\\\")\\n\\n# If no PR fo
... (truncated)
```

**Sample 3** from `/home/will/.claude/projects/-home-will-local-dev-qualCap/b0a68c65-8fbb-4b65-b0f5-ef5e4a6449e7.jsonl` line 3:

```json
{
  "parentUuid": "c810c777-708e-4de0-8d51-6f925e5ccc81",
  "isSidechain": false,
  "userType": "external",
  "cwd": "/home/will/local_dev/qualCap",
  "sessionId": "b0a68c65-8fbb-4b65-b0f5-ef5e4a6449e7",
  "version": "1.0.24",
  "type": "assistant",
  "timestamp": "2025-06-15T13:55:45.602Z",
  "message": {
    "id": "msg_01DxDRHx8QWYVLcauh654a4R",
    "type": "message",
    "role": "assistant",
    "model": "claude-opus-4-20250514",
    "content": [
      {
        "type": "text",
        "text": "I'll help you extract and organize the PR review feedback. Let me start by detecting your repository and PR context."
      }
    ],
    "stop_reason": null,
    "stop_sequence": null,
    "usage": {
      "input_tokens": 4,
      "cache_creation_input_tokens": 4676,
      "cache_read_input_tokens": 13167,
      "output_tokens": 5,
      "service_tier": "standard"
    }
  },
  "requestId": "req_011CQA4iNZPBH8S9BoxJp2hM",
  "uuid": "3184f264-f4a5-4077-b820-e31f2cda1d31"
}
```

#### Example Files:

- `/home/will/.claude/projects/-home-will-local-dev-qualCap/b0a68c65-8fbb-4b65-b0f5-ef5e4a6449e7.jsonl`

---


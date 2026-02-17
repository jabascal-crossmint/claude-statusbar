#!/bin/bash

# Test script for claude-statusbar

echo "=== Test 1: Basic directory (no git) ==="
echo '{"workspace":{"current_dir":"/tmp"}}' | ./target/release/claude-statusbar
echo -e "\n"

echo "=== Test 2: Git repo, no session ==="
echo '{"workspace":{"current_dir":"'$(pwd)'"}}' | ./target/release/claude-statusbar
echo -e "\n"

echo "=== Test 3: With low context usage (gray) ==="
cat > /tmp/test-low.jsonl << 'EOF'
{"timestamp": 1708186800, "message": {"role": "user", "content": "Test message"}}
{"timestamp": 1708187000, "message": {"role": "assistant", "content": "Response", "usage": {"input_tokens": 5000, "output_tokens": 2000, "cache_read_input_tokens": 3000, "cache_creation_input_tokens": 1000}}}
EOF
echo '{"workspace":{"current_dir":"'$(pwd)'"},"model":{"display_name":"Claude Sonnet 4.5"},"session_id":"low-usage","transcript_path":"/tmp/test-low.jsonl"}' | ./target/release/claude-statusbar
echo -e "\n"

echo "=== Test 4: With medium context usage (yellow) ==="
cat > /tmp/test-medium.jsonl << 'EOF'
{"timestamp": 1708186800, "message": {"role": "user", "content": "Test message"}}
{"timestamp": 1708187000, "message": {"role": "assistant", "content": "Response", "usage": {"input_tokens": 40000, "output_tokens": 20000, "cache_read_input_tokens": 20000, "cache_creation_input_tokens": 5000}}}
EOF
echo '{"workspace":{"current_dir":"'$(pwd)'"},"model":{"display_name":"Claude Sonnet 4.5"},"session_id":"med-usage","transcript_path":"/tmp/test-medium.jsonl"}' | ./target/release/claude-statusbar
echo -e "\n"

echo "=== Test 5: With high context usage (orange) ==="
cat > /tmp/test-high.jsonl << 'EOF'
{"timestamp": 1708186800, "message": {"role": "user", "content": "Test message"}}
{"timestamp": 1708187000, "message": {"role": "assistant", "content": "Response", "usage": {"input_tokens": 50000, "output_tokens": 30000, "cache_read_input_tokens": 30000, "cache_creation_input_tokens": 5000}}}
EOF
echo '{"workspace":{"current_dir":"'$(pwd)'"},"model":{"display_name":"Claude Opus 4.6"},"session_id":"high-usage","transcript_path":"/tmp/test-high.jsonl"}' | ./target/release/claude-statusbar
echo -e "\n"

echo "=== Test 6: With critical context usage (red) ==="
cat > /tmp/test-critical.jsonl << 'EOF'
{"timestamp": 1708186800, "message": {"role": "user", "content": "Test message"}}
{"timestamp": 1708197000, "message": {"role": "assistant", "content": "Response", "usage": {"input_tokens": 60000, "output_tokens": 40000, "cache_read_input_tokens": 40000, "cache_creation_input_tokens": 5000}}}
EOF
echo '{"workspace":{"current_dir":"'$(pwd)'"},"model":{"display_name":"Claude Haiku 4.5"},"session_id":"critical","transcript_path":"/tmp/test-critical.jsonl"}' | ./target/release/claude-statusbar
echo -e "\n"

echo "=== Test 7: With --short flag ==="
echo '{"workspace":{"current_dir":"'$(pwd)'"},"model":{"display_name":"Claude Sonnet 4.5"},"session_id":"short-test","transcript_path":"/tmp/test-low.jsonl"}' | ./target/release/claude-statusbar --short
echo -e "\n"

echo "=== Test 8: Empty workspace (should show ~) ==="
echo '{}' | ./target/release/claude-statusbar
echo -e "\n"

echo "All tests completed!"

#!/bin/bash

# Example: Search for tasks with the "update-auto" tag across all lists in a space
# 
# This script demonstrates how to use the clickup-cli to interactively search
# for all tasks that are marked with the "update-auto" tag across all lists
# in a selected space.
#
# The script will:
# 1. Display available workspaces and prompt for selection
# 2. Display available spaces in the selected workspace and prompt for selection
# 3. Search through all lists in the selected space for tasks with the specified tag
# 4. Display all matching tasks in a formatted table
#
# Prerequisites:
# 1. Install the CLI: cargo install --path .
# 2. Set up authentication: clickup-cli auth set

echo "🔍 Interactive Search for Tasks with 'update-auto' Tag"
echo "=================================================="
echo ""

echo "This will search through ALL lists in a selected space for tasks with the 'update-auto' tag."
echo "You'll be prompted to select a workspace and space if not already specified."
echo ""

# Search for tasks with the update-auto tag
clickup-cli tasks search-by-tag --tag "update-auto"

echo ""
echo "✅ Search completed!"
echo ""
echo "💡 Tips:"
echo "   - You can specify workspace and space IDs to skip the prompts:"
echo "     clickup-cli tasks search-by-tag --tag 'update-auto' --workspace-id 'workspace_123' --space-id 'space_456'"
echo "   - This searches through ALL lists in the selected space"
echo "   - Use this when you don't know which specific list contains your tasks"
echo "   - For faster results on large spaces, use the list-specific command instead" 
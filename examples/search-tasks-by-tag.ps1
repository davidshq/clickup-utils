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

Write-Host "🔍 Interactive Search for Tasks with 'update-auto' Tag" -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Green
Write-Host ""

Write-Host "This will search through ALL lists in a selected space for tasks with the 'update-auto' tag." -ForegroundColor Yellow
Write-Host "You'll be prompted to select a workspace and space if not already specified." -ForegroundColor Yellow
Write-Host ""

# Search for tasks with the update-auto tag
clickup-cli tasks search-by-tag --tag "update-auto"

Write-Host ""
Write-Host "✅ Search completed!" -ForegroundColor Green
Write-Host ""
Write-Host "💡 Tips:" -ForegroundColor Cyan
Write-Host "   - You can specify workspace and space IDs to skip the prompts:" -ForegroundColor White
Write-Host "     clickup-cli tasks search-by-tag --tag 'update-auto' --workspace-id 'workspace_123' --space-id 'space_456'" -ForegroundColor Gray
Write-Host "   - This searches through ALL lists in the selected space" -ForegroundColor White
Write-Host "   - Use this when you don't know which specific list contains your tasks" -ForegroundColor White
Write-Host "   - For faster results on large spaces, use the list-specific command instead" -ForegroundColor White 
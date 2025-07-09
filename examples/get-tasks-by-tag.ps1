# Example: Get all tasks with the "update-auto" tag
# 
# This script demonstrates how to use the clickup-cli to find all tasks
# that are marked with the "update-auto" tag in a specific list.
#
# Note: This uses client-side filtering - all tasks are fetched from the list
# and then filtered locally to show only those with the specified tag.
#
# Prerequisites:
# 1. Install the CLI: cargo install --path .
# 2. Set up authentication: clickup-cli auth set
# 3. Know your list ID (you can find this by listing workspaces, spaces, and lists)

Write-Host "🔍 Finding tasks with 'update-auto' tag..." -ForegroundColor Green

# Replace "your-list-id" with your actual list ID
$LIST_ID = "your-list-id"

# Get tasks with the update-auto tag
clickup-cli tasks list-by-tag `
  --list-id $LIST_ID `
  --tag "update-auto"

Write-Host ""
Write-Host "✅ Done! All tasks with 'update-auto' tag have been listed above." -ForegroundColor Green
Write-Host ""
Write-Host "💡 Tips:" -ForegroundColor Yellow
Write-Host "   - Use 'clickup-cli workspaces list' to find your workspace ID"
Write-Host "   - Use 'clickup-cli spaces list --workspace-id <workspace-id>' to find your space ID"
Write-Host "   - Use 'clickup-cli lists list --space-id <space-id>' to find your list ID"
Write-Host "   - You can also filter by other tags by changing the --tag parameter" 
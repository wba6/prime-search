#!/bin/bash

# Script Name: fix_markdown.sh
# Description: Fixes invalid Markdown formatting by ensuring proper spacing around headings.
# Usage: ./fix_markdown.sh [-b] file1.md [file2.md ...]
# Options:
#   -b    Create a backup of the original file with a .bak extension.

# Function to display usage instructions
usage() {
    echo "Usage: $0 [-b] file1.md [file2.md ...]"
    echo "  -b    Create a backup of the original file with a .bak extension."
    exit 1
}

# Initialize backup flag
BACKUP=0

# Parse options
while getopts ":b" opt; do
  case ${opt} in
    b )
      BACKUP=1
      ;;
    \? )
      echo "Invalid Option: -$OPTARG" 1>&2
      usage
      ;;
  esac
done
shift $((OPTIND -1))

# Check if at least one file is provided
if [ "$#" -lt 1 ]; then
    usage
fi

# Loop through all provided markdown files
for file in "$@"; do
    # Check if file exists
    if [ ! -f "$file" ]; then
        echo "Error: File not found - $file" 1>&2
        continue
    fi

    # If backup flag is set, create a backup
    if [ $BACKUP -eq 1 ]; then
        cp "$file" "${file}.bak"
        echo "Backup created: ${file}.bak"
    fi

    # Create a temporary file for processing
    tmpfile=$(mktemp)

    # Use awk to process the file
    awk '
    BEGIN {
        in_yaml = 0      # Flag to indicate if inside YAML front matter
        need_blank_after = 0  # Flag to indicate if a blank line is needed after a heading
        prev_blank = 1    # Flag to track if the previous line was blank
    }
    /^---$/ {
        if (in_yaml == 0) {
            in_yaml = 1
        } else {
            in_yaml = 0
        }
        print $0
        next
    }
    {
        if (in_yaml) {
            print $0
            next
        }

        if ($0 ~ /^#{1,6} /) {  # Match headings (from # to ######)
            if (prev_blank == 0) {
                print ""  # Insert a blank line before the heading
            }
            print $0
            need_blank_after = 1  # Set flag to insert blank line after heading
            prev_blank = 0
            next
        }

        if (need_blank_after) {
            if ($0 ~ /^$/) {
                # Next line is already a blank line
                need_blank_after = 0
            } else if ($0 ~ /^#{1,6} /) {
                # Next line is another heading; no need to add a blank line
                need_blank_after = 0
            } else {
                print ""  # Insert a blank line after the heading
                need_blank_after = 0
            }
        }

        print $0
        prev_blank = ($0 ~ /^$/) ? 1 : 0  # Update prev_blank flag
    }
    ' "$file" > "$tmpfile"

    # Replace the original file with the processed temporary file
    mv "$tmpfile" "$file"

    echo "Processed: $file"
done

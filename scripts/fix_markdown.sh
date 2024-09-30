#!/bin/bash

# Script Name: fix_markdown.sh
# Description: Fixes invalid Markdown formatting by ensuring proper spacing, list indentation, and removing trailing whitespaces.
# Usage: ./fix_markdown.sh [-b] file1.md [file2.md ...]

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

    # Initialize YAML front matter flags
    in_yaml=0

    # Process the file line by line
    while IFS= read -r line; do
        # Check for YAML front matter start/end
        if [[ "$line" == "---" ]]; then
            if [[ $in_yaml -eq 0 ]]; then
                in_yaml=1
            else
                in_yaml=0
            fi
            echo "$line" >> "$tmpfile"
            continue
        fi

        # If inside YAML, do not process
        if [[ $in_yaml -eq 1 ]]; then
            echo "$line" >> "$tmpfile"
            continue
        fi

        # Handle headings: ensure blank lines before and after
        if [[ "$line" =~ ^#{1,6}\  ]]; then
            # Add a blank line before heading if not already present
            if [[ $(tail -n 1 "$tmpfile") != "" ]]; then
                echo "" >> "$tmpfile"
            fi
            echo "$line" >> "$tmpfile"
            # Set a flag to add a blank line after heading
            add_blank_after=1
            continue
        fi

        if [[ "$add_blank_after" -eq 1 ]]; then
            if [[ "$line" != "" && ! "$line" =~ ^#{1,6}\  ]]; then
                echo "" >> "$tmpfile"
            fi
            add_blank_after=0
        fi

        # Fix unordered list indentation to two spaces
        if [[ "$line" =~ ^[*+-]\  ]]; then
            # Replace leading symbols with two spaces and a dash
            fixed_line=$(echo "$line" | sed -E 's/^([*+-])\ +/  - /')
            echo "$fixed_line" >> "$tmpfile"
            continue
        fi

        # Remove trailing whitespaces
        fixed_line=$(echo "$line" | sed -E 's/[ \t]+$//')
        echo "$fixed_line" >> "$tmpfile"
    done < "$file"

    # Further processing: ensure single blank lines between paragraphs
    # Remove multiple consecutive blank lines
    sed -i '/^$/N;/^\n$/D' "$tmpfile"

    # Replace the original file with the processed temporary file
    mv "$tmpfile" "$file"

    echo "Processed: $file"
done

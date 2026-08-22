#!/bin/sh -e

echo "⚪️⚪️⚪️⚪️⚪️⚪️⚪️⚪️⚪️⚪️⚪️⚪️⚪️⚪️"

echo "Updating assets volume with latest from lila-assets image..."
echo "This is how the static files are updated on the lila-server container."
echo "This script is necessary because lila reads the manifest from disk and needs the files."

rm -rf /assets/*
cp -r /usr/share/nginx/html/public/* /assets/

echo "Done!"

echo "🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢"

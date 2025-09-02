#!/bin/bash
# Deploy visitor book to production

set -e

echo "🚀 Deploying MMOGIT Visitor Book..."

# Build API
echo "📦 Building API..."
cd api
cargo build --release
cd ..

# Create deployment package
echo "📦 Creating deployment package..."
tar -czf visitor-book-deploy.tar.gz \
    api/target/release/visitor-book-api \
    api/visitor-book.service \
    api/nginx-visitor-book.conf

# Deploy to server
echo "📤 Deploying to server..."
scp visitor-book-deploy.tar.gz root@91.98.123.26:/tmp/

echo "🔧 Installing on server..."
ssh root@91.98.123.26 << 'EOF'
cd /tmp
tar -xzf visitor-book-deploy.tar.gz
mv api/target/release/visitor-book-api /root/visitor-book-api-new
systemctl stop visitor-book || true
mv /root/visitor-book-api-new /root/visitor-book-api
systemctl restart visitor-book
systemctl status visitor-book --no-pager
rm -rf /tmp/visitor-book-deploy.tar.gz /tmp/api
EOF

# Clean up
rm visitor-book-deploy.tar.gz

echo "✅ API deployed and running!"

# Deploy web assets to GitHub Pages
echo "📦 Deploying web assets to GitHub Pages..."
echo "Run: git push origin main"
echo "GitHub Pages will automatically update from the web-visitor-book folder"

echo "✅ Deployment complete!"
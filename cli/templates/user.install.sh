set -euox pipefail

if getent passwd {{username}} >/dev/null 2>&1; then
    echo "User '{{username}}' already exists, skipping..."
else
    echo "Creating user '{{username}}'..."
    sudo adduser {{username}}
    sudo usermod -aG sudo {{username}}
    echo "{{username}} ALL=(ALL) NOPASSWD: ALL" | sudo tee /etc/sudoers.d/{{username}}
fi

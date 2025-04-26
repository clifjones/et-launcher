FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

# Install Tauri dependencies
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    dbus-x11 \
    libcanberra-gtk3-module \
    && rm -rf /var/lib/apt/lists/*

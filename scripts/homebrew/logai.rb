class Logai < Formula
  desc "AI-powered log analyzer with MCP integration - Groups errors, suggests fixes, and connects external tools"
  homepage "https://github.com/ranjan-mohanty/logai"
  version "0.2.0"
  license "MIT"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.2.0/logai-macos-aarch64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.2.0/logai-macos-x86_64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.2.0/logai-linux-aarch64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.2.0/logai-linux-x86_64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  def install
    bin.install "logai"
  end

  test do
    system "#{bin}/logai", "--version"
  end
end

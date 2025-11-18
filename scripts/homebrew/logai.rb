class Logai < Formula
  desc "AI-powered log analyzer with MCP integration"
  homepage "https://github.com/ranjan-mohanty/logai"
  version "0.1.0-beta.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.1.0-beta.1/logai-macos-aarch64.tar.gz"
      sha256 "PLACEHOLDER_MACOS_ARM64"
    else
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.1.0-beta.1/logai-macos-x86_64.tar.gz"
      sha256 "PLACEHOLDER_MACOS_X86_64"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.1.0-beta.1/logai-linux-aarch64.tar.gz"
      sha256 "PLACEHOLDER_LINUX_ARM64"
    else
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.1.0-beta.1/logai-linux-x86_64.tar.gz"
      sha256 "PLACEHOLDER_LINUX_X86_64"
    end
  end

  def install
    bin.install "logai"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/logai --version")
  end
end

class Logai < Formula
  desc "AI-powered log analyzer with MCP integration"
  homepage "https://github.com/ranjan-mohanty/logai"
  version "0.1.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.1.1/logai-macos-aarch64.tar.gz"
      sha256 "f6e9a0c44bb2584ff8ca88934e8a9fb5b1ccec4699721493f417224888fbdd93"
    else
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.1.1/logai-macos-x86_64.tar.gz"
      sha256 "5f2661de15a265ea6df320952da3031551d7076e1802f4fb9e804999feaf6403"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.1.1/logai-linux-aarch64.tar.gz"
      sha256 "bd22882bf98ea1622a9049b6f76029eec00cdb938ae1cb4b7382b71acd291558"
    else
      url "https://github.com/ranjan-mohanty/logai/releases/download/v0.1.1/logai-linux-x86_64.tar.gz"
      sha256 "fb9fb611bdd0440e0283d1985133c8e11ec01d5ef090a227bd3e0633a43c8982"
    end
  end

  def install
    bin.install "logai"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/logai --version")
  end
end

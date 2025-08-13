class Vsix < Formula
  desc "Download and install .vsix extensions into Visual Studio Code and Cursor"
  homepage "https://github.com/beeltec/vsix"
  license "MIT"
  version "1.0.0"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/beeltec/vsix/releases/download/v1.0.0/vsix-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM64"
    else
      url "https://github.com/beeltec/vsix/releases/download/v1.0.0/vsix-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_64"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/beeltec/vsix/releases/download/v1.0.0/vsix-aarch64-linux.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_ARM"
    else
      url "https://github.com/beeltec/vsix/releases/download/v1.0.0/vsix-x86_64-linux.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_X86"
    end
  end

  def install
    bin.install "vsix"
  end

  test do
    assert_match "vsix #{version}", shell_output("#{bin}/vsix --version")
  end
end
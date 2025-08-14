class Vsix < Formula
  desc "Download and install .vsix extensions into Visual Studio Code and Cursor"
  homepage "https://github.com/beeltec/vsix"
  license "MIT"
  version "1.0.2"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/beeltec/vsix/releases/download/v1.0.2/vsix-aarch64-apple-darwin.tar.gz"
      sha256 "5c93c62272f30d6bf3ec670c34b43308dfef71db4b20843f7ddec46f846f1a46"
    else
      url "https://github.com/beeltec/vsix/releases/download/v1.0.2/vsix-x86_64-apple-darwin.tar.gz"
      sha256 "2d9c76f5b2a19af001937ae071a9d904cfeef9bbd56afd5249687be796d4cc57"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/beeltec/vsix/releases/download/v1.0.2/vsix-aarch64-linux.tar.gz"
      sha256 "9bcef7247839c54d5e547b2483503b1e5ef52f9989c6d930eb8dd54f45a7a292_ARM"
    else
      url "https://github.com/beeltec/vsix/releases/download/v1.0.2/vsix-x86_64-linux.tar.gz"
      sha256 "2d5b654dc4d0863bfa72a87def5f05e30594122e1780e576bfb38705cd77a87b"
    end
  end

  def install
    bin.install "vsix"
  end

  test do
    assert_match "vsix #{version}", shell_output("#{bin}/vsix --version")
  end
end
class Vsix < Formula
  desc "Download and install .vsix extensions into Visual Studio Code and Cursor"
  homepage "https://github.com/beeltec/vsix"
  license "MIT"
  version "1.0.0"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/beeltec/vsix/releases/download/v1.0.0/vsix-aarch64-apple-darwin.tar.gz"
      sha256 "69088c67cdf48c49772b85d31032cf7d784aa293d6f0231e63eb83e33776c0ba"
    else
      url "https://github.com/beeltec/vsix/releases/download/v1.0.0/vsix-x86_64-apple-darwin.tar.gz"
      sha256 "5a0620836a6d86b5e362c0db5a90b6d9e923409ec8ce45e81a377ac1b08d3cf4"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/beeltec/vsix/releases/download/v1.0.0/vsix-aarch64-linux.tar.gz"
      sha256 "9bcef7247839c54d5e547b2483503b1e5ef52f9989c6d930eb8dd54f45a7a292_ARM"
    else
      url "https://github.com/beeltec/vsix/releases/download/v1.0.0/vsix-x86_64-linux.tar.gz"
      sha256 "9bcef7247839c54d5e547b2483503b1e5ef52f9989c6d930eb8dd54f45a7a292_X86"
    end
  end

  def install
    bin.install "vsix"
  end

  test do
    assert_match "vsix #{version}", shell_output("#{bin}/vsix --version")
  end
end
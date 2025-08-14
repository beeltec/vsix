class Vsix < Formula
  desc "Download and install .vsix extensions into Visual Studio Code and Cursor"
  homepage "https://github.com/beeltec/vsix"
  license "MIT"
  version "1.0.1"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/beeltec/vsix/releases/download/v1.0.1/vsix-aarch64-apple-darwin.tar.gz"
      sha256 "6e07ddc7aac7c1e7d12ba86f04fcd024a13087d3bb4173287a47c3a5c969ef92"
    else
      url "https://github.com/beeltec/vsix/releases/download/v1.0.1/vsix-x86_64-apple-darwin.tar.gz"
      sha256 "d9e7251e62843032e197076b5f787607d4488e633b7460e7521429b66232df60"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/beeltec/vsix/releases/download/v1.0.1/vsix-aarch64-linux.tar.gz"
      sha256 "9bcef7247839c54d5e547b2483503b1e5ef52f9989c6d930eb8dd54f45a7a292_ARM"
    else
      url "https://github.com/beeltec/vsix/releases/download/v1.0.1/vsix-x86_64-linux.tar.gz"
      sha256 "6e8108518b4132c65c74babfd72742fb1ebebd383afd2281407a73d2229a3ee2"
    end
  end

  def install
    bin.install "vsix"
  end

  test do
    assert_match "vsix #{version}", shell_output("#{bin}/vsix --version")
  end
end
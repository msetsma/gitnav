class Gitnav < Formula
  desc "Fast git repository navigator with fuzzy finding"
  homepage "https://github.com/msetsma/gitnav"
  version "0.1.0"
  license "MIT OR Apache-2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_ARM64"
    else
      url "https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_X86_64"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_ARM64_LINUX"
    else
      url "https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_X86_64_LINUX"
    end
  end

  depends_on "fzf"

  def install
    bin.install "gitnav"
  end

  def caveats
    <<~EOS
      To enable shell integration, add one of the following to your shell config:

      For zsh (~/.zshrc):
        eval "$(gitnav --init zsh)"

      For bash (~/.bashrc):
        eval "$(gitnav --init bash)"

      For fish (~/.config/fish/config.fish):
        gitnav --init fish | source

      Then use the 'gn' command to navigate repositories.
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/gitnav --version")
  end
end

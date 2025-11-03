class Gitnav < Formula
  desc "Fast git repository navigator with fuzzy finding"
  homepage "https://github.com/msetsma/gitnav"
  version "VERSION_PLACEHOLDER"
  license "MIT OR Apache-2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/msetsma/gitnav/releases/download/#{version}/gitnav-aarch64-apple-darwin.tar.gz"
      sha256 "MACOS_ARM_SHA_PLACEHOLDER"
    else
      url "https://github.com/msetsma/gitnav/releases/download/#{version}/gitnav-x86_64-apple-darwin.tar.gz"
      sha256 "MACOS_X86_SHA_PLACEHOLDER"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/msetsma/gitnav/releases/download/#{version}/gitnav-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "LINUX_ARM_SHA_PLACEHOLDER"
    else
      url "https://github.com/msetsma/gitnav/releases/download/#{version}/gitnav-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "LINUX_GNU_SHA_PLACEHOLDER"
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

      For nushell (~/.config/nushell/config.nu):
        gitnav --init nu | save --force ~/.cache/gitnav/init.nu
        source ~/.cache/gitnav/init.nu

      Then use the 'gn' command to navigate repositories.
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/gitnav --version")
  end
end

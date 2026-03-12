class ClawMaster < Formula
  desc "Personal AI gateway inspired by OpenClaw"
  homepage "https://www.clawmaster.org/"
  license "MIT"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/clawmaster-org/clawmaster/releases/download/v#{version}/clawmaster-#{version}-aarch64-apple-darwin.tar.gz"
    else
      url "https://github.com/clawmaster-org/clawmaster/releases/download/v#{version}/clawmaster-#{version}-x86_64-apple-darwin.tar.gz"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/clawmaster-org/clawmaster/releases/download/v#{version}/clawmaster-#{version}-aarch64-unknown-linux-gnu.tar.gz"
    else
      url "https://github.com/clawmaster-org/clawmaster/releases/download/v#{version}/clawmaster-#{version}-x86_64-unknown-linux-gnu.tar.gz"
    end
  end

  def install
    bin.install "clawmaster"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/clawmaster --version")
  end
end

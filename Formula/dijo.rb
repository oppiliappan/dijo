class dijo < Formula
  desc "Terminal Habit Tracker Written in Rust"
  homepage "https://github.com/nerdypepper/dijo"
  url  "https://github.com/slano-ls/homebrew-dijo/releases/latest/download/dijo-mac.tar.gz"
  sha256 "47c5b5678c58a280e7e424ce076faef488ce2c0ba29d01705847da418298ef0d"
  version "0.1.0"

  def install
    bin.install "dijo"
  end
end

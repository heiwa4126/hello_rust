# hello_rust

Rust + GitHub Actionsの練習。
goreleaserのときみたいに、git tag & push --tagでGitHubのreleasesにビルドされたものがあがるようにする。


# 参考

- [RustのLinux/Windows/macOS向け64bitバイナリをGitHub Actionsで生成する - Qiita](https://qiita.com/dalance/items/66d97c252b8dd9c96c29)
- [dalance/procs: A modern replacement for ps written in Rust](https://github.com/dalance/procs)
- [softprops/action-gh-release: 📦 GitHub Action for creating GitHub Releases](https://github.com/softprops/action-gh-release)


# TODO

- muslのGitHub Actionsが途中で落ちる謎を解く(いまはglibにしてある)。たぶん3OS分が同時に動くから制限に引っかかるんだと思う。
- tarball(zip)のバージョンがCargo.tomlと一致しないのをなんか考える。
- Windows版のzipのディレクトリが深いのをなおす。
- Linux版はzipよりtarがいいと思うので直す。
- cargo-rpmやcargo-debが、Ubuntu1804LTSのパッケージ版のRustだとコンパイルに失敗するので(バージョンが古い)、あきらめてrustupインストールにする。

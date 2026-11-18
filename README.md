This repository exists to host PoC code for a paper.

The code was written under severe time constraints and is only an absolute bare minimum implementation. It has very few features (almost all existing features are already utilized in `samples/pipeline.yaml`), is extremely slow and likely very buggy.

---

Run example: `cargo run --release -- samples/pipeilne.yaml samples/curl.exe`

Expected output: `List[Ranges[0x140045020-0x140045300,],]` (curl_easy_strerror)

---

The curl.exe in the repository was compiled locally with `x86_64-w64-mingw32-gcc 16.2.0` (provided by Gentoo `dev-util/mingw64-toolchain` version `14.0.0_p1`), statically linked `curl-8.23.0`, `openssl-3.5.9`, `zlib-1.4`. No code from the file is ever executed during the analysis.
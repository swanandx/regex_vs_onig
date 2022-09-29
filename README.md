# regex_vs_onig

Using [hyperfine](https://github.com/sharkdp/hyperfine) for benchmarking after building binaries in release mode.

```bash
hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' './use_regex https://google.com' './use_onig https://google.com'
```
<br/>

![bench](https://user-images.githubusercontent.com/73115739/193082922-5e427906-886e-4545-bac3-5eca0d019b34.png)
